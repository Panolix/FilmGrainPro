#pragma once

#include <vector>
#include <string>
#include <unordered_map>
#include <memory>
#include "../utils/image.h"

/**
 * Pattern Library for FilmGrain Pro
 * Manages grain patterns, textures, and procedural generation
 */
class PatternLibrary {
public:
    PatternLibrary();
    ~PatternLibrary();
    
    // Initialization
    bool Initialize();
    void Shutdown();
    
    // Pattern management
    bool LoadPattern(const std::string& pattern_id, const std::string& filepath);
    bool GeneratePattern(const std::string& pattern_id, const FilmStock& stock, int width, int height);
    const Image* GetPattern(const std::string& pattern_id) const;
    
    // Pattern generation
    bool GenerateGrainPattern(const FilmStock& stock, int width, int height, Image& output);
    bool GenerateNoisePattern(float intensity, int width, int height, Image& output);
    
    // Cache management
    void ClearCache();
    size_t GetCacheSize() const;
    void SetMaxCacheSize(size_t max_size_mb);
    
private:
    struct PatternInfo {
        std::unique_ptr<Image> pattern;
        std::string film_stock_id;
        size_t memory_size;
        uint64_t last_access_time;
    };
    
    std::unordered_map<std::string, PatternInfo> pattern_cache_;
    size_t max_cache_size_mb_;
    size_t current_cache_size_;
    bool is_initialized_;
    
    // Pattern generation methods
    void GenerateTabularGrainPattern(const FilmStock& stock, int width, int height, Image& output);
    void GenerateIrregularGrainPattern(const FilmStock& stock, int width, int height, Image& output);
    void GenerateConventionalGrainPattern(const FilmStock& stock, int width, int height, Image& output);
    
    // Utility methods
    void ApplyGrainDistribution(Image& pattern, const FilmStock& stock);
    void ApplyClusteringEffect(Image& pattern, const FilmStock& stock);
    void EvictOldPatterns();
    uint64_t GetCurrentTimeMs() const;
};