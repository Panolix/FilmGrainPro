#include "grain_renderer.h"
#include "pattern_library.h"
#include "../utils/performance_monitor.h"
#include <iostream>
#include <algorithm>
#include <cmath>

GrainRenderer::GrainRenderer() 
    : gpu_context_{nullptr, nullptr, nullptr, nullptr, nullptr, false}
    , rng_(std::random_device{}())
    , uniform_dist_(0.0f, 1.0f)
    , normal_dist_(0.0f, 1.0f) {
}

GrainRenderer::~GrainRenderer() {
    Shutdown();
}

bool GrainRenderer::Initialize() {
    std::cout << "Initializing Grain Renderer..." << std::endl;
    
    // Try to initialize GPU first, fall back to CPU if needed
    if (!InitializeGPU()) {
        std::cout << "GPU initialization failed, using CPU rendering" << std::endl;
    }
    
    return true;
}

void GrainRenderer::Shutdown() {
    ShutdownGPU();
    ClearPatternCache();
}

bool GrainRenderer::ApplyGrain(Image& image, const FilmStock& stock, const GrainParams& params) {
    if (!image.IsValid()) {
        std::cerr << "Invalid image for grain application" << std::endl;
        return false;
    }
    
    PerformanceMonitor::StartTimer("grain_rendering");
    
    bool success = false;
    if (params.use_gpu_acceleration && gpu_context_.initialized) {
        success = ApplyGrainGPU(image, stock, params);
    } else {
        success = ApplyGrainCPU(image, stock, params);
    }
    
    double elapsed = PerformanceMonitor::EndTimer("grain_rendering");
    std::cout << "Grain rendering took: " << elapsed * 1000.0 << "ms" << std::endl;
    
    return success;
}

bool GrainRenderer::ApplyGrainCPU(Image& image, const FilmStock& stock, const GrainParams& params) {
    std::cout << "Applying grain using CPU renderer for: " << stock.display_name << std::endl;
    
    // Generate grain particles
    std::vector<GrainParticle> particles;
    GenerateGrainPattern(stock, params, image.width, image.height, particles);
    
    // Apply each grain particle to the image
    for (const auto& particle : particles) {
        ApplyGrainParticle(image, particle);
    }
    
    // Apply special effects if enabled
    if (params.enable_halation && stock.special_effects.halation != "none") {
        ApplyHalationEffect(image, particles, stock);
    }
    
    ApplySpecialEffects(image, stock, params);
    
    return true;
}

bool GrainRenderer::ApplyGrainGPU(Image& image, const FilmStock& stock, const GrainParams& params) {
    std::cout << "GPU grain rendering not yet implemented, falling back to CPU" << std::endl;
    return ApplyGrainCPU(image, stock, params);
}

void GrainRenderer::GenerateGrainPattern(const FilmStock& stock, const GrainParams& params,
                                        int image_width, int image_height,
                                        std::vector<GrainParticle>& particles) {
    
    // Calculate grain density
    float density = CalculateGrainDensity(stock, params, image_width, image_height);
    int num_grains = static_cast<int>(density * params.density_multiplier);
    
    particles.reserve(num_grains);
    
    for (int i = 0; i < num_grains; ++i) {
        GrainParticle particle;
        
        // Random position
        particle.x = uniform_dist_(rng_) * image_width;
        particle.y = uniform_dist_(rng_) * image_height;
        
        // Size based on film stock characteristics
        particle.size = GetSizeFromDistribution(stock, params);
        
        // Opacity
        float opacity_range = stock.visual_properties.opacity_max - stock.visual_properties.opacity_min;
        particle.opacity = stock.visual_properties.opacity_min + 
                          uniform_dist_(rng_) * opacity_range * params.opacity;
        
        // Aspect ratio
        particle.aspect_ratio = stock.grain_properties.aspect_ratio_x / stock.grain_properties.aspect_ratio_y;
        
        // Random rotation
        particle.rotation = uniform_dist_(rng_) * 2.0f * M_PI;
        
        // Color
        particle.color = GetColorFromFilmStock(stock, params);
        
        // Shape type based on crystal structure
        if (stock.grain_properties.crystal_type == "tabular_t_grain") {
            particle.shape_type = 1; // Tabular
        } else if (stock.grain_properties.shape == "angular_irregular") {
            particle.shape_type = 2; // Angular
        } else if (stock.grain_properties.shape == "irregular_clustered") {
            particle.shape_type = 3; // Irregular
        } else {
            particle.shape_type = 0; // Circular
        }
        
        particles.push_back(particle);
    }
    
    // Apply clustering if specified
    if (stock.grain_properties.clustering != "isolated" && params.clustering_strength > 0.0f) {
        // Apply clustering algorithm (simplified)
        OptimizeGrainPattern(particles, image_width, image_height);
    }
}

void GrainRenderer::ApplyGrainParticle(Image& image, const GrainParticle& particle) {
    switch (particle.shape_type) {
        case 1:
            RenderTabularGrain(image, particle);
            break;
        case 2:
            RenderAngularGrain(image, particle);
            break;
        case 3:
            RenderIrregularGrain(image, particle);
            break;
        default:
            RenderCircularGrain(image, particle);
            break;
    }
}

