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
    
    // Load both JSON files
    bool success = true;
    success &= LoadFromFile("stocks5.json");
    success &= LoadFromFile("advanced-shapeetc.json");
    
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
            
            auto film_stock = CreateFilmStockFromBasicJSON(stock_id, stock_data);
            if (film_stock) {
                film_stocks_[stock_id] = std::unique_ptr<FilmStock>(film_stock);
                std::cout << "Loaded film stock: " << film_stock->display_name << std::endl;
            }
        }
        return true;
    } catch (const json::exception& e) {
        std::cerr << "Error parsing basic film stock JSON: " << e.what() << std::endl;
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