#include "html_like_ui.h"
#include <iostream>
#include <algorithm>
#include <cmath>

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#endif

HTMLLikeUI::HTMLLikeUI(FilmGrainEngine* engine)
    : engine_(engine)
    , window_(nullptr)
    , window_width_(1280)
    , window_height_(720)
    , intensity_(1.0f)
    , opacity_(0.6f)
    , grain_size_(1.0f)
    , density_(1.0f)
    , selected_film_stock_(0)
    , dropdown_open_(false)
    , mouse_x_(0)
    , mouse_y_(0)
    , mouse_pressed_(false)
    , mouse_was_pressed_(false)
    , need_grain_update_(true) {
}

HTMLLikeUI::~HTMLLikeUI() {
}

bool HTMLLikeUI::Initialize(GLFWwindow* window) {
    window_ = window;
    glfwGetWindowSize(window_, &window_width_, &window_height_);
    
    SetupLayout();
    LoadFilmStocks();
    CreateTestImage();
    
    std::cout << "HTML-like UI initialized with " << film_stock_names_.size() << " film stocks" << std::endl;
    return true;
}

void HTMLLikeUI::SetupLayout() {
    // Layout exactly like HTML version
    layout_.preview_x = 0;
    layout_.preview_y = 0;
    layout_.preview_w = window_width_ * 0.75f;  // 75% width
    layout_.preview_h = window_height_;
    
    layout_.panel_x = window_width_ * 0.75f;    // Right 25%
    layout_.panel_y = 0;
    layout_.panel_w = window_width_ * 0.25f;
    layout_.panel_h = window_height_;
    
    // Dropdown
    layout_.dropdown_x = layout_.panel_x + 20;
    layout_.dropdown_y = 60;
    layout_.dropdown_w = layout_.panel_w - 40;
    layout_.dropdown_h = 40;
    
    // Sliders
    layout_.slider_x = layout_.panel_x + 20;
    layout_.slider_y = 150;
    layout_.slider_w = layout_.panel_w - 40;
    layout_.slider_h = 30;
    
    // Apply button
    layout_.button_x = layout_.panel_x + 20;
    layout_.button_y = window_height_ - 80;
    layout_.button_w = layout_.panel_w - 40;
    layout_.button_h = 50;
}

void HTMLLikeUI::LoadFilmStocks() {
    auto stocks = engine_->GetAvailableFilmStocks();
    film_stock_names_.clear();
    
    for (const auto& stock : stocks) {
        film_stock_names_.push_back(stock.display_name);
    }
    
    if (film_stock_names_.empty()) {
        film_stock_names_ = {
            "Kodak Tri-X 400",
            "Ilford HP5 Plus", 
            "Kodak T-Max 400",
            "Fuji Acros 100",
            "Kodak Portra 400"
        };
    }
    
    std::cout << "Loaded " << film_stock_names_.size() << " film stocks" << std::endl;
}

void HTMLLikeUI::CreateTestImage() {
    // Create gradient test image exactly like HTML
    preview_image_.Allocate(800, 600, 3);
    grain_image_.Allocate(800, 600, 3);
    
    for (int y = 0; y < 600; ++y) {
        for (int x = 0; x < 800; ++x) {
            uint8_t* pixel = preview_image_.GetPixelPtr(x, y);
            if (pixel) {
                uint8_t gray = static_cast<uint8_t>((x * 255) / 800);
                pixel[0] = gray;
                pixel[1] = gray;
                pixel[2] = gray;
            }
        }
    }
    grain_image_.data = preview_image_.data;
}

void HTMLLikeUI::Render() {
    glfwGetWindowSize(window_, &window_width_, &window_height_);
    SetupLayout(); // Update layout if window resized
    
    // Setup OpenGL for proper 2D rendering
    glViewport(0, 0, window_width_, window_height_);
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(0, window_width_, window_height_, 0, -1, 1); // Top-left origin
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    
    glDisable(GL_DEPTH_TEST);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    
    if (need_grain_update_) {
        UpdateGrain();
        need_grain_update_ = false;
    }
    
    RenderBackground();
    RenderPreviewPanel();
    RenderControlPanel();
}

