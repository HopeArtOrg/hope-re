use ndarray::{Array, Array4};
use ort::session::Session;
use std::sync::atomic::Ordering;
use tauri::Emitter;

use super::types::{
    AlgorithmParams, ModelRunFn, ProtectionProgress, ProtectionState, SPSA_DIRECTIONS_PER_ITER,
    TILE_SIZE, TileProgress,
};

struct Xoshiro128 {
    s: [u32; 4],
}

impl Xoshiro128 {
    fn new(seed: u64) -> Self {
        let mut s = [0u32; 4];
        let mut z = seed;
        for slot in &mut s {
            z = z.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(1);
            *slot = (z >> 32) as u32 ^ z as u32;
            if *slot == 0 {
                *slot = 1;
            }
        }
        Self { s }
    }

    fn next_u32(&mut self) -> u32 {
        let result = self.s[0]
            .wrapping_add(self.s[3])
            .rotate_left(7)
            .wrapping_add(self.s[0]);
        let t = self.s[1] << 9;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(11);
        result
    }

    fn next_bool(&mut self) -> bool {
        self.next_u32() & 1 == 0
    }
}

fn generate_spsa_direction(num_elements: usize, rng: &mut Xoshiro128) -> Vec<f32> {
    (0..num_elements)
        .map(|_| if rng.next_bool() { 1.0 } else { -1.0 })
        .collect()
}

fn expand_edge_weights(edge_weights: &[f32], num_elements: usize) -> Vec<f32> {
    let mut edge_flat = vec![1.0f32; num_elements];
    edge_weights.iter().enumerate().for_each(|(i, &ew)| {
        let base = i * 3;
        if base + 2 < num_elements {
            edge_flat[base] = ew;
            edge_flat[base + 1] = ew;
            edge_flat[base + 2] = ew;
        }
    });
    edge_flat
}

const MOMENTUM_BETA: f32 = 0.85;

pub fn spsa_pgd_on_tile(
    session: &mut Session,
    base_tile: &Array4<f32>,
    params: &AlgorithmParams,
    iterations: u32,
    run_model: &mut ModelRunFn,
    edge_weights: &[f32],
    progress: &TileProgress,
    state: &ProtectionState,
) -> Result<Array4<f32>, String> {
    let shape = base_tile.shape();
    let num_elements = shape.iter().product::<usize>();
    let mut perturbation = vec![0.0f32; num_elements];
    let mut momentum = vec![0.0f32; num_elements];

    let epsilon = params.epsilon;
    let alpha = epsilon * params.alpha_multiplier / iterations.max(1) as f32;
    let ck_initial = epsilon * 0.1;
    let perceptual_weight = params.perceptual_weight;

    let edge_flat = expand_edge_weights(edge_weights, num_elements);
    let base_flat: Vec<f32> = base_tile.iter().copied().collect();

    let mut rng = Xoshiro128::new(get_nanos_seed());

    for k in 0..iterations {
        if state.is_cancelled.load(Ordering::SeqCst) {
            return Err("Protection cancelled".to_string());
        }

        let ck = ck_initial / ((k + 1) as f32).powf(0.101);
        let ak = alpha / ((k + 1) as f32).powf(0.602);

        let (grad_accum, valid_count) = (0..SPSA_DIRECTIONS_PER_ITER).try_fold(
            (vec![0.0f32; num_elements], 0u32),
            |(mut acc, count), _| {
                if state.is_cancelled.load(Ordering::SeqCst) {
                    return Err("Protection cancelled".to_string());
                }

                let direction = generate_spsa_direction(num_elements, &mut rng);

                let plus_data: Vec<f32> = base_flat
                    .iter()
                    .zip(perturbation.iter())
                    .zip(direction.iter())
                    .map(|((b, p), d)| (b + p + ck * d).clamp(0.0, 1.0))
                    .collect();

                let minus_data: Vec<f32> = base_flat
                    .iter()
                    .zip(perturbation.iter())
                    .zip(direction.iter())
                    .map(|((b, p), d)| (b + p - ck * d).clamp(0.0, 1.0))
                    .collect();

                let p_loss_diff = if perceptual_weight > 0.0 {
                    calculate_perceptual_diff(
                        &base_flat,
                        &plus_data,
                        &minus_data,
                        &edge_flat,
                        perceptual_weight,
                    )
                } else {
                    0.0
                };

                let tile_shape = (shape[0], shape[1], shape[2], shape[3]);
                let loss_plus = run_model(
                    session,
                    &Array::from_shape_vec(tile_shape, plus_data).map_err(|e| e.to_string())?,
                )?;
                let loss_minus = run_model(
                    session,
                    &Array::from_shape_vec(tile_shape, minus_data).map_err(|e| e.to_string())?,
                )?;

                let diff_over_2ck = (loss_plus - loss_minus + p_loss_diff) / (2.0 * ck);
                acc.iter_mut().zip(direction.iter()).for_each(|(a, d)| {
                    *a += diff_over_2ck * d;
                });

                Ok((acc, count + 1))
            },
        )?;

        if valid_count > 0 {
            update_perturbation(
                &mut perturbation,
                &mut momentum,
                &grad_accum,
                &edge_flat,
                valid_count,
                ak,
                epsilon,
            );
        }

        if k % 5 == 0 {
            emit_tile_progress(progress, k + 1, iterations);
        }
    }

    let result_data = base_flat
        .iter()
        .zip(perturbation.iter())
        .map(|(b, p)| (b + p).clamp(0.0, 1.0))
        .collect();

    Array::from_shape_vec((shape[0], shape[1], shape[2], shape[3]), result_data)
        .map_err(|e| format!("Reshape error: {}", e))
}