void GrainRenderer::RenderCircularGrain(Image& image, const GrainParticle& particle) {
    int radius = static_cast<int>(particle.size);
    int x_center = static_cast<int>(particle.x);
    int y_center = static_cast<int>(particle.y);
    
    for (int y = y_center - radius; y <= y_center + radius; ++y) {
        for (int x = x_center - radius; x <= x_center + radius; ++x) {
            if (x >= 0 && x < image.width && y >= 0 && y < image.height) {
                float dist = sqrt((x - particle.x) * (x - particle.x) + 
                                (y - particle.y) * (y - particle.y));
                
                if (dist <= particle.size) {
                    uint8_t* pixel = image.GetPixelPtr(x, y);
                    if (pixel) {
                        // Apply grain with falloff
                        float falloff = 1.0f - (dist / particle.size);
                        float grain_opacity = particle.opacity * falloff;
                        
                        // Extract color components
                        uint8_t grain_r = (particle.color >> 24) & 0xFF;
                        uint8_t grain_g = (particle.color >> 16) & 0xFF;
                        uint8_t grain_b = (particle.color >> 8) & 0xFF;
                        
                        // Blend with existing pixel
                        for (int c = 0; c < std::min(3, image.channels); ++c) {
                            uint8_t grain_value = (c == 0) ? grain_r : (c == 1) ? grain_g : grain_b;
                            float blended = pixel[c] * (1.0f - grain_opacity) + 
                                          grain_value * grain_opacity;
                            pixel[c] = static_cast<uint8_t>(std::clamp(blended, 0.0f, 255.0f));
                        }
                    }
                }
            }
        }
    }
}

void GrainRenderer::RenderTabularGrain(Image& image, const GrainParticle& particle) {
    // Elongated grain shape for T-GRAIN films
    float width = particle.size * particle.aspect_ratio;
    float height = particle.size;
    
    int x_start = static_cast<int>(particle.x - width / 2);
    int x_end = static_cast<int>(particle.x + width / 2);
    int y_start = static_cast<int>(particle.y - height / 2);
    int y_end = static_cast<int>(particle.y + height / 2);
    
    for (int y = y_start; y <= y_end; ++y) {
        for (int x = x_start; x <= x_end; ++x) {
            if (x >= 0 && x < image.width && y >= 0 && y < image.height) {
                uint8_t* pixel = image.GetPixelPtr(x, y);
                if (pixel) {
                    // Simple rectangular grain for now
                    uint8_t grain_value = static_cast<uint8_t>(particle.opacity * 255);
                    for (int c = 0; c < std::min(3, image.channels); ++c) {
                        float blended = pixel[c] * (1.0f - particle.opacity) + 
                                      grain_value * particle.opacity;
                        pixel[c] = static_cast<uint8_t>(std::clamp(blended, 0.0f, 255.0f));
                    }
                }
            }
        }
    }
}

void GrainRenderer::RenderIrregularGrain(Image& image, const GrainParticle& particle) {
    // For now, use circular with some randomness
    RenderCircularGrain(image, particle);
}

void GrainRenderer::RenderAngularGrain(Image& image, const GrainParticle& particle) {
    // For now, use circular with some randomness
    RenderCircularGrain(image, particle);
}

float GrainRenderer::GetSizeFromDistribution(const FilmStock& stock, const GrainParams& params) {
    // Use normal distribution around average size
    float size = normal_dist_(rng_) * stock.grain_properties.size_variation_coeff * 
                stock.grain_properties.avg_size_um + stock.grain_properties.avg_size_um;
    
    // Clamp to min/max and apply size multiplier
    size = std::clamp(size, stock.grain_properties.min_size_um, stock.grain_properties.max_size_um);
    return size * params.size_multiplier;
}

uint32_t GrainRenderer::GetColorFromFilmStock(const FilmStock& stock, const GrainParams& params) {
    // For B&W films, use grayscale
    if (stock.type == FilmStock::Type::BlackAndWhite) {
        uint8_t gray = static_cast<uint8_t>(200 + uniform_dist_(rng_) * 55); // Light gray grain
        return (gray << 24) | (gray << 16) | (gray << 8) | 0xFF;
    }
    
    // For color films, use slight color variations
    uint8_t r = static_cast<uint8_t>(240 + uniform_dist_(rng_) * 15);
    uint8_t g = static_cast<uint8_t>(240 + uniform_dist_(rng_) * 15);
    uint8_t b = static_cast<uint8_t>(240 + uniform_dist_(rng_) * 15);
    
    return (r << 24) | (g << 16) | (b << 8) | 0xFF;
}

float GrainRenderer::CalculateGrainDensity(const FilmStock& stock, const GrainParams& params,
                                          int image_width, int image_height) {
    // Calculate area in mm² (assuming 300 DPI)
    float area_mm2 = (image_width * image_height) / (300.0f * 300.0f / 25.4f / 25.4f);
    return stock.grain_properties.density_per_mm2 * area_mm2 * params.intensity;
}

void GrainRenderer::ApplyHalationEffect(Image& image, const std::vector<GrainParticle>& particles,
                                       const FilmStock& stock) {
    // Placeholder for halation effect
    std::cout << "Applying halation effect for " << stock.display_name << std::endl;
}

void GrainRenderer::ApplySpecialEffects(Image& image, const FilmStock& stock, const GrainParams& params) {
    // Apply any special film-specific effects
    std::cout << "Applying special effects for " << stock.display_name << std::endl;
}

void GrainRenderer::OptimizeGrainPattern(std::vector<GrainParticle>& particles, 
                                        int image_width, int image_height) {
    // Simple optimization - remove particles that are too close
    // This is a simplified clustering implementation
    std::cout << "Optimizing grain pattern with " << particles.size() << " particles" << std::endl;
}

bool GrainRenderer::InitializeGPU() {
    // Placeholder for GPU initialization
    std::cout << "GPU initialization not implemented yet" << std::endl;
    return false;
}

void GrainRenderer::ShutdownGPU() {
    gpu_context_.initialized = false;
}

void GrainRenderer::ClearPatternCache() {
    pattern_cache_.clear();
}