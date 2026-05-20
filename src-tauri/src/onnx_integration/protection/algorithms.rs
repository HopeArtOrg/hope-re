use ndarray::Array4;
use ort::session::Session;
use ort::value::Tensor;

use super::types::AlgorithmParams;
pub fn get_noise_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.24 / 0.5,
        max_iterations: 500,
        alpha_multiplier: 5.0,
        perceptual_weight: 0.15,
    }
}

pub fn get_glaze_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.18 / 0.5,
        max_iterations: 600,
        alpha_multiplier: 4.5,
        perceptual_weight: 0.3,
    }
}

pub fn get_nightshade_params(intensity: f32) -> AlgorithmParams {
    AlgorithmParams {
        epsilon: intensity * 0.16 / 0.5,
        max_iterations: 750,
        alpha_multiplier: 5.0,
        perceptual_weight: 0.4,
    }
}

pub fn get_glaze_style_index(style: &str) -> i64 {
    match style {
        "abstract" => 0,
        "impressionist" => 1,
        "cubist" => 2,
        "sketch" => 3,
        "watercolor" => 4,
        _ => 0,
    }
}

pub fn get_nightshade_target_index(target: &str) -> i64 {
    match target {
        "dog" => 0,
        "cat" => 1,
        "car" => 2,
        "landscape" => 3,
        "person" => 4,
        "building" => 5,
        "food" => 6,
        "abstract" => 7,
        _ => 0,
    }
}

pub fn run_noise_model(session: &mut Session, input: &Array4<f32>) -> Result<f32, String> {
    let shape = input.shape();
    let data: Box<[f32]> = input
        .iter()
        .copied()
        .collect::<Vec<f32>>()
        .into_boxed_slice();
    let input_tensor = Tensor::from_array(([shape[0], shape[1], shape[2], shape[3]], data))
        .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor])
        .map_err(|e| format!("Failed to run noise model: {}", e))?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract noise model output: {}", e))
}

pub fn run_glaze_model(
    session: &mut Session,
    input: &Array4<f32>,
    style_index: i64,
) -> Result<f32, String> {
    let shape = input.shape();
    let input_data: Box<[f32]> = input
        .iter()
        .copied()
        .collect::<Vec<f32>>()
        .into_boxed_slice();
    let input_tensor = Tensor::from_array(([shape[0], shape[1], shape[2], shape[3]], input_data))
        .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    let style_data: Box<[i32]> = Box::new([style_index as i32]);
    let style_tensor = Tensor::from_array(([1_usize], style_data))
        .map_err(|e| format!("Failed to create style tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor, style_tensor])
        .map_err(|e| {
            log::error!("Glaze model error: {}", e);
            format!("Failed to run glaze model: {}", e)
        })?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract glaze model output: {}", e))
}

pub fn run_nightshade_model(
    session: &mut Session,
    input: &Array4<f32>,
    target_index: i64,
) -> Result<f32, String> {
    let shape = input.shape();
    let input_data: Box<[f32]> = input
        .iter()
        .copied()
        .collect::<Vec<f32>>()
        .into_boxed_slice();
    let input_tensor = Tensor::from_array(([shape[0], shape[1], shape[2], shape[3]], input_data))
        .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    let target_data: Box<[i32]> = Box::new([target_index as i32]);
    let target_tensor = Tensor::from_array(([1_usize], target_data))
        .map_err(|e| format!("Failed to create target tensor: {}", e))?;

    let outputs = session
        .run(ort::inputs![input_tensor, target_tensor])
        .map_err(|e| {
            log::error!("Nightshade model error: {}", e);
            format!("Failed to run nightshade model: {}", e)
        })?;

    outputs[0]
        .try_extract_scalar::<f32>()
        .map_err(|e| format!("Failed to extract nightshade model output: {}", e))
}
