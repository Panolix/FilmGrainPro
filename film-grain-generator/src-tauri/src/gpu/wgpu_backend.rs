use super::GpuGrainRenderer;
use crate::{FilmStock, GrainParams, Grain};
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct GpuGrain {
    position: [f32; 2],
    size: f32,
    opacity: f32,
    shape_factor: f32,
    _padding: [f32; 3], // Align to 32 bytes
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct GpuParams {
    width: u32,
    height: u32,
    grain_count: u32,
    _padding: u32,
    grain_color: [f32; 4], // RGBA
}

pub struct WgpuGrainRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl WgpuGrainRenderer {
    pub async fn new() -> Result<Self, String> {
        // Initialize WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Failed to find suitable GPU adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Film Grain GPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create GPU device: {}", e))?;

        // Create compute shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Film Grain Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/grain_compute.wgsl").into()),
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Grain Compute Bind Group Layout"),
            entries: &[
                // Grain data buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Parameters buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Output texture
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        // Create compute pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Grain Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Grain Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

        Ok(Self {
            device,
            queue,
            compute_pipeline,
            bind_group_layout,
        })
    }

    fn convert_grains_to_gpu_format(&self, grains: &[Grain]) -> Vec<GpuGrain> {
        grains.iter().map(|grain| GpuGrain {
            position: [grain.x, grain.y],
            size: grain.size,
            opacity: grain.opacity,
            shape_factor: grain.shape_factor,
            _padding: [0.0; 3],
        }).collect()
    }

    fn get_grain_color(&self, stock: &FilmStock) -> [f32; 4] {
        // Extract color from film stock or use default
        if let Some(rgb_range) = stock.color_properties.rgb_ranges.first() {
            [
                rgb_range.r[0] as f32 / 255.0,
                rgb_range.g[0] as f32 / 255.0,
                rgb_range.b[0] as f32 / 255.0,
                1.0,
            ]
        } else {
            [0.8, 0.8, 0.8, 1.0] // Default grain color
        }
    }
}

impl GpuGrainRenderer for WgpuGrainRenderer {
    fn render_grains(&self, grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
        let gpu_grains = self.convert_grains_to_gpu_format(grains);
        
        // Create buffers
        let grain_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grain Buffer"),
            contents: bytemuck::cast_slice(&gpu_grains),
            usage: wgpu::BufferUsages::STORAGE,
        });

        let gpu_params = GpuParams {
            width: params.width,
            height: params.height,
            grain_count: grains.len() as u32,
            _padding: 0,
            grain_color: self.get_grain_color(stock),
        };

        let params_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params Buffer"),
            contents: bytemuck::cast_slice(&[gpu_params]),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        // Create output texture
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Output Texture"),
            size: wgpu::Extent3d {
                width: params.width,
                height: params.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create bind group
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Grain Compute Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: grain_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
            ],
        });

        // Create output buffer for reading back data
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (params.width * params.height * 4) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Dispatch compute shader
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Grain Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Grain Compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            
            // Dispatch with 8x8 workgroups
            let workgroup_size = 8;
            let dispatch_x = (params.width + workgroup_size - 1) / workgroup_size;
            let dispatch_y = (params.height + workgroup_size - 1) / workgroup_size;
            compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
        }

        // Copy texture to buffer
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(params.width * 4),
                    rows_per_image: Some(params.height),
                },
            },
            wgpu::Extent3d {
                width: params.width,
                height: params.height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));

        // Read back the data
        let buffer_slice = output_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            sender.send(result).unwrap();
        });

        self.device.poll(wgpu::Maintain::Wait);
        
        receiver.recv().unwrap()
            .map_err(|e| format!("Failed to map buffer: {:?}", e))?;

        let data = buffer_slice.get_mapped_range();
        let result = data.to_vec();
        drop(data);
        output_buffer.unmap();

        Ok(result)
    }

    fn get_backend_name(&self) -> &'static str {
        "WebGPU"
    }

    fn is_available(&self) -> bool {
        true
    }
}