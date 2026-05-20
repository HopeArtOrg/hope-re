use image::DynamicImage;
use ort::session::Session;
use std::sync::atomic::Ordering;
use tauri::Emitter;

use super::preprocessing::{compute_edge_weight_map, preprocess_tile};
use super::spsa::spsa_pgd_on_tile;
use super::types::{
    AlgorithmParams, ModelRunFn, ProtectionProgress, ProtectionState, TileProgress, TILE_OVERLAP,
    TILE_SIZE,
};

#[derive(Debug, Clone, Copy)]
struct TileRegion {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

pub fn apply_model_protection(
    img: &DynamicImage,
    session: &mut Session,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    app: &tauri::AppHandle,
    state: &ProtectionState,
) -> Result<DynamicImage, String> {
    params.validate()?;

    let (width, height) = (img.width(), img.height());
    let stride = TILE_SIZE - TILE_OVERLAP;

    let mut result_accum = vec![0.0f32; (width * height * 3) as usize];
    let mut weight_accum = vec![0.0f32; (width * height) as usize];

    let regions = calculate_tile_regions(width, height, stride);
    let total_tiles = regions.len() as u32;

    for (i, region) in regions.into_iter().enumerate() {
        if state.is_cancelled.load(Ordering::SeqCst) {
            return Err("Protection cancelled".to_string());
        }

        let base_tile = preprocess_tile(img, region.x, region.y, region.w, region.h);
        let edge_weights = compute_edge_weight_map(&base_tile);

        let protected_tile = spsa_pgd_on_tile(
            session,
            &base_tile,
            params,
            iterations,
            run_model,
            &edge_weights,
            &TileProgress {
                app: app.clone(),
                tile_current: (i + 1) as u32,
                tile_total: total_tiles,
            },
            state,
        )?;

        blend_tile_direct(
            &protected_tile,
            &region,
            width,
            &mut result_accum,
            &mut weight_accum,
        );

        emit_protection_progress(app, (i + 1) as u32, total_tiles, iterations);
    }

    construct_result_image(width, height, img, &result_accum, &weight_accum)
}

fn calculate_tile_regions(width: u32, height: u32, stride: u32) -> Vec<TileRegion> {
    let tiles_x = if width <= TILE_SIZE {
        1
    } else {
        ((width - TILE_OVERLAP) as f32 / stride as f32).ceil() as u32
    };
    let tiles_y = if height <= TILE_SIZE {
        1
    } else {
        ((height - TILE_OVERLAP) as f32 / stride as f32).ceil() as u32
    };

    (0..tiles_y)
        .flat_map(|ty| {
            (0..tiles_x).map(move |tx| {
                let x = (tx * stride).min(width.saturating_sub(TILE_SIZE));
                let y = (ty * stride).min(height.saturating_sub(TILE_SIZE));
                TileRegion {
                    x,
                    y,
                    w: TILE_SIZE.min(width - x),
                    h: TILE_SIZE.min(height - y),
                }
            })
        })
        .collect()
}

fn blend_tile_direct(
    protected_tile: &ndarray::Array4<f32>,
    region: &TileRegion,
    width: u32,
    result_accum: &mut [f32],
    weight_accum: &mut [f32],
) {
    let ts = TILE_SIZE as usize;
    let no_scale = region.w == TILE_SIZE && region.h == TILE_SIZE;

    for py in 0..region.h {
        for px in 0..region.w {
            let (dst_x, dst_y) = (region.x + px, region.y + py);
            let pixel_idx = (dst_y as usize * width as usize) + dst_x as usize;
            let dst_idx = pixel_idx * 3;

            let edge_x = px.min(region.w - 1 - px).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
            let edge_y = py.min(region.h - 1 - py).min(TILE_OVERLAP) as f32 / TILE_OVERLAP as f32;
            let weight = edge_x.min(edge_y).max(0.01);

            if dst_idx + 2 >= result_accum.len() {
                continue;
            }

            let color = if no_scale {
                let (sx, sy) = (px as usize, py as usize);
                [
                    protected_tile[[0, sy, sx, 0]],
                    protected_tile[[0, sy, sx, 1]],
                    protected_tile[[0, sy, sx, 2]],
                ]
            } else {
                sample_bilinear(protected_tile, px, py, region.w, region.h, ts)
            };

            for c in 0..3 {
                result_accum[dst_idx + c] += color[c] * 255.0 * weight;
            }

            if pixel_idx < weight_accum.len() {
                weight_accum[pixel_idx] += weight;
            }
        }
    }
}

fn sample_bilinear(
    tile: &ndarray::Array4<f32>,
    px: u32,
    py: u32,
    rw: u32,
    rh: u32,
    ts: usize,
) -> [f32; 3] {
    let scale_x = TILE_SIZE as f32 / rw as f32;
    let scale_y = TILE_SIZE as f32 / rh as f32;
    let (src_fx, src_fy) = (px as f32 * scale_x, py as f32 * scale_y);

    let x0 = (src_fx as usize).min(ts - 1);
    let y0 = (src_fy as usize).min(ts - 1);
    let x1 = (x0 + 1).min(ts - 1);
    let y1 = (y0 + 1).min(ts - 1);
    let (fx, fy) = (src_fx - x0 as f32, src_fy - y0 as f32);

    let mut res = [0.0f32; 3];
    for c in 0..3 {
        let v00 = tile[[0, y0, x0, c]];
        let v10 = tile[[0, y0, x1, c]];
        let v01 = tile[[0, y1, x0, c]];
        let v11 = tile[[0, y1, x1, c]];
        res[c] = v00 * (1.0 - fx) * (1.0 - fy)
            + v10 * fx * (1.0 - fy)
            + v01 * (1.0 - fx) * fy
            + v11 * fx * fy;
    }
    res
}

fn construct_result_image(
    width: u32,
    height: u32,
    original: &DynamicImage,
    result_accum: &[f32],
    weight_accum: &[f32],
) -> Result<DynamicImage, String> {
    let mut result_pixels = vec![0u8; (width * height * 4) as usize];
    let original_rgba = original.to_rgba8();

    for y in 0..height {
        for x in 0..width {
            let pixel_idx = (y as usize * width as usize) + x as usize;
            let (rgb_idx, rgba_idx) = (pixel_idx * 3, pixel_idx * 4);
            let orig = original_rgba.get_pixel(x, y);

            if rgba_idx + 3 < result_pixels.len() {
                if pixel_idx < weight_accum.len() && weight_accum[pixel_idx] > 0.0 {
                    let w = weight_accum[pixel_idx];
                    result_pixels[rgba_idx] = (result_accum[rgb_idx] / w).clamp(0.0, 255.0) as u8;
                    result_pixels[rgba_idx + 1] =
                        (result_accum[rgb_idx + 1] / w).clamp(0.0, 255.0) as u8;
                    result_pixels[rgba_idx + 2] =
                        (result_accum[rgb_idx + 2] / w).clamp(0.0, 255.0) as u8;
                } else {
                    result_pixels[rgba_idx] = orig[0];
                    result_pixels[rgba_idx + 1] = orig[1];
                    result_pixels[rgba_idx + 2] = orig[2];
                }
                result_pixels[rgba_idx + 3] = orig[3];
            }
        }
    }

    image::RgbaImage::from_raw(width, height, result_pixels)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| "Failed to construct result image".to_string())
}

fn emit_protection_progress(app: &tauri::AppHandle, current: u32, total: u32, iterations: u32) {
    let percent = (current as f64 / total as f64 * 95.0).min(95.0);
    let _ = app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: "processing".to_string(),
            tile_current: current,
            tile_total: total,
            iteration_current: iterations,
            iteration_total: iterations,
            percent,
        },
    );
}