void HTMLLikeUI::RenderBackground() {
    // Dark background like HTML
    DrawRect(0, 0, window_width_, window_height_, 0.067f, 0.067f, 0.067f);
}

void HTMLLikeUI::RenderPreviewPanel() {
    // Left panel - darker background
    DrawRect(layout_.preview_x, layout_.preview_y, layout_.preview_w, layout_.preview_h, 0.04f, 0.04f, 0.04f);
    
    // Center the image
    float img_w = 600;
    float img_h = 450;
    float img_x = layout_.preview_x + (layout_.preview_w - img_w) / 2;
    float img_y = layout_.preview_y + (layout_.preview_h - img_h) / 2;
    
    // Draw the grain image
    DrawImage(img_x, img_y, img_w, img_h);
    
    // White border
    DrawBorder(img_x - 2, img_y - 2, img_w + 4, img_h + 4, 2, 1.0f, 1.0f, 1.0f);
}

void HTMLLikeUI::RenderControlPanel() {
    // Right panel background
    DrawRect(layout_.panel_x, layout_.panel_y, layout_.panel_w, layout_.panel_h, 0.08f, 0.08f, 0.08f);
    
    // Left border
    DrawRect(layout_.panel_x, layout_.panel_y, 2, layout_.panel_h, 0.3f, 0.3f, 0.3f);
    
    // Title
    DrawText(layout_.panel_x + 20, 30, "FILM GRAIN CONTROLS", 16.0f);
    
    RenderDropdown();
    RenderSliders();
    RenderApplyButton();
}

void HTMLLikeUI::RenderDropdown() {
    // Label
    DrawText(layout_.dropdown_x, layout_.dropdown_y - 25, "Film Stock", 12.0f);
    
    // Dropdown button
    bool hovered = IsMouseOver(layout_.dropdown_x, layout_.dropdown_y, layout_.dropdown_w, layout_.dropdown_h);
    float bg_color = hovered ? 0.2f : 0.15f;
    
    DrawRect(layout_.dropdown_x, layout_.dropdown_y, layout_.dropdown_w, layout_.dropdown_h, bg_color, bg_color, bg_color);
    DrawBorder(layout_.dropdown_x, layout_.dropdown_y, layout_.dropdown_w, layout_.dropdown_h, 2, 0.4f, 0.4f, 0.4f);
    
    // Current selection
    if (selected_film_stock_ < (int)film_stock_names_.size()) {
        DrawText(layout_.dropdown_x + 10, layout_.dropdown_y + 12, film_stock_names_[selected_film_stock_], 12.0f);
    }
    
    // Arrow
    DrawText(layout_.dropdown_x + layout_.dropdown_w - 25, layout_.dropdown_y + 12, "▼", 12.0f);
    
    // Dropdown list
    if (dropdown_open_) {
        int visible_items = std::min(6, (int)film_stock_names_.size());
        for (int i = 0; i < visible_items; ++i) {
            float item_y = layout_.dropdown_y + layout_.dropdown_h + i * 35;
            bool item_hovered = IsMouseOver(layout_.dropdown_x, item_y, layout_.dropdown_w, 35);
            bool is_selected = (i == selected_film_stock_);
            
            float item_bg = is_selected ? 0.3f : (item_hovered ? 0.2f : 0.12f);
            DrawRect(layout_.dropdown_x, item_y, layout_.dropdown_w, 35, item_bg, item_bg + 0.1f, item_bg + 0.2f);
            DrawBorder(layout_.dropdown_x, item_y, layout_.dropdown_w, 35, 1, 0.3f, 0.3f, 0.3f);
            
            DrawText(layout_.dropdown_x + 10, item_y + 10, film_stock_names_[i], 11.0f);
            
            // Handle click
            if (item_hovered && mouse_pressed_ && !mouse_was_pressed_) {
                selected_film_stock_ = i;
                dropdown_open_ = false;
                need_grain_update_ = true;
                std::cout << "Selected: " << film_stock_names_[i] << std::endl;
            }
        }
    }
    
    // Handle dropdown button click
    if (hovered && mouse_pressed_ && !mouse_was_pressed_) {
        dropdown_open_ = !dropdown_open_;
    }
}

