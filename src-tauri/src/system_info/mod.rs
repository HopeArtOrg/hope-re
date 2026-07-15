pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod platform;
pub mod storage;

pub use cpu::get_cpu_info;
pub use gpu::get_gpu_info;
#[cfg(not(all(target_os = "android", not(target_arch = "aarch64"))))]
pub use gpu::has_nvidia_gpu;
pub use memory::get_memory_info;
pub use platform::{get_platform_info, PlatformInfo};
pub use storage::get_storage_info;
