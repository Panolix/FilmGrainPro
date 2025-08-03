#include "color_processor.h"
#include <iostream>
#include <algorithm>
#include <cmath>

ColorProcessor::ColorProcessor() : is_initialized_(false) {
}

ColorProcessor::~ColorProcessor() {
    Shutdown();
}

bool ColorProcessor::Initialize() {
    if (is_initialized_) return true;
    
    std::cout << "Initializing Color Processor..." << std::endl;
    is_initialized_ = true;
    return true;
}

void ColorProcessor::Shutdown() {
    is_initialized_ = false;
}

bool ColorProcessor::ProcessImage(Image& image, const FilmStock& stock, const GrainParams& params) {
    if (!is_initialized_ || !image.IsValid()) {
        return false;
    }
    
    // Apply film-specific color processing based on type
    switch (stock.type) {
        case FilmStock::Type::BlackAndWhite:
            ApplyBlackAndWhiteResponse(image, stock);
            break;
        case FilmStock::Type::ColorNegative:
            ApplyColorNegativeResponse(image, stock);
            break;
        case FilmStock::Type::ColorSlide:
            ApplySlideFilmResponse(image, stock);
            break;
    }
    
    // Apply contrast adjustments based on film characteristics
    if (stock.visual_properties.contrast_level == "high") {
        AdjustContrast(image, 1.2f);
    } else if (stock.visual_properties.contrast_level == "low") {
        AdjustContrast(image, 0.8f);
    }
    
    return true;
}

void ColorProcessor::ApplyBlackAndWhiteResponse(Image& image, const FilmStock& stock) {
    // Apply characteristic B&W film response
    for (int y = 0; y < image.height; ++y) {
        for (int x = 0; x < image.width; ++x) {
            uint8_t* pixel = image.GetPixelPtr(x, y);
            if (!pixel) continue;
            
            // Convert to grayscale with film-specific weighting
            float r = pixel[0] / 255.0f;
            float g = pixel[1] / 255.0f;
            float b = pixel[2] / 255.0f;
            
            // Film-specific RGB to grayscale conversion
            float gray = 0.299f * r + 0.587f * g + 0.114f * b;
            
            // Apply film characteristic curve
            gray = std::pow(gray, 0.9f); // Slight gamma adjustment
            
            uint8_t gray_value = static_cast<uint8_t>(std::clamp(gray * 255.0f, 0.0f, 255.0f));
            pixel[0] = pixel[1] = pixel[2] = gray_value;
        }
    }
}

void ColorProcessor::ApplyColorNegativeResponse(Image& image, const FilmStock& stock) {
    // Apply color negative film characteristics
    for (int y = 0; y < image.height; ++y) {
        for (int x = 0; x < image.width; ++x) {
            uint8_t* pixel = image.GetPixelPtr(x, y);
            if (!pixel) continue;
            
            float r = pixel[0] / 255.0f;
            float g = pixel[1] / 255.0f;
            float b = pixel[2] / 255.0f;
            
            // Apply characteristic S-curve
            r = std::pow(r, 0.85f);
            g = std::pow(g, 0.85f);
            b = std::pow(b, 0.85f);
            
            uint8_t r_val = static_cast<uint8_t>(r * 255);
            uint8_t g_val = static_cast<uint8_t>(g * 255);
            uint8_t b_val = static_cast<uint8_t>(b * 255);
            ClampPixel(r_val, g_val, b_val);
            
            pixel[0] = static_cast<uint8_t>(r * 255);
            pixel[1] = static_cast<uint8_t>(g * 255);
            pixel[2] = static_cast<uint8_t>(b * 255);
        }
    }
}

void ColorProcessor::ApplySlideFilmResponse(Image& image, const FilmStock& stock) {
    // Apply slide film characteristics (higher contrast, saturated colors)
    for (int y = 0; y < image.height; ++y) {
        for (int x = 0; x < image.width; ++x) {
            uint8_t* pixel = image.GetPixelPtr(x, y);
            if (!pixel) continue;
            
            float r = pixel[0] / 255.0f;
            float g = pixel[1] / 255.0f;
            float b = pixel[2] / 255.0f;
            
            // Higher contrast curve for slide film
            r = std::pow(r, 1.2f);
            g = std::pow(g, 1.2f);
            b = std::pow(b, 1.2f);
            
            pixel[0] = static_cast<uint8_t>(std::clamp(r * 255.0f, 0.0f, 255.0f));
            pixel[1] = static_cast<uint8_t>(std::clamp(g * 255.0f, 0.0f, 255.0f));
            pixel[2] = static_cast<uint8_t>(std::clamp(b * 255.0f, 0.0f, 255.0f));
        }
    }
}

void ColorProcessor::AdjustContrast(Image& image, float contrast_level) {
    for (int y = 0; y < image.height; ++y) {
        for (int x = 0; x < image.width; ++x) {
            uint8_t* pixel = image.GetPixelPtr(x, y);
            if (!pixel) continue;
            
            for (int c = 0; c < image.channels; ++c) {
                float value = pixel[c] / 255.0f;
                value = (value - 0.5f) * contrast_level + 0.5f;
                pixel[c] = static_cast<uint8_t>(std::clamp(value * 255.0f, 0.0f, 255.0f));
            }
        }
    }
}

void ColorProcessor::ClampPixel(uint8_t& r, uint8_t& g, uint8_t& b) {
    // Ensure values are within valid range
    r = std::clamp(static_cast<int>(r), 0, 255);
    g = std::clamp(static_cast<int>(g), 0, 255);
    b = std::clamp(static_cast<int>(b), 0, 255);
}