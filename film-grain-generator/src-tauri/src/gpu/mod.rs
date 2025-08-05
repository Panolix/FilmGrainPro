use crate::{FilmStock, GrainParams, Grain};

pub mod wgpu_backend;

#[cfg(target_os = "macos")]
pub mod metal_backend;

#[cfg(all(target_os = "windows", feature = "cuda"))]
pub mod cuda_backend;

#[cfg(all(target_os = "linux", feature = "opencl"))]
pub mod opencl_backend;

#[derive(Debug, Clone)]
pub enum GpuBackend {
    Wgpu,
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(all(target_os = "windows", feature = "cuda"))]
    Cuda,
    #[cfg(all(target_os = "linux", feature = "opencl"))]
    OpenCL,
    Cpu, // Fallback
}

pub trait GpuGrainRenderer: Send + Sync {
    fn render_grains(&self, grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String>;
    fn get_backend_name(&self) -> &'static str;
    fn is_available(&self) -> bool;
}

pub struct GpuManager {
    renderer: Box<dyn GpuGrainRenderer + Send + Sync>,
    backend: GpuBackend,
}

impl GpuManager {
    pub async fn new() -> Result<Self, String> {
        // Try to initialize the best available GPU backend
        let (renderer, backend) = Self::initialize_best_backend().await?;
        
        Ok(Self {
            renderer,
            backend,
        })
    }

    async fn initialize_best_backend() -> Result<(Box<dyn GpuGrainRenderer + Send + Sync>, GpuBackend), String> {
        // Priority order: Metal (Apple Silicon) > CUDA (NVIDIA) > OpenCL (AMD) > WebGPU > CPU
        
        #[cfg(target_os = "macos")]
        {
            if let Ok(metal_renderer) = metal_backend::MetalGrainRenderer::new().await {
                println!("🚀 Initialized Metal GPU acceleration for Apple Silicon");
                return Ok((Box::new(metal_renderer), GpuBackend::Metal));
            }
        }

        #[cfg(all(target_os = "windows", feature = "cuda"))]
        {
            if let Ok(cuda_renderer) = cuda_backend::CudaGrainRenderer::new().await {
                println!("🚀 Initialized CUDA GPU acceleration for NVIDIA");
                return Ok((Box::new(cuda_renderer), GpuBackend::Cuda));
            }
        }

        #[cfg(all(target_os = "linux", feature = "opencl"))]
        {
            if let Ok(opencl_renderer) = opencl_backend::OpenCLGrainRenderer::new().await {
                println!("🚀 Initialized OpenCL GPU acceleration for AMD");
                return Ok((Box::new(opencl_renderer), GpuBackend::OpenCL));
            }
        }

        // Fallback to WebGPU (works on all platforms)
        if let Ok(wgpu_renderer) = wgpu_backend::WgpuGrainRenderer::new().await {
            println!("🚀 Initialized WebGPU acceleration");
            return Ok((Box::new(wgpu_renderer), GpuBackend::Wgpu));
        }

        Err("No GPU backend available, falling back to CPU".to_string())
    }

    pub async fn render_grains(&self, grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
        if !self.renderer.is_available() {
            return Err("GPU renderer is not available".to_string());
        }

        self.renderer.render_grains(grains, params, stock)
    }

    pub fn get_backend_info(&self) -> (GpuBackend, &'static str) {
        (self.backend.clone(), self.renderer.get_backend_name())
    }
}