#[cfg(target_os = "macos")]
use super::GpuGrainRenderer;
use crate::{FilmStock, GrainParams, Grain};
use metal::*;
use objc::rc::autoreleasepool;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct MetalGrain {
    position: [f32; 2],
    size: f32,
    opacity: f32,
    shape_factor: f32,
    _padding: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct MetalParams {
    width: u32,
    height: u32,
    grain_count: u32,
    _padding: u32,
    grain_color: [f32; 4],
}

pub struct MetalGrainRenderer {
    device: Device,
    command_queue: CommandQueue,
    pipeline_state: ComputePipelineState,
}

impl MetalGrainRenderer {
    pub async fn new() -> Result<Self, String> {
        autoreleasepool(|| {
            // Get the default Metal device (Apple Silicon GPU)
            let device = Device::system_default()
                .ok_or("No Metal device found (Apple Silicon required)")?;

            println!("🍎 Metal Device: {}", device.name());

            // Create command queue
            let command_queue = device.new_command_queue();

            // Create Metal shader library
            let shader_source = include_str!("shaders/grain_compute.metal");
            let library = device
                .new_library_with_source(shader_source, &CompileOptions::new())
                .map_err(|e| format!("Failed to compile Metal shader: {}", e))?;

            // Get the compute function
            let kernel_function = library
                .get_function("grain_compute_kernel", None)
                .map_err(|e| format!("Failed to get kernel function: {}", e))?;

            // Create compute pipeline state
            let pipeline_state = device
                .new_compute_pipeline_state_with_function(&kernel_function)
                .map_err(|e| format!("Failed to create pipeline state: {}", e))?;

            Ok(Self {
                device,
                command_queue,
                pipeline_state,
            })
        })
    }

    fn convert_grains_to_metal_format(&self, grains: &[Grain]) -> Vec<MetalGrain> {
        grains.iter().map(|grain| MetalGrain {
            position: [grain.x, grain.y],
            size: grain.size,
            opacity: grain.opacity,
            shape_factor: grain.shape_factor,
            _padding: [0.0; 3],
        }).collect()
    }

    fn get_grain_color(&self, stock: &FilmStock) -> [f32; 4] {
        if let Some(rgb_range) = stock.color_properties.rgb_ranges.first() {
            [
                rgb_range.r[0] as f32 / 255.0,
                rgb_range.g[0] as f32 / 255.0,
                rgb_range.b[0] as f32 / 255.0,
                1.0,
            ]
        } else {
            [0.8, 0.8, 0.8, 1.0]
        }
    }
}

impl GpuGrainRenderer for MetalGrainRenderer {
    fn render_grains(&self, grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
        autoreleasepool(|| {
            let metal_grains = self.convert_grains_to_metal_format(grains);
            
            // Create buffers
            let grain_buffer = self.device.new_buffer_with_data(
                metal_grains.as_ptr() as *const _,
                (metal_grains.len() * mem::size_of::<MetalGrain>()) as u64,
                MTLResourceOptions::StorageModeShared,
            );

            let metal_params = MetalParams {
                width: params.width,
                height: params.height,
                grain_count: grains.len() as u32,
                _padding: 0,
                grain_color: self.get_grain_color(stock),
            };

            let params_buffer = self.device.new_buffer_with_data(
                &metal_params as *const _ as *const _,
                mem::size_of::<MetalParams>() as u64,
                MTLResourceOptions::StorageModeShared,
            );

            // Create output texture
            let texture_descriptor = TextureDescriptor::new();
            texture_descriptor.set_texture_type(MTLTextureType::D2);
            texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
            texture_descriptor.set_width(params.width as u64);
            texture_descriptor.set_height(params.height as u64);
            texture_descriptor.set_usage(MTLTextureUsage::ShaderWrite | MTLTextureUsage::ShaderRead);

            let output_texture = self.device.new_texture(&texture_descriptor);

            // Create output buffer for reading back data
            let output_buffer_size = (params.width * params.height * 4) as u64;
            let output_buffer = self.device.new_buffer(
                output_buffer_size,
                MTLResourceOptions::StorageModeShared,
            );

            // Create command buffer and encoder
            let command_buffer = self.command_queue.new_command_buffer();
            let compute_encoder = command_buffer.new_compute_command_encoder();

            // Set pipeline state and buffers
            compute_encoder.set_compute_pipeline_state(&self.pipeline_state);
            compute_encoder.set_buffer(0, Some(&grain_buffer), 0);
            compute_encoder.set_buffer(1, Some(&params_buffer), 0);
            compute_encoder.set_texture(0, Some(&output_texture));

            // Calculate thread group sizes for optimal Apple Silicon performance
            let thread_group_size = MTLSize::new(8, 8, 1);
            let thread_groups = MTLSize::new(
                ((params.width + 7) / 8) as u64,
                ((params.height + 7) / 8) as u64,
                1,
            );

            compute_encoder.dispatch_thread_groups(thread_groups, thread_group_size);
            compute_encoder.end_encoding();

            // Copy texture to buffer for readback
            let blit_encoder = command_buffer.new_blit_command_encoder();
            blit_encoder.copy_from_texture_to_buffer(
                &output_texture,
                0, // source slice
                0, // source level
                MTLOrigin { x: 0, y: 0, z: 0 },
                MTLSize::new(params.width as u64, params.height as u64, 1),
                &output_buffer,
                0, // destination offset
                (params.width * 4) as u64, // bytes per row
                (params.width * params.height * 4) as u64, // bytes per image
                MTLBlitOption::None,
            );
            blit_encoder.end_encoding();

            // Commit and wait for completion
            command_buffer.commit();
            command_buffer.wait_until_completed();

            // Read back the data
            let data_ptr = output_buffer.contents() as *const u8;
            let data_slice = unsafe {
                std::slice::from_raw_parts(data_ptr, output_buffer_size as usize)
            };

            Ok(data_slice.to_vec())
        })
    }

    fn get_backend_name(&self) -> &'static str {
        "Metal (Apple Silicon)"
    }

    fn is_available(&self) -> bool {
        Device::system_default().is_some()
    }
}