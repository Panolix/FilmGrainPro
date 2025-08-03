#pragma once

#include "../utils/image.h"

/**
 * Color Processing System for FilmGrain Pro
 * Handles color grading, film response curves, and color science
 */
class ColorProcessor {
public:
    ColorProcessor();
    ~ColorProcessor();
    
    // Initialization
    bool Initialize();
    void Shutdown();
    
    // Main processing functions
    bool ProcessImage(Image& image, const FilmStock& stock, const GrainParams& params);
    
    // Color grading operations
    void ApplyFilmResponse(Image& image, const FilmStock& stock);
    void ApplyColorCast(Image& image, const FilmStock& stock);
    void AdjustContrast(Image& image, float contrast_level);
    void AdjustSaturation(Image& image, float saturation);
    
    // Film-specific color science
    void ApplyBlackAndWhiteResponse(Image& image, const FilmStock& stock);
    void ApplyColorNegativeResponse(Image& image, const FilmStock& stock);
    void ApplySlideFilmResponse(Image& image, const FilmStock& stock);
    
private:
    bool is_initialized_;
    
    // Color transformation matrices
    struct ColorMatrix {
        float matrix[3][3];
    };
    
    // Film response curves
    struct ResponseCurve {
        std::vector<float> input_values;
        std::vector<float> output_values;
    };
    
    // Internal methods
    void ApplyColorMatrix(Image& image, const ColorMatrix& matrix);
    void ApplyResponseCurve(Image& image, const ResponseCurve& curve, int channel = -1);
    float InterpolateResponse(const ResponseCurve& curve, float input);
    
    // Utility methods
    void RGBToLab(float r, float g, float b, float& l, float& a, float& b_out);
    void LabToRGB(float l, float a, float b, float& r, float& g, float& b_out);
    void ClampPixel(uint8_t& r, uint8_t& g, uint8_t& b);
};