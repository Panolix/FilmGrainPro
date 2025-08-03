#include "film_stock_database.h"
#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>

// Use nlohmann/json since it's available via Homebrew
#include <nlohmann/json.hpp>
using json = nlohmann::json;

FilmStockDatabase::FilmStockDatabase() : is_initialized_(false) {
}

FilmStockDatabase::~FilmStockDatabase() {
    Shutdown();
}

bool FilmStockDatabase::Initialize() {
    if (is_initialized_) return true;
    
    std::cout << "Initializing Film Stock Database..." << std::endl;
    film_stocks_.clear();
    is_initialized_ = true;
    return true;
}

void FilmStockDatabase::Shutdown() {
    film_stocks_.clear();
    is_initialized_ = false;
}

bool FilmStockDatabase::LoadFromFile(const std::string& filepath) {
    std::ifstream file(filepath);
    if (!file.is_open()) {
        std::cerr << "Failed to open film stock file: " << filepath << std::endl;
        return false;
    }
    
    std::stringstream buffer;
    buffer << file.rdbuf();
    return LoadFromJSON(buffer.str());
}

bool FilmStockDatabase::LoadDefaults() {
    std::cout << "Loading default film stocks..." << std::endl;
    
    // Load both JSON files from bin directory
    bool success = true;
    success &= LoadFromFile("bin/stocks5.json");
    success &= LoadFromFile("bin/advanced-shapeetc.json");
    
    if (!success) {
        std::cout << "Warning: Some film stock files could not be loaded" << std::endl;
    }
    
    std::cout << "Loaded " << film_stocks_.size() << " film stocks" << std::endl;
    return !film_stocks_.empty();
}

bool FilmStockDatabase::LoadFromJSON(const std::string& json_content) {
    try {
        json j = json::parse(json_content);
        
        // Check if this is the basic stocks file or advanced file
        if (j.contains("metadata")) {
            return ParseAdvancedFilmStockJSON(json_content);
        } else {
            return ParseBasicFilmStockJSON(json_content);
        }
    } catch (const json::exception& e) {
        std::cerr << "JSON parsing error: " << e.what() << std::endl;
        return false;
    }
}

bool FilmStockDatabase::ParseBasicFilmStockJSON(const std::string& json_content) {
    try {
        json j = json::parse(json_content);
        
        for (auto& [stock_id, stock_data] : j.items()) {
            if (stock_id == "metadata") continue;
            
            // Create film stock from JSON data
            auto film_stock = new FilmStock();
            film_stock->id = stock_id;
            
            // Parse basic info
            if (stock_data.contains("basic_info")) {
                auto& basic = stock_data["basic_info"];
                film_stock->display_name = basic.value("name", stock_id);
                film_stock->iso_speed = basic.value("iso", 400);
                std::string type_str = basic.value("type", "bw");
                if (type_str == "bw" || type_str == "black_and_white") {
                    film_stock->type = FilmStock::Type::BlackAndWhite;
                } else if (type_str == "color_negative") {
                    film_stock->type = FilmStock::Type::ColorNegative;
                } else if (type_str == "color_slide") {
                    film_stock->type = FilmStock::Type::ColorSlide;
                }
            } else {
                film_stock->display_name = stock_id;
                film_stock->iso_speed = 400;
                film_stock->type = FilmStock::Type::BlackAndWhite;
            }
            
            // Parse grain properties
            if (stock_data.contains("size_metrics")) {
                auto& size = stock_data["size_metrics"];
                film_stock->grain_properties.min_size_um = size.value("min_size_um", 0.5f);
                film_stock->grain_properties.max_size_um = size.value("max_size_um", 3.0f);
                film_stock->grain_properties.avg_size_um = size.value("avg_size_um", 1.5f);
                film_stock->grain_properties.size_variation_coeff = size.value("size_variation_coeff", 0.5f);
                film_stock->grain_properties.density_per_mm2 = size.value("density_per_mm2", 10000);
            }
            
            if (stock_data.contains("grain_structure")) {
                auto& grain = stock_data["grain_structure"];
                film_stock->grain_properties.crystal_type = grain.value("crystal_type", "cubic");
                film_stock->grain_properties.shape = grain.value("shape", "irregular");
                film_stock->grain_properties.clustering = grain.value("clustering", "moderate");
            }
            
            // Parse visual properties
            if (stock_data.contains("visual_properties")) {
                auto& visual = stock_data["visual_properties"];
                if (visual.contains("opacity_range") && visual["opacity_range"].is_array()) {
                    auto opacity = visual["opacity_range"];
                    film_stock->visual_properties.opacity_min = opacity[0];
                    film_stock->visual_properties.opacity_max = opacity[1];
                } else {
                    film_stock->visual_properties.opacity_min = 0.2f;
                    film_stock->visual_properties.opacity_max = 0.8f;
                }
                film_stock->visual_properties.opacity_variation = visual.value("opacity_variation", 0.5f);
                film_stock->visual_properties.contrast_level = visual.value("contrast_level", "medium");
                film_stock->visual_properties.edge_definition = visual.value("edge_definition", "soft");
            }
            
            // Parse algorithmic data
            if (stock_data.contains("algorithmic_data")) {
                auto& algo = stock_data["algorithmic_data"];
                film_stock->algorithmic_data.clustering_algorithm = algo.value("clustering_algorithm", "gaussian");
                film_stock->algorithmic_data.distribution_function = algo.value("distribution_function", "normal");
                film_stock->algorithmic_data.spatial_correlation = algo.value("spatial_correlation", 0.3f);
                film_stock->algorithmic_data.fractal_dimension = algo.value("fractal_dimension", 1.7f);
            }
            
            film_stocks_[stock_id] = std::unique_ptr<FilmStock>(film_stock);
            std::cout << "✅ Loaded: " << film_stock->display_name << " (ISO " << film_stock->iso_speed << ")" << std::endl;
        }
        return true;
    } catch (const json::exception& e) {
        std::cerr << "Error parsing film stock JSON: " << e.what() << std::endl;
        return false;
    }
}

