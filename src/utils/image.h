#pragma once

#include <vector>
#include <string>
#include <memory>

/**
 * Image data structure for FilmGrain Pro
 * Supports various formats and provides efficient memory management
 */
struct Image {
    // Image dimensions
    int width = 0;
    int height = 0;
    int channels = 0;  // 1=grayscale, 3=RGB, 4=RGBA
    
    // Image data (row-major order)
    std::vector<uint8_t> data;
    
    // Metadata
    std::string format;  // "RGB", "RGBA", "GRAY"
    float dpi = 300.0f;
    
    // Constructors
    Image() = default;
    Image(int w, int h, int c);
    Image(int w, int h, int c, const uint8_t* pixel_data);
    
    // Basic operations
    bool IsValid() const;
    size_t GetDataSize() const;
    uint8_t* GetPixelPtr(int x, int y);
    const uint8_t* GetPixelPtr(int x, int y) const;
    
    // Image operations
    bool LoadFromFile(const std::string& filepath);
    bool SaveToFile(const std::string& filepath) const;
    bool Resize(int new_width, int new_height);
    Image Clone() const;
    
    // Pixel access helpers
    void SetPixel(int x, int y, uint8_t r, uint8_t g, uint8_t b, uint8_t a = 255);
    void GetPixel(int x, int y, uint8_t& r, uint8_t& g, uint8_t& b, uint8_t& a) const;
    
    // Memory management
    void Clear();
    void Allocate(int w, int h, int c);
};

/**
 * Film stock data structure
 */
struct FilmStock {
    // Basic information
    std::string id;
    std::string display_name;
    std::string manufacturer;
    int iso_speed;
    
    enum class Type {
        BlackAndWhite,
        ColorNegative,
        ColorSlide
    } type;
    
    // Grain characteristics
    struct GrainProperties {
        float min_size_um;
        float max_size_um;
        float avg_size_um;
        float size_variation_coeff;
        int density_per_mm2;
        std::string crystal_type;  // "cubic", "tabular", "irregular"
        std::string shape;         // "irregular", "flat", "angular"
        float aspect_ratio_x;
        float aspect_ratio_y;
        std::string clustering;    // "none", "mild", "moderate", "heavy"
    } grain_properties;
    
    // Visual properties
    struct VisualProperties {
        float opacity_min;
        float opacity_max;
        float opacity_variation;
        std::string contrast_level;  // "low", "medium", "high"
        std::string edge_definition; // "soft", "sharp"
        float highlight_visibility;
        float shadow_visibility;
        float midtone_prominence;
    } visual_properties;
    
    // Color properties
    struct ColorProperties {
        std::string primary_cast;    // "neutral", "warm", "cool"
        struct RGBRange {
            int r_min, r_max;
            int g_min, g_max;
            int b_min, b_max;
            float weight;
        };
        std::vector<RGBRange> rgb_ranges;
        std::string color_variation; // "none", "low", "medium", "high"
        std::string saturation_level;
    } color_properties;
    
    // Special effects
    struct SpecialEffects {
        std::string halation;        // "none", "mild", "moderate", "strong"
        uint32_t halation_color;     // RGBA color
        float halation_radius;
        std::vector<std::string> unique_artifacts;
        std::string light_interaction;
    } special_effects;
    
    // Algorithmic data for rendering
    struct AlgorithmicData {
        std::string clustering_algorithm;
        std::string distribution_function;
        float spatial_correlation;
        float fractal_dimension;
    } algorithmic_data;
};

/**
 * Grain rendering parameters
 */
struct GrainParams {
    float intensity = 1.0f;          // 0.0 - 3.0
    float opacity = 0.6f;            // 0.0 - 1.0
    float size_multiplier = 1.0f;    // 0.1 - 5.0
    float density_multiplier = 1.0f; // 0.1 - 3.0
    
    enum class ProcessingType {
        Normal,
        Push1Stop,
        Push2Stop,
        Pull1Stop
    } processing = ProcessingType::Normal;
    
    bool enable_halation = false;
    float halation_strength = 1.0f;
    
    // Advanced parameters
    float spatial_correlation = 0.0f;  // 0.0 - 1.0
    float clustering_strength = 0.0f;  // 0.0 - 1.0
    bool enable_color_variation = true;
    float color_variation_strength = 1.0f;
    
    // Performance settings
    enum class Quality {
        Draft,    // Fast preview
        Normal,   // Balanced
        High,     // Best quality
        Ultra     // Maximum quality
    } quality = Quality::Normal;
    
    bool use_gpu_acceleration = true;
};