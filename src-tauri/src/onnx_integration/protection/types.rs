use std::sync::atomic::AtomicBool;
use ndarray::Array4;
use ort::session::Session;

pub const TILE_SIZE: u32 = 224;
pub const TILE_OVERLAP: u32 = 32;
pub const SPSA_DIRECTIONS_PER_ITER: usize = 4;

pub type ModelRunFn = dyn FnMut(&mut Session, &Array4<f32>) -> Result<f32, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionSettings {
    pub algorithm: String,
    pub intensity: f32,
    pub output_quality: u8,
    pub render_quality: u8,
    pub glaze_style: Option<String>,
    pub nightshade_target: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProtectionResult {
    pub image_base64: String,
    pub success: bool,
    pub message: String,
    pub model_used: bool,
}

pub struct TileProgress {
    pub app: tauri::AppHandle,
    pub tile_current: u32,
    pub tile_total: u32,
}

#[derive(Debug, Clone)]
pub struct AlgorithmParams {
    pub epsilon: f32,
    pub max_iterations: u32,
    pub alpha_multiplier: f32,
    pub perceptual_weight: f32,
}

impl AlgorithmParams {
    pub fn validate(&self) -> Result<(), String> {
        if !self.epsilon.is_finite() {
            return Err(format!(
                "epsilon must be a finite number, got {}",
                self.epsilon
            ));
        }

        if self.epsilon <= 0.0 {
            log::warn!(
                "epsilon is very small ({}) which may cause poor protection. Consider increasing intensity.",
                self.epsilon
            );
        }

        if !self.alpha_multiplier.is_finite() || self.alpha_multiplier <= 0.0 {
            return Err(format!(
                "alpha_multiplier must be a positive finite number, got {}",
                self.alpha_multiplier
            ));
        }

        if !self.perceptual_weight.is_finite() {
            return Err(format!(
                "perceptual_weight must be a finite number, got {}",
                self.perceptual_weight
            ));
        }

        if self.perceptual_weight < 0.0 || self.perceptual_weight > 1.0 {
            return Err(format!(
                "perceptual_weight must be in range [0.0, 1.0], got {}",
                self.perceptual_weight
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProtectionProgress {
    pub stage: String,
    pub tile_current: u32,
    pub tile_total: u32,
    pub iteration_current: u32,
    pub iteration_total: u32,
    pub percent: f64,
}

pub struct ProtectionState {
    pub is_cancelled: AtomicBool,
}

impl Default for ProtectionState {
    fn default() -> Self {
        Self {
            is_cancelled: AtomicBool::new(false),
        }
    }
}