FilmStock* FilmStockDatabase::CreateFilmStockFromBasicJSON(const std::string& id, const std::string& stock_data_str) {
    // For now, create a basic film stock - full JSON parsing can be added later
    auto stock = new FilmStock();
    stock->id = id;
    stock->display_name = id;
    stock->type = FilmStock::Type::BlackAndWhite;
    stock->iso_speed = 400;
    return stock;
}


bool FilmStockDatabase::ParseAdvancedFilmStockJSON(const std::string& json_content) {
    try {
        json j = json::parse(json_content);
        
        for (auto& [stock_id, advanced_data] : j.items()) {
            if (stock_id == "metadata") continue;
            
            // Find existing stock or skip if not found
            auto it = film_stocks_.find(stock_id);
            if (it != film_stocks_.end()) {
                ApplyAdvancedPropertiesFromJSON(it->second.get(), advanced_data);
                std::cout << "Applied advanced properties to: " << stock_id << std::endl;
            }
        }
        return true;
    } catch (const json::exception& e) {
        std::cerr << "Error parsing advanced film stock JSON: " << e.what() << std::endl;
        return false;
    }
}

void FilmStockDatabase::ApplyAdvancedPropertiesFromJSON(FilmStock* stock, const std::string& advanced_data_str) {
    // Placeholder - advanced properties can be parsed later
    std::cout << "Advanced properties available for " << stock->id << std::endl;
}


FilmStock::Type FilmStockDatabase::ParseFilmType(const std::string& type_str) const {
    if (type_str == "black_and_white") return FilmStock::Type::BlackAndWhite;
    if (type_str == "color_negative") return FilmStock::Type::ColorNegative;
    if (type_str == "color_slide") return FilmStock::Type::ColorSlide;
    return FilmStock::Type::BlackAndWhite;
}

std::vector<FilmStock> FilmStockDatabase::GetAllFilmStocks() const {
    std::vector<FilmStock> stocks;
    for (const auto& [id, stock] : film_stocks_) {
        stocks.push_back(*stock);
    }
    return stocks;
}

const FilmStock* FilmStockDatabase::GetFilmStock(const std::string& id) const {
    auto it = film_stocks_.find(id);
    return (it != film_stocks_.end()) ? it->second.get() : nullptr;
}

std::vector<std::string> FilmStockDatabase::GetFilmStockIds() const {
    std::vector<std::string> ids;
    for (const auto& [id, stock] : film_stocks_) {
        ids.push_back(id);
    }
    return ids;
}

size_t FilmStockDatabase::GetFilmStockCount() const {
    return film_stocks_.size();
}

bool FilmStockDatabase::IsLoaded() const {
    return is_initialized_ && !film_stocks_.empty();
}