void HTMLLikeUI::RenderSliders() {
    float y = layout_.slider_y;
    float spacing = 70;
    
    // Intensity slider
    DrawSlider(layout_.slider_x, y, layout_.slider_w, layout_.slider_h, intensity_, 0.0f, 3.0f, "Intensity");
    HandleSliderDrag(layout_.slider_x, y, layout_.slider_w, &intensity_, 0.0f, 3.0f);
    y += spacing;
    
    // Opacity slider
    DrawSlider(layout_.slider_x, y, layout_.slider_w, layout_.slider_h, opacity_, 0.0f, 1.0f, "Opacity");
    HandleSliderDrag(layout_.slider_x, y, layout_.slider_w, &opacity_, 0.0f, 1.0f);
    y += spacing;
    
    // Grain Size slider
    DrawSlider(layout_.slider_x, y, layout_.slider_w, layout_.slider_h, grain_size_, 0.1f, 5.0f, "Grain Size");
    HandleSliderDrag(layout_.slider_x, y, layout_.slider_w, &grain_size_, 0.1f, 5.0f);
    y += spacing;
    
    // Density slider
    DrawSlider(layout_.slider_x, y, layout_.slider_w, layout_.slider_h, density_, 0.1f, 3.0f, "Density");
    HandleSliderDrag(layout_.slider_x, y, layout_.slider_w, &density_, 0.1f, 3.0f);
}

void HTMLLikeUI::RenderApplyButton() {
    bool hovered = IsMouseOver(layout_.button_x, layout_.button_y, layout_.button_w, layout_.button_h);
    bool pressed = hovered && mouse_pressed_;
    
    float r = pressed ? 0.3f : (hovered ? 0.25f : 0.2f);
    float g = pressed ? 0.5f : (hovered ? 0.4f : 0.3f);
    float b = pressed ? 0.7f : (hovered ? 0.5f : 0.4f);
    
    DrawRect(layout_.button_x, layout_.button_y, layout_.button_w, layout_.button_h, r, g, b);
    DrawBorder(layout_.button_x, layout_.button_y, layout_.button_w, layout_.button_h, 2, 0.6f, 0.6f, 0.6f);
    
    DrawText(layout_.button_x + layout_.button_w/2 - 50, layout_.button_y + 18, "APPLY GRAIN", 14.0f);
    
    if (hovered && mouse_pressed_ && !mouse_was_pressed_) {
        need_grain_update_ = true;
        std::cout << "🎬 APPLY GRAIN - " << film_stock_names_[selected_film_stock_] 
                  << " | I:" << intensity_ << " O:" << opacity_ 
                  << " S:" << grain_size_ << " D:" << density_ << std::endl;
    }
}

void HTMLLikeUI::DrawRect(float x, float y, float w, float h, float r, float g, float b, float a) {
    glColor4f(r, g, b, a);
    glBegin(GL_QUADS);
    glVertex2f(x, y);
    glVertex2f(x + w, y);
    glVertex2f(x + w, y + h);
    glVertex2f(x, y + h);
    glEnd();
}

void HTMLLikeUI::DrawBorder(float x, float y, float w, float h, float thickness, float r, float g, float b) {
    glColor3f(r, g, b);
    glLineWidth(thickness);
    glBegin(GL_LINE_LOOP);
    glVertex2f(x, y);
    glVertex2f(x + w, y);
    glVertex2f(x + w, y + h);
    glVertex2f(x, y + h);
    glEnd();
}

void HTMLLikeUI::DrawText(float x, float y, const std::string& text, float size) {
    // Draw visible text using simple rectangles
    glColor3f(0.9f, 0.9f, 0.9f);
    
    float char_width = size * 0.6f;
    float char_height = size;
    
    for (size_t i = 0; i < text.length(); ++i) {
        float char_x = x + i * char_width;
        
        // Draw visible character representation
        glBegin(GL_QUADS);
        glVertex2f(char_x, y);
        glVertex2f(char_x + char_width * 0.8f, y);
        glVertex2f(char_x + char_width * 0.8f, y + char_height);
        glVertex2f(char_x, y + char_height);
        glEnd();
    }
}

