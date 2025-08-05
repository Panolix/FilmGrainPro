#include <metal_stdlib>
using namespace metal;

struct MetalGrain {
    float2 position;
    float size;
    float opacity;
    float shape_factor;
    float3 padding;
};

struct MetalParams {
    uint width;
    uint height;
    uint grain_count;
    uint padding;
    float4 grain_color;
};

float random(float2 seed) {
    return fract(sin(dot(seed, float2(12.9898, 78.233))) * 43758.5453);
}

float4 render_grain_at_pixel(float2 pixel_pos, MetalGrain grain, MetalParams params) {
    float2 grain_center = grain.position;
    float2 distance_vec = pixel_pos - grain_center;
    
    // Apply shape factor for realistic grain shapes
    float2 adjusted_distance = float2(
        distance_vec.x / grain.shape_factor,
        distance_vec.y
    );
    
    float distance = length(adjusted_distance);
    
    if (distance > grain.size) {
        return float4(0.0, 0.0, 0.0, 0.0);
    }
    
    // Calculate edge falloff for realistic grain edges
    float edge_alpha = 1.0;
    if (distance > grain.size * 0.7) {
        edge_alpha = (grain.size - distance) / (grain.size * 0.3);
        edge_alpha = max(edge_alpha, 0.0);
    }
    
    // Apply opacity with some randomness for natural variation
    float noise = random(grain_center + pixel_pos * 0.1) * 0.1 - 0.05;
    float final_opacity = grain.opacity * edge_alpha * (1.0 + noise);
    
    return float4(
        params.grain_color.rgb,
        clamp(final_opacity, 0.0, 1.0)
    );
}

kernel void grain_compute_kernel(
    constant MetalGrain* grains [[buffer(0)]],
    constant MetalParams& params [[buffer(1)]],
    texture2d<float, access::write> output_texture [[texture(0)]],
    uint2 gid [[thread_position_in_grid]]
) {
    // Check bounds
    if (gid.x >= params.width || gid.y >= params.height) {
        return;
    }
    
    float2 pixel_pos = float2(gid.x, gid.y);
    float4 final_color = float4(0.0, 0.0, 0.0, 0.0);
    
    // Render all grains that affect this pixel
    for (uint i = 0; i < params.grain_count; i++) {
        MetalGrain grain = grains[i];
        float4 grain_contribution = render_grain_at_pixel(pixel_pos, grain, params);
        
        // Alpha blend the grain contribution
        if (grain_contribution.a > 0.0) {
            float alpha = grain_contribution.a;
            final_color = float4(
                final_color.rgb * (1.0 - alpha) + grain_contribution.rgb * alpha,
                max(final_color.a, alpha)
            );
        }
    }
    
    output_texture.write(final_color, gid);
}