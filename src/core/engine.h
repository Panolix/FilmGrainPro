#pragma once

#include <memory>
#include <vector>
#include <string>
#include <unordered_map>
#include <functional>

// Forward declarations
class GrainRenderer;
class ColorProcessor;
class PatternLibrary;
class FilmStockDatabase;
struct Image;
struct FilmStock;

/**
 * Core engine for FilmGrain Pro
 * Manages all grain processing, rendering, and film stock simulation
 */
class FilmGrainEngine {
public:
    FilmGrainEngine();
    ~FilmGrainEngine();
    
    // Engine lifecycle
    bool Initialize();
    void Shutdown();
    
    // Main processing functions
    bool ProcessImage(const Image& input, Image& output, 
                     const std::string& film_stock_id,
                     const struct GrainParams& params);
    
    bool ProcessImageAsync(const Image& input, 
                          const std::string& film_stock_id,
                          const struct GrainParams& params,
                          std::function<void(const Image&)> callback);
    
    // Film stock management
    std::vector<FilmStock> GetAvailableFilmStocks() const;
    const FilmStock* GetFilmStock(const std::string& id) const;
    bool LoadFilmStockDatabase(const std::string& path);
    
    // Real-time preview
    void SetPreviewImage(const Image& image);
    void UpdatePreview(const std::string& film_stock_id, 
                      const struct GrainParams& params);
    const Image& GetPreviewImage() const;
    
    // Performance monitoring
    struct PerformanceStats {
        float fps;
        float processing_time_ms;
        size_t memory_usage_mb;
        float gpu_utilization;
    };
    PerformanceStats GetPerformanceStats() const;
    
    // Singleton access
    static FilmGrainEngine& Instance();
    
private:
    // Core components
    std::unique_ptr<GrainRenderer> grain_renderer_;
    std::unique_ptr<ColorProcessor> color_processor_;
    std::unique_ptr<PatternLibrary> pattern_library_;
    std::unique_ptr<FilmStockDatabase> film_stock_db_;
    
    // State management
    std::unique_ptr<Image> preview_image_;
    std::unique_ptr<Image> processed_preview_;
    bool preview_needs_update_;
    
    // Performance tracking
    mutable PerformanceStats performance_stats_;
    
    // Internal methods
    void InitializeGPU();
    void LoadDefaultFilmStocks();
    void UpdatePerformanceStats() const;
};