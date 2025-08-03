#pragma once

#include <GLFW/glfw3.h>
#include <vector>
#include <string>
#include "../core/engine.h"
#include "../utils/image.h"

class HTMLLikeUI {
public:
    HTMLLikeUI(FilmGrainEngine* engine);
    ~HTMLLikeUI();
    
    bool Initialize(GLFWwindow* window);
    void Render();
    void HandleMouse(double x, double y, bool pressed);
    void HandleKeyboard();
    
private:
    FilmGrainEngine* engine_;
    GLFWwindow* window_;
    int window_width_, window_height_;
    
    // UI State
    float intensity_;
    float opacity_;
    float grain_size_;
    float density_;
    int selected_film_stock_;
    bool dropdown_open_;
    
    // Mouse
    double mouse_x_, mouse_y_;
    bool mouse_pressed_;
    bool mouse_was_pressed_;
    
    // Film stocks
    std::vector<std::string> film_stock_names_;
    
    // Preview
    Image preview_image_;
    Image grain_image_;
    bool need_grain_update_;
    
    // UI Layout
    struct UILayout {
        float preview_x, preview_y, preview_w, preview_h;
        float panel_x, panel_y, panel_w, panel_h;
        float dropdown_x, dropdown_y, dropdown_w, dropdown_h;
        float slider_x, slider_y, slider_w, slider_h;
        float button_x, button_y, button_w, button_h;
    } layout_;
    
    // Rendering
    void SetupLayout();
    void LoadFilmStocks();
    void CreateTestImage();
    void UpdateGrain();
    
    void RenderBackground();
    void RenderPreviewPanel();
    void RenderControlPanel();
    void RenderDropdown();
    void RenderSliders();
    void RenderApplyButton();
    
    // UI Components
    void DrawRect(float x, float y, float w, float h, float r, float g, float b, float a = 1.0f);
    void DrawBorder(float x, float y, float w, float h, float thickness, float r, float g, float b);
    void DrawText(float x, float y, const std::string& text, float size = 12.0f);
    void DrawImage(float x, float y, float w, float h);
    void DrawSlider(float x, float y, float w, float h, float value, float min_val, float max_val, const std::string& label);
    
    // Interaction
    bool IsMouseOver(float x, float y, float w, float h);
    void HandleSliderDrag(float slider_x, float slider_y, float slider_w, float* value, float min_val, float max_val);
    void HandleDropdownClick();
    void HandleApplyClick();
};