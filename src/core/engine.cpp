#include "engine.h"
#include "grain_renderer.h"
#include "color_processor.h"
#include "pattern_library.h"
#include "film_stock_database.h"
#include "../utils/image.h"
#include "../utils/performance_monitor.h"
#include <iostream>
#include <chrono>
#include <thread>

FilmGrainEngine::FilmGrainEngine() 
    : grain_renderer_(nullptr)
    , color_processor_(nullptr)
    , pattern_library_(nullptr)
    , film_stock_db_(nullptr)
    , preview_image_(nullptr)
    , processed_preview_(nullptr)
    , preview_needs_update_(false)
    , performance_stats_{0.0f, 0.0f, 0, 0.0f}
{
}

FilmGrainEngine::~FilmGrainEngine() {
    Shutdown();
}

bool FilmGrainEngine::Initialize() {
    std::cout << "Initializing FilmGrain Engine...\n";
    
    try {
        // Initialize GPU context
        InitializeGPU();
        
        // Create core components
        grain_renderer_ = std::make_unique<GrainRenderer>();
        if (!grain_renderer_->Initialize()) {
            std::cerr << "Failed to initialize grain renderer\n";
            return false;
        }
        
        color_processor_ = std::make_unique<ColorProcessor>();
        if (!color_processor_->Initialize()) {
            std::cerr << "Failed to initialize color processor\n";
            return false;
        }
        
        pattern_library_ = std::make_unique<PatternLibrary>();
        if (!pattern_library_->Initialize()) {
            std::cerr << "Failed to initialize pattern library\n";
            return false;
        }
        
        film_stock_db_ = std::make_unique<FilmStockDatabase>();
        if (!film_stock_db_->Initialize()) {
            std::cerr << "Failed to initialize film stock database\n";
            return false;
        }
        
        // Load default film stocks
        LoadDefaultFilmStocks();
        
        // Initialize preview images
        preview_image_ = std::make_unique<Image>(1920, 1080, 4);
        processed_preview_ = std::make_unique<Image>(1920, 1080, 4);
        
        std::cout << "FilmGrain Engine initialized successfully\n";
        return true;
        
    } catch (const std::exception& e) {
        std::cerr << "Engine initialization failed: " << e.what() << std::endl;
        return false;
    }
}

void FilmGrainEngine::Shutdown() {
    std::cout << "Shutting down FilmGrain Engine...\n";
    
    // Clean up in reverse order
    processed_preview_.reset();
    preview_image_.reset();
    film_stock_db_.reset();
    pattern_library_.reset();
    color_processor_.reset();
    grain_renderer_.reset();
    
    std::cout << "FilmGrain Engine shutdown complete\n";
}

bool FilmGrainEngine::ProcessImage(const Image& input, Image& output, 
                                  const std::string& film_stock_id,
                                  const GrainParams& params) {
    auto start_time = std::chrono::high_resolution_clock::now();
    
    try {
        // Get film stock data
        const FilmStock* stock = GetFilmStock(film_stock_id);
        if (!stock) {
            std::cerr << "Film stock not found: " << film_stock_id << std::endl;
            return false;
        }
        
        // Ensure output image is correct size
        output.Allocate(input.width, input.height, input.channels);
        
        // Copy input to output as base
        output.data = input.data;
        
        // Apply color processing first
        color_processor_->ProcessImage(output, *stock, params);
        
        // Apply grain rendering
        grain_renderer_->ApplyGrain(output, *stock, params);
        
        // Update performance stats
        auto end_time = std::chrono::high_resolution_clock::now();
        auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);
        performance_stats_.processing_time_ms = static_cast<float>(duration.count());
        performance_stats_.fps = 1000.0f / performance_stats_.processing_time_ms;
        
        UpdatePerformanceStats();
        
        return true;
        
    } catch (const std::exception& e) {
        std::cerr << "Image processing failed: " << e.what() << std::endl;
        return false;
    }
}

bool FilmGrainEngine::ProcessImageAsync(const Image& input, 
                                       const std::string& film_stock_id,
                                       const GrainParams& params,
                                       std::function<void(const Image&)> callback) {
    // Launch async processing
    std::thread([this, input, film_stock_id, params, callback]() {
        Image output;
        if (ProcessImage(input, output, film_stock_id, params)) {
            callback(output);
        }
    }).detach();
    
    return true;
}

std::vector<FilmStock> FilmGrainEngine::GetAvailableFilmStocks() const {
    if (!film_stock_db_) {
        return {};
    }
    return film_stock_db_->GetAllFilmStocks();
}

const FilmStock* FilmGrainEngine::GetFilmStock(const std::string& id) const {
    if (!film_stock_db_) {
        return nullptr;
    }
    return film_stock_db_->GetFilmStock(id);
}

bool FilmGrainEngine::LoadFilmStockDatabase(const std::string& path) {
    if (!film_stock_db_) {
        return false;
    }
    return film_stock_db_->LoadFromFile(path);
}

void FilmGrainEngine::SetPreviewImage(const Image& image) {
    if (!preview_image_) {
        preview_image_ = std::make_unique<Image>(image.width, image.height, image.channels);
    }
    
    preview_image_->data = image.data;
    preview_needs_update_ = true;
}

void FilmGrainEngine::UpdatePreview(const std::string& film_stock_id, 
                                   const GrainParams& params) {
    if (!preview_image_ || !processed_preview_) {
        return;
    }
    
    // Process preview image
    ProcessImage(*preview_image_, *processed_preview_, film_stock_id, params);
    preview_needs_update_ = false;
}

const Image& FilmGrainEngine::GetPreviewImage() const {
    if (processed_preview_ && !preview_needs_update_) {
        return *processed_preview_;
    }
    return *preview_image_;
}

FilmGrainEngine::PerformanceStats FilmGrainEngine::GetPerformanceStats() const {
    return performance_stats_;
}

FilmGrainEngine& FilmGrainEngine::Instance() {
    static FilmGrainEngine instance;
    return instance;
}

void FilmGrainEngine::InitializeGPU() {
    // Initialize Vulkan/OpenGL context
    // This would be implemented with actual GPU initialization
    std::cout << "Initializing GPU context...\n";
    
    // For now, just simulate GPU initialization
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    
    std::cout << "GPU context initialized\n";
}

void FilmGrainEngine::LoadDefaultFilmStocks() {
    if (!film_stock_db_) {
        return;
    }
    
    std::cout << "Loading default film stocks...\n";
    
    // Load from embedded data or default file
    film_stock_db_->LoadDefaults();
    
    auto stocks = film_stock_db_->GetAllFilmStocks();
    std::cout << "Loaded " << stocks.size() << " film stocks\n";
}

void FilmGrainEngine::UpdatePerformanceStats() const {
    // Update memory usage
    performance_stats_.memory_usage_mb = PerformanceMonitor::GetMemoryUsageMB();
    
    // Update GPU utilization (would be real GPU query)
    performance_stats_.gpu_utilization = 45.0f; // Simulated
}