fn calculate_perceptual_diff(
    base: &[f32],
    plus: &[f32],
    minus: &[f32],
    edges: &[f32],
    weight: f32,
) -> f32 {
    let n = base.len() as f32;
    let diff_sum: f32 = base
        .iter()
        .zip(plus.iter())
        .zip(minus.iter())
        .zip(edges.iter())
        .map(|(((b, p), m), e)| {
            let dp = p - b;
            let dm = m - b;
            let inv_edge = (1.5 - e).clamp(0.5, 1.5);
            (dp * dp - dm * dm) * inv_edge
        })
        .sum();
    weight * diff_sum * (1.0 / n) * 100.0
}

fn update_perturbation(
    perturbation: &mut [f32],
    momentum: &mut [f32],
    grad_accum: &[f32],
    edges: &[f32],
    count: u32,
    ak: f32,
    epsilon: f32,
) {
    let scale = 1.0 / count as f32;
    perturbation
        .iter_mut()
        .zip(momentum.iter_mut())
        .zip(grad_accum.iter())
        .zip(edges.iter())
        .for_each(|(((p, m), g), e)| {
            let weighted_grad = g * scale * e;
            *m = MOMENTUM_BETA * *m + (1.0 - MOMENTUM_BETA) * weighted_grad;
            let sign = if *m > 0.0 {
                1.0
            } else if *m < 0.0 {
                -1.0
            } else {
                0.0
            };
            *p = (*p - ak * sign).clamp(-epsilon, epsilon);
        });
}

fn emit_tile_progress(progress: &TileProgress, k: u32, iterations: u32) {
    let tile_frac = if progress.tile_total > 0 {
        (progress.tile_current - 1) as f64 / progress.tile_total as f64
    } else {
        0.0
    };
    let iter_frac = k as f64 / iterations as f64;
    let per_tile = 1.0 / progress.tile_total.max(1) as f64;
    let percent = ((tile_frac + iter_frac * per_tile) * 95.0).min(95.0);

    let _ = progress.app.emit(
        "protection-progress",
        ProtectionProgress {
            stage: "processing".to_string(),
            tile_current: progress.tile_current,
            tile_total: progress.tile_total,
            iteration_current: k,
            iteration_total: iterations,
            percent,
        },
    );
}

fn get_nanos_seed() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}
