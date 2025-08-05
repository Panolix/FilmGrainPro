struct GpuGrain {
    position: vec2<f32>,
    size: f32,
    opacity: f32,
    shape_factor: f32,
    padding: vec3<f32>,
}

struct GpuParams {
    width: u32,
    height: u32,
    grain_count: u32,
    padding: u32,
    grain_color: vec4<f32>,
}

@group(0) @binding(0) var<storage, read> grains: array<GpuGrain>;
@group(0) @binding(1) var<uniform> params: GpuParams;
@group(0) @binding(2) var output_texture: texture_storage_2d<rgba8unorm, write>;

fn random(seed: vec2<f32>) -> f32 {
    return fract(sin(dot(seed, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn render_grain_at_pixel(pixel_pos: vec2<f32>, grain: GpuGrain) -> vec4<f32> {
    let grain_center = grain.position;
    let distance_vec = pixel_pos - grain_center;
    
    // Apply shape factor for realistic grain shapes
    let adjusted_distance = vec2<f32>(
        distance_vec.x / grain.shape_factor,
        distance_vec.y
    );
    
    let distance = length(adjusted_distance);
    
    if distance > grain.size {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Calculate edge falloff for realistic grain edges
    var edge_alpha = 1.0;
    if distance > grain.size * 0.7 {
        edge_alpha = (grain.size - distance) / (grain.size * 0.3);
        edge_alpha = max(edge_alpha, 0.0);
    }
    
    // Apply opacity with some randomness for natural variation
    let noise = random(grain_center + pixel_pos * 0.1) * 0.1 - 0.05;
    let final_opacity = grain.opacity * edge_alpha * (1.0 + noise);
    
    return vec4<f32>(
        params.grain_color.rgb,
        clamp(final_opacity, 0.0, 1.0)
    );
}

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let pixel_coord = vec2<i32>(i32(global_id.x), i32(global_id.y));
    let pixel_pos = vec2<f32>(f32(global_id.x), f32(global_id.y));
    
    // Check bounds
    if global_id.x >= params.width || global_id.y >= params.height {
        return;
    }
    
    var final_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    // Render all grains that affect this pixel
    for (var i = 0u; i < params.grain_count; i++) {
        let grain = grains[i];
        let grain_contribution = render_grain_at_pixel(pixel_pos, grain);
        
        // Alpha blend the grain contribution
        if grain_contribution.a > 0.0 {
            let alpha = grain_contribution.a;
            final_color = vec4<f32>(
                final_color.rgb * (1.0 - alpha) + grain_contribution.rgb * alpha,
                max(final_color.a, alpha)
            );
        }
    }
    
    textureStore(output_texture, pixel_coord, final_color);
}