#include "image.h"
#include <algorithm>
#include <cstring>
#include <iostream>

// Image implementation
Image::Image(int w, int h, int c) : width(w), height(h), channels(c) {
    Allocate(w, h, c);
}

Image::Image(int w, int h, int c, const uint8_t* pixel_data) : width(w), height(h), channels(c) {
    Allocate(w, h, c);
    if (pixel_data) {
        std::memcpy(data.data(), pixel_data, GetDataSize());
    }
}

bool Image::IsValid() const {
    return width > 0 && height > 0 && channels > 0 && !data.empty();
}

size_t Image::GetDataSize() const {
    return static_cast<size_t>(width) * height * channels;
}

uint8_t* Image::GetPixelPtr(int x, int y) {
    if (x < 0 || x >= width || y < 0 || y >= height) return nullptr;
    return &data[(y * width + x) * channels];
}

const uint8_t* Image::GetPixelPtr(int x, int y) const {
    if (x < 0 || x >= width || y < 0 || y >= height) return nullptr;
    return &data[(y * width + x) * channels];
}

bool Image::LoadFromFile(const std::string& filepath) {
    // TODO: Implement using OpenCV or stb_image
    // For now, create a placeholder implementation
    std::cout << "Loading image from: " << filepath << std::endl;
    
    // Create a test image (red gradient)
    Allocate(512, 512, 3);
    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            uint8_t* pixel = GetPixelPtr(x, y);
            pixel[0] = static_cast<uint8_t>((x * 255) / width);  // Red gradient
            pixel[1] = static_cast<uint8_t>((y * 255) / height); // Green gradient
            pixel[2] = 128; // Blue constant
        }
    }
    
    format = "RGB";
    return true;
}

bool Image::SaveToFile(const std::string& filepath) const {
    // TODO: Implement using OpenCV or stb_image
    std::cout << "Saving image to: " << filepath << std::endl;
    return true;
}

bool Image::Resize(int new_width, int new_height) {
    if (new_width <= 0 || new_height <= 0) return false;
    
    // Simple nearest-neighbor resize for now
    std::vector<uint8_t> new_data(new_width * new_height * channels);
    
    for (int y = 0; y < new_height; ++y) {
        for (int x = 0; x < new_width; ++x) {
            int src_x = (x * width) / new_width;
            int src_y = (y * height) / new_height;
            
            const uint8_t* src_pixel = GetPixelPtr(src_x, src_y);
            uint8_t* dst_pixel = &new_data[(y * new_width + x) * channels];
            
            if (src_pixel) {
                std::memcpy(dst_pixel, src_pixel, channels);
            }
        }
    }
    
    width = new_width;
    height = new_height;
    data = std::move(new_data);
    
    return true;
}

Image Image::Clone() const {
    Image copy(width, height, channels);
    copy.data = data;
    copy.format = format;
    copy.dpi = dpi;
    return copy;
}

void Image::SetPixel(int x, int y, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    uint8_t* pixel = GetPixelPtr(x, y);
    if (!pixel) return;
    
    pixel[0] = r;
    if (channels > 1) pixel[1] = g;
    if (channels > 2) pixel[2] = b;
    if (channels > 3) pixel[3] = a;
}

void Image::GetPixel(int x, int y, uint8_t& r, uint8_t& g, uint8_t& b, uint8_t& a) const {
    const uint8_t* pixel = GetPixelPtr(x, y);
    if (!pixel) {
        r = g = b = a = 0;
        return;
    }
    
    r = pixel[0];
    g = (channels > 1) ? pixel[1] : pixel[0];
    b = (channels > 2) ? pixel[2] : pixel[0];
    a = (channels > 3) ? pixel[3] : 255;
}

void Image::Clear() {
    width = height = channels = 0;
    data.clear();
    format.clear();
}

void Image::Allocate(int w, int h, int c) {
    width = w;
    height = h;
    channels = c;
    data.resize(GetDataSize());
    std::fill(data.begin(), data.end(), 0);
}