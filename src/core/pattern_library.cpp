#include "pattern_library.h"
#include <iostream>
#include <random>
#include <chrono>
#include <algorithm>

PatternLibrary::PatternLibrary() 
    : max_cache_size_mb_(256)
    , current_cache_size_(0)
    , is_initialized_(false) {
}

PatternLibrary::~PatternLibrary() {
    Shutdown();
}

bool PatternLibrary::Initialize() {
    if (is_initialized_) return true;
    
    std::cout << "Initializing Pattern Library..." << std::endl;
    pattern_cache_.clear();
    current_cache_size_ = 0;
    is_initialized_ = true;
    return true;
}

void PatternLibrary::Shutdown() {
    ClearCache();
    is_initialized_ = false;
}

bool PatternLibrary::GenerateGrainPattern(const FilmStock& stock, int width, int height, Image& output) {
    if (!is_initialized_) return false;
    
    output.Allocate(width, height, 4); // RGBA for grain patterns
    
    // Generate pattern based on crystal type
    if (stock.grain_properties.crystal_type == "tabular_t_grain") {
        GenerateTabularGrainPattern(stock, width, height, output);
    } else if (stock.grain_properties.shape == "irregular_clustered" || 
               stock.grain_properties.shape == "angular_irregular") {
        GenerateIrregularGrainPattern(stock, width, height, output);
    } else {
        GenerateConventionalGrainPattern(stock, width, height, output);
    }
    
    // Apply clustering and distribution effects
    ApplyGrainDistribution(output, stock);
    if (stock.grain_properties.clustering != "isolated") {
        ApplyClusteringEffect(output, stock);
    }
    
    return true;
}

void PatternLibrary::GenerateTabularGrainPattern(const FilmStock& stock, int width, int height, Image& output) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> pos_dist(0.0f, 1.0f);
    std::normal_distribution<float> size_dist(stock.grain_properties.avg_size_um, 
                                            stock.grain_properties.avg_size_um * stock.grain_properties.size_variation_coeff);
    
    // Calculate number of grains based on density
    float area_mm2 = (width * height) / (300.0f * 300.0f / 25.4f / 25.4f); // Assuming 300 DPI
    int num_grains = static_cast<int>(stock.grain_properties.density_per_mm2 * area_mm2);
    
    // Clear the output
    std::fill(output.data.begin(), output.data.end(), 0);
    
    for (int i = 0; i < num_grains; ++i) {
        float x = pos_dist(gen) * width;
        float y = pos_dist(gen) * height;
        float size = std::max(0.5f, size_dist(gen));
        
        // Tabular grains are elongated
        float width_grain = size * stock.grain_properties.aspect_ratio_y;
        float height_grain = size;
        
        // Draw elongated grain
        int x_start = static_cast<int>(x - width_grain / 2);
        int x_end = static_cast<int>(x + width_grain / 2);
        int y_start = static_cast<int>(y - height_grain / 2);
        int y_end = static_cast<int>(y + height_grain / 2);
        
        for (int py = y_start; py <= y_end; ++py) {
            for (int px = x_start; px <= x_end; ++px) {
                if (px >= 0 && px < width && py >= 0 && py < height) {
                    uint8_t* pixel = output.GetPixelPtr(px, py);
                    if (pixel) {
                        // Set grain opacity
                        uint8_t opacity = static_cast<uint8_t>(
                            std::clamp(stock.visual_properties.opacity_min * 255 + 
                                     pos_dist(gen) * (stock.visual_properties.opacity_max - stock.visual_properties.opacity_min) * 255, 
                                     0.0f, 255.0f));
                        pixel[3] = std::max(pixel[3], opacity);
                        pixel[0] = pixel[1] = pixel[2] = 255; // White grain
                    }
                }
            }
        }
    }
}

void PatternLibrary::GenerateIrregularGrainPattern(const FilmStock& stock, int width, int height, Image& output) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> pos_dist(0.0f, 1.0f);
    std::normal_distribution<float> size_dist(stock.grain_properties.avg_size_um, 
                                            stock.grain_properties.avg_size_um * stock.grain_properties.size_variation_coeff);
    
    float area_mm2 = (width * height) / (300.0f * 300.0f / 25.4f / 25.4f);
    int num_grains = static_cast<int>(stock.grain_properties.density_per_mm2 * area_mm2);
    
    std::fill(output.data.begin(), output.data.end(), 0);
    
    for (int i = 0; i < num_grains; ++i) {
        float x = pos_dist(gen) * width;
        float y = pos_dist(gen) * height;
        float size = std::max(0.3f, size_dist(gen));
        
        // Irregular shape - use random points around center
        int num_points = 6 + (gen() % 4); // 6-9 points for irregular shape
        std::vector<std::pair<float, float>> points;
        
        for (int p = 0; p < num_points; ++p) {
            float angle = (2.0f * M_PI * p) / num_points + pos_dist(gen) * 0.5f;
            float radius = size * (0.5f + pos_dist(gen) * 0.5f);
            points.push_back({x + radius * cos(angle), y + radius * sin(angle)});
        }
        
        // Fill irregular grain shape (simplified as circle for now)
        int radius = static_cast<int>(size);
        for (int py = static_cast<int>(y - radius); py <= static_cast<int>(y + radius); ++py) {
            for (int px = static_cast<int>(x - radius); px <= static_cast<int>(x + radius); ++px) {
                if (px >= 0 && px < width && py >= 0 && py < height) {
                    float dist = sqrt((px - x) * (px - x) + (py - y) * (py - y));
                    if (dist <= radius) {
                        uint8_t* pixel = output.GetPixelPtr(px, py);
                        if (pixel) {
                            uint8_t opacity = static_cast<uint8_t>(
                                std::clamp((1.0f - dist / radius) * stock.visual_properties.opacity_max * 255, 
                                         0.0f, 255.0f));
                            pixel[3] = std::max(pixel[3], opacity);
                            pixel[0] = pixel[1] = pixel[2] = 255;
                        }
                    }
                }
            }
        }
    }
}

void PatternLibrary::GenerateConventionalGrainPattern(const FilmStock& stock, int width, int height, Image& output) {
    // Similar to irregular but with more regular, cubic-like shapes
    GenerateIrregularGrainPattern(stock, width, height, output);
}

void PatternLibrary::ApplyGrainDistribution(Image& pattern, const FilmStock& stock) {
    // Apply spatial distribution effects based on film characteristics
    if (stock.algorithmic_data.spatial_correlation > 0.1f) {
        // Apply some spatial correlation effects
        // This is a simplified implementation
        std::cout << "Applying spatial correlation: " << stock.algorithmic_data.spatial_correlation << std::endl;
    }
}

void PatternLibrary::ApplyClusteringEffect(Image& pattern, const FilmStock& stock) {
    // Apply clustering based on film properties
    std::cout << "Applying clustering effect: " << stock.grain_properties.clustering << std::endl;
}

void PatternLibrary::ClearCache() {
    pattern_cache_.clear();
    current_cache_size_ = 0;
}

size_t PatternLibrary::GetCacheSize() const {
    return current_cache_size_;
}

uint64_t PatternLibrary::GetCurrentTimeMs() const {
    return std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::steady_clock::now().time_since_epoch()).count();
}