void HTMLLikeUI::DrawImage(float x, float y, float w, float h) {
    // Draw the grain image as colored pixels
    int step = 3; // Performance optimization
    
    for (int py = 0; py < h; py += step) {
        for (int px = 0; px < w; px += step) {
            int src_x = (px * grain_image_.width) / w;
            int src_y = (py * grain_image_.height) / h;
            
            const uint8_t* pixel = grain_image_.GetPixelPtr(src_x, src_y);
            if (pixel) {
                float r = pixel[0] / 255.0f;
                float g = pixel[1] / 255.0f;
                float b = pixel[2] / 255.0f;
                DrawRect(x + px, y + py, step, step, r, g, b);
            }
        }
    }
}

void HTMLLikeUI::DrawSlider(float x, float y, float w, float h, float value, float min_val, float max_val, const std::string& label) {
    // Label
    DrawText(x, y - 20, label, 12.0f);
    
    // Track
    DrawRect(x, y, w, h, 0.2f, 0.2f, 0.2f);
    
    // Fill
    float fill_ratio = (value - min_val) / (max_val - min_val);
    float fill_width = fill_ratio * w;
    DrawRect(x, y, fill_width, h, 0.4f, 0.6f, 0.8f);
    
    // Handle
    float handle_x = x + fill_width - 8;
    DrawRect(handle_x, y - 3, 16, h + 6, 0.9f, 0.9f, 0.9f);
    
    // Value
    char value_str[32];
    snprintf(value_str, sizeof(value_str), "%.2f", value);
    DrawText(x + w + 10, y + 5, value_str, 10.0f);
}

bool HTMLLikeUI::IsMouseOver(float x, float y, float w, float h) {
    return (mouse_x_ >= x && mouse_x_ <= x + w && mouse_y_ >= y && mouse_y_ <= y + h);
}

void HTMLLikeUI::HandleSliderDrag(float slider_x, float slider_y, float slider_w, float* value, float min_val, float max_val) {
    if (IsMouseOver(slider_x, slider_y, slider_w, 30) && mouse_pressed_) {
        float ratio = std::clamp((float)(mouse_x_ - slider_x) / slider_w, 0.0f, 1.0f);
        float new_value = min_val + ratio * (max_val - min_val);
        
        if (std::abs(new_value - *value) > 0.01f) {
            *value = new_value;
            need_grain_update_ = true;
        }
    }
}

void HTMLLikeUI::HandleMouse(double x, double y, bool pressed) {
    mouse_was_pressed_ = mouse_pressed_;
    mouse_x_ = x;
    mouse_y_ = y;
    mouse_pressed_ = pressed;
    
    // Close dropdown if clicking outside
    if (pressed && !mouse_was_pressed_ && dropdown_open_) {
        if (!IsMouseOver(layout_.dropdown_x, layout_.dropdown_y, layout_.dropdown_w, 250)) {
            dropdown_open_ = false;
        }
    }
}

void HTMLLikeUI::HandleKeyboard() {
    // Handle keyboard shortcuts if needed
}

void HTMLLikeUI::UpdateGrain() {
    // Reset to original
    grain_image_.data = preview_image_.data;
    
    // Apply realistic grain simulation
    for (size_t i = 0; i < grain_image_.data.size(); i += 3) {
        float base_brightness = grain_image_.data[i] / 255.0f;
        
        // Generate grain noise
        float grain_noise = ((rand() % 2000) / 1000.0f - 1.0f);
        grain_noise *= intensity_ * opacity_ * (0.2f + base_brightness * 0.8f);
        grain_noise *= grain_size_ * 0.15f;
        
        // Apply to all channels
        for (int c = 0; c < 3; ++c) {
            int val = grain_image_.data[i + c] + (int)(grain_noise * 255 * density_);
            grain_image_.data[i + c] = std::clamp(val, 0, 255);
        }
    }
}