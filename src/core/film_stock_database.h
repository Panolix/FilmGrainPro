#pragma once

#include <vector>
#include <string>
#include <unordered_map>
#include <memory>
#include "../utils/image.h"

// We'll include the full header in the .cpp file

/**
 * Film Stock Database - Manages loading and access to film stock data
 * Loads from JSON files and provides efficient access to film stock properties
 */
class FilmStockDatabase {
public:
    FilmStockDatabase();
    ~FilmStockDatabase();
    
    // Initialization
    bool Initialize();
    void Shutdown();
    
    // Data loading
    bool LoadFromFile(const std::string& filepath);
    bool LoadDefaults();
    bool LoadFromJSON(const std::string& json_content);
    
    // Data access
    std::vector<FilmStock> GetAllFilmStocks() const;
    const FilmStock* GetFilmStock(const std::string& id) const;
    std::vector<std::string> GetFilmStockIds() const;
    
    // Filtering and search
    std::vector<FilmStock> GetFilmStocksByType(FilmStock::Type type) const;
    std::vector<FilmStock> GetFilmStocksByISO(int min_iso, int max_iso) const;
    std::vector<FilmStock> SearchFilmStocks(const std::string& query) const;
    
    // Statistics
    size_t GetFilmStockCount() const;
    bool IsLoaded() const;
    
private:
    std::unordered_map<std::string, std::unique_ptr<FilmStock>> film_stocks_;
    bool is_initialized_;
    
    // JSON parsing helpers
    bool ParseBasicFilmStockJSON(const std::string& json_content);
    bool ParseAdvancedFilmStockJSON(const std::string& json_content);
    FilmStock* CreateFilmStockFromBasicJSON(const std::string& id, const std::string& stock_data_str);
    void ApplyAdvancedPropertiesFromJSON(FilmStock* stock, const std::string& advanced_data_str);
    
    // Utility methods
    FilmStock::Type ParseFilmType(const std::string& type_str) const;
    void ValidateFilmStock(const FilmStock& stock) const;
};