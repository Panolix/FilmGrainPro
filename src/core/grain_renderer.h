#pragma once

#include <memory>
#include <vector>
#include <random>
#include "../utils/image.h"

struct FilmStock;
struct GrainParams;

/**
 * High-performance grain rendering system
 * Implements scientifically accurate grain patterns based on film stock research
 */
class GrainRenderer {
public:
    GrainRenderer();
    ~GrainRenderer();
    
    // Initialization
    bool Initialize();
    void Shutdown();
    
    // Main grain application
    bool ApplyGrain(Image& image, const FilmStock& stock, const GrainParams& params);
    
    // GPU-accelerated rendering
    bool ApplyGrainGPU(Image& image, const FilmStock& stock, const GrainParams& params);
    
    // CPU fallback rendering
    bool ApplyGrainCPU(Image& image, const FilmStock& stock, const GrainParams& params);
    
    // Pattern management
    bool LoadGrainPattern(const std::string& pattern_id, const std::string& path);
    void ClearPatternCache();
    
private:
    struct GrainParticle {
        float x, y;                    // Position
        float size;                    // Size in pixels
        float opacity;                 // Opacity 0-1
        float aspect_ratio;            // Width/height ratio
        float rotation;                // Rotation in radians
        uint32_t color;               // RGBA color
        int shape_type;               // Shape identifier
    };
    
    struct GrainPattern {
        std::vector<GrainParticle> particles;
        int width, height;
        std::string film_stock_id;
        float density_per_mm2;
    };
    
    // GPU resources
    struct GPUContext {
        void* vulkan_device;          // VkDevice
        void* compute_pipeline;       // VkPipeline
        void* descriptor_set;         // VkDescriptorSet
        void* grain_buffer;           // VkBuffer
        void* uniform_buffer;         // VkBuffer
        bool initialized;
    } gpu_context_;
    
    // Pattern cache
    std::unordered_map<std::string, std::unique_ptr<GrainPattern>> pattern_cache_;
    
    // Random number generation
    std::mt19937 rng_;
    std::uniform_real_distribution<float> uniform_dist_;
    std::normal_distribution<float> normal_dist_;
    
    // Internal methods
    void GenerateGrainPattern(const FilmStock& stock, const GrainParams& params,
                             int image_width, int image_height,
                             std::vector<GrainParticle>& particles);
    
    void ApplyGrainParticle(Image& image, const GrainParticle& particle);
    
    void RenderTabularGrain(Image& image, const GrainParticle& particle);
    void RenderIrregularGrain(Image& image, const GrainParticle& particle);
    void RenderAngularGrain(Image& image, const GrainParticle& particle);
    void RenderCircularGrain(Image& image, const GrainParticle& particle);
    
    void ApplyHalationEffect(Image& image, const std::vector<GrainParticle>& particles,
                            const FilmStock& stock);
    
    void ApplySpecialEffects(Image& image, const FilmStock& stock, const GrainParams& params);
    
    // GPU methods
    bool InitializeGPU();
    void ShutdownGPU();
    bool CreateComputePipeline();
    bool UploadGrainData(const std::vector<GrainParticle>& particles);
    
    // Utility methods
    float GetSizeFromDistribution(const FilmStock& stock, const GrainParams& params);
    uint32_t GetColorFromFilmStock(const FilmStock& stock, const GrainParams& params);
    float CalculateGrainDensity(const FilmStock& stock, const GrainParams& params,
                               int image_width, int image_height);
    
    // Performance optimization
    void OptimizeGrainPattern(std::vector<GrainParticle>& particles, 
                             int image_width, int image_height);
};