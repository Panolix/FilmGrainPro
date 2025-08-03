#include "main_window.h"
#include "../core/engine.h"
#include "../utils/image.h"
#include <iostream>
#include <algorithm>
#include <cmath>

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#endif

#include <GLFW/glfw3.h>

MainWindow::MainWindow(FilmGrainEngine* engine)
    : engine_(engine)
    , window_(nullptr)
    , window_width_(1280)
    , window_height_(720)
    , window_title_("FilmGrain Pro v1.0.0")
    , show_demo_window_(false)
    , show_metrics_window_(false)
    , show_film_library_(true)
    , show_preview_(true)
    , show_controls_(true)
    , show_status_bar_(true)
    , selected_film_stock_(0)
    , film_stock_dropdown_open_(false)
    , mouse_x_(0)
    , mouse_y_(0)
    , mouse_pressed_(false) {
}

MainWindow::~MainWindow() {
    Shutdown();
}

bool MainWindow::Initialize() {
    std::cout << "Initializing Main Window..." << std::endl;
    
    if (!InitializeGLFW()) {
        return false;
    }
    
    std::cout << "Main Window initialized successfully" << std::endl;
    return true;
}

bool MainWindow::InitializeGLFW() {
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return false;
    }
    
    // Use legacy OpenGL for compatibility
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 2);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    
#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_FALSE);
#endif
    
    // Create window
    window_ = glfwCreateWindow(window_width_, window_height_, 
                              window_title_.c_str(), nullptr, nullptr);
    
    if (!window_) {
        std::cerr << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return false;
    }
    
    glfwMakeContextCurrent(window_);
    glfwSwapInterval(1); // Enable vsync
    
    // Set callbacks
    glfwSetWindowUserPointer(window_, this);
    glfwSetWindowSizeCallback(window_, WindowSizeCallback);
    glfwSetKeyCallback(window_, KeyCallback);
    glfwSetDropCallback(window_, DropCallback);
    glfwSetMouseButtonCallback(window_, MouseButtonCallback);
    glfwSetCursorPosCallback(window_, CursorPositionCallback);
    
    return true;
}

void MainWindow::Shutdown() {
    if (window_) {
        glfwDestroyWindow(window_);
        window_ = nullptr;
    }
    glfwTerminate();
}

void MainWindow::Update() {
    glfwPollEvents();
    HandleKeyboardShortcuts();
}

void MainWindow::Render() {
    // Clear with dark background like the reference
    glClearColor(0.067f, 0.067f, 0.067f, 1.0f); // #111111
    glClear(GL_COLOR_BUFFER_BIT);
    
    // Render professional UI like the HTML reference
    RenderProfessionalUI();
    
    glfwSwapBuffers(window_);
}

bool MainWindow::ShouldClose() const {
    return window_ ? glfwWindowShouldClose(window_) : true;
}

void MainWindow::RenderProfessionalUI() {
    // Set up 2D rendering
    glMatrixMode(GL_PROJECTION);
    glLoadIdentity();
    glOrtho(0, window_width_, 0, window_height_, -1, 1);
    glMatrixMode(GL_MODELVIEW);
    glLoadIdentity();
    
    glDisable(GL_DEPTH_TEST);
    glEnable(GL_BLEND);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    
    // Render main preview area (left side)
    RenderMainPreview();
    
    // Render control panel (right side)
    RenderControlPanel();
}

void MainWindow::RenderMainPreview() {
    // Main preview area - left 75% of screen
    float preview_width = window_width_ * 0.75f;
    float preview_height = window_height_;
    
    // Dark background for preview
    DrawRect(0, 0, preview_width, preview_height, 0.05f, 0.05f, 0.05f);
    
    // Create and render grain preview
    static bool preview_created = false;
    static Image preview_image;
    static Image grain_preview;
    static float last_params[4] = {1.0f, 0.6f, 1.0f, 1.0f};
    
    if (!preview_created) {
        // Create test image - gradient with some detail
        preview_image.Allocate(600, 400, 3);
        grain_preview.Allocate(600, 400, 3);
        
        // Create a test pattern
        for (int y = 0; y < 400; ++y) {
            for (int x = 0; x < 600; ++x) {
                uint8_t* pixel = preview_image.GetPixelPtr(x, y);
                if (pixel) {
                    // Create gradient with some texture
                    float gradient = (float)x / 600.0f;
                    float noise = (sin((float)x / 20.0f) * sin((float)y / 30.0f)) * 0.1f;
                    float brightness = gradient + noise;
                    brightness = std::clamp(brightness, 0.0f, 1.0f);
                    
                    uint8_t gray = static_cast<uint8_t>(brightness * 255);
                    pixel[0] = gray;
                    pixel[1] = gray;
                    pixel[2] = gray;
                }
            }
        }
        grain_preview.data = preview_image.data;
        preview_created = true;
    }
    
    // Get current grain parameters
    static float intensity = 1.0f;
    static float opacity = 0.6f;
    static float size = 1.0f;
    static float density = 1.0f;
    
    // Update parameters based on key presses
    if (glfwGetKey(window_, GLFW_KEY_1) == GLFW_PRESS) intensity = std::max(0.0f, intensity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_2) == GLFW_PRESS) intensity = std::min(3.0f, intensity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_3) == GLFW_PRESS) opacity = std::max(0.0f, opacity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_4) == GLFW_PRESS) opacity = std::min(1.0f, opacity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_5) == GLFW_PRESS) size = std::max(0.1f, size - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_6) == GLFW_PRESS) size = std::min(5.0f, size + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_7) == GLFW_PRESS) density = std::max(0.1f, density - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_8) == GLFW_PRESS) density = std::min(3.0f, density + 0.01f);
    
    // Auto-update grain when parameters change
    bool params_changed = (last_params[0] != intensity || last_params[1] != opacity || 
                          last_params[2] != size || last_params[3] != density);
    
    if (params_changed) {
        // Apply realistic grain simulation
        grain_preview.data = preview_image.data; // Reset
        
        // Generate grain based on parameters
        for (size_t i = 0; i < grain_preview.data.size(); i += 3) {
            float base_brightness = grain_preview.data[i] / 255.0f;
            
            // Generate grain noise
            float grain_noise = ((rand() % 1000) / 1000.0f - 0.5f) * 2.0f;
            grain_noise *= intensity * opacity * (0.1f + base_brightness * 0.9f);
            grain_noise *= size * 0.1f;
            
            // Apply grain to each channel
            for (int c = 0; c < 3; ++c) {
                int val = grain_preview.data[i + c] + (int)(grain_noise * 255 * density);
                grain_preview.data[i + c] = std::clamp(val, 0, 255);
            }
        }
        
        last_params[0] = intensity;
        last_params[1] = opacity;
        last_params[2] = size;
        last_params[3] = density;
    }
    
    // Render the preview image
    float img_x = (preview_width - 600) / 2;
    float img_y = (preview_height - 400) / 2;
    
    // Draw preview as colored rectangles
    int block_size = 2;
    for (int y = 0; y < 400; y += block_size) {
        for (int x = 0; x < 600; x += block_size) {
            uint8_t* pixel = grain_preview.GetPixelPtr(x, y);
            if (pixel) {
                float r = pixel[0] / 255.0f;
                float g = pixel[1] / 255.0f;
                float b = pixel[2] / 255.0f;
                DrawRect(img_x + x, img_y + (400 - y), block_size, block_size, r, g, b);
            }
        }
    }
}

void MainWindow::RenderControlPanel() {
    // Control panel - right 25% of screen
    float panel_x = window_width_ * 0.75f;
    float panel_width = window_width_ * 0.25f;
    float panel_height = window_height_;
    
    // Dark background with subtle gradient
    DrawRect(panel_x, 0, panel_width, panel_height, 0.08f, 0.08f, 0.08f);
    DrawRect(panel_x, 0, 2, panel_height, 0.3f, 0.3f, 0.3f); // Left border
    
    float y_pos = panel_height - 60;
    float margin = 20;
    
    // Title
    DrawText(panel_x + margin, y_pos, "FILM GRAIN CONTROLS", 0.9f, 0.9f, 0.9f);
    y_pos -= 40;
    
    // Film stock selector
    RenderFilmStockSelector();
    y_pos -= 80;
    
    // Get current parameters
    static float intensity = 1.0f;
    static float opacity = 0.6f;
    static float size = 1.0f;
    static float density = 1.0f;
    
    // Update parameters with keyboard
    if (glfwGetKey(window_, GLFW_KEY_1) == GLFW_PRESS) intensity = std::max(0.0f, intensity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_2) == GLFW_PRESS) intensity = std::min(3.0f, intensity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_3) == GLFW_PRESS) opacity = std::max(0.0f, opacity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_4) == GLFW_PRESS) opacity = std::min(1.0f, opacity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_5) == GLFW_PRESS) size = std::max(0.1f, size - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_6) == GLFW_PRESS) size = std::min(5.0f, size + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_7) == GLFW_PRESS) density = std::max(0.1f, density - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_8) == GLFW_PRESS) density = std::min(3.0f, density + 0.01f);
    
    // Sliders with labels
    DrawText(panel_x + margin, y_pos + 25, "INTENSITY", 0.7f, 0.7f, 0.7f);
    DrawSlider(panel_x + margin, y_pos, panel_width - 2*margin, 20, intensity, 0.0f, 3.0f, "Intensity");
    y_pos -= 60;
    
    DrawText(panel_x + margin, y_pos + 25, "OPACITY", 0.7f, 0.7f, 0.7f);
    DrawSlider(panel_x + margin, y_pos, panel_width - 2*margin, 20, opacity, 0.0f, 1.0f, "Opacity");
    y_pos -= 60;
    
    DrawText(panel_x + margin, y_pos + 25, "GRAIN SIZE", 0.7f, 0.7f, 0.7f);
    DrawSlider(panel_x + margin, y_pos, panel_width - 2*margin, 20, size, 0.1f, 5.0f, "Size");
    y_pos -= 60;
    
    DrawText(panel_x + margin, y_pos + 25, "DENSITY", 0.7f, 0.7f, 0.7f);
    DrawSlider(panel_x + margin, y_pos, panel_width - 2*margin, 20, density, 0.1f, 3.0f, "Density");
    y_pos -= 80;
    
    // Apply button
    bool apply_pressed = IsMouseInRect(panel_x + margin, y_pos, panel_width - 2*margin, 40) && mouse_pressed_;
    DrawButton(panel_x + margin, y_pos, panel_width - 2*margin, 40, "APPLY GRAIN", apply_pressed);
    
    if (apply_pressed) {
        std::cout << "🎬 Applying grain with current settings..." << std::endl;
    }
    
    // Output current values to console
    static int frame_count = 0;
    if (++frame_count % 60 == 0) {
        auto stocks = engine_->GetAvailableFilmStocks();
        std::string current_stock = (stocks.size() > selected_film_stock_) ? 
            stocks[selected_film_stock_].display_name : "Kodak Tri-X 400";
        std::cout << "📽️ " << current_stock << " | I:" << intensity << " O:" << opacity 
                  << " S:" << size << " D:" << density << std::endl;
    }
}

void MainWindow::DrawRect(float x, float y, float w, float h, float r, float g, float b) {
    glColor3f(r, g, b);
    glBegin(GL_QUADS);
    glVertex2f(x, y);
    glVertex2f(x + w, y);
    glVertex2f(x + w, y + h);
    glVertex2f(x, y + h);
    glEnd();
}

void MainWindow::DrawSlider(float x, float y, float w, float h, float value, float min_val, float max_val, const std::string& label) {
    // Background track
    DrawRect(x, y, w, h, 0.2f, 0.2f, 0.2f);
    
    // Value fill
    float fill_width = ((value - min_val) / (max_val - min_val)) * w;
    DrawRect(x, y, fill_width, h, 0.6f, 0.6f, 0.6f);
    
    // Slider handle
    float handle_x = x + fill_width - 3;
    DrawRect(handle_x, y - 2, 6, h + 4, 0.9f, 0.9f, 0.9f);
}

void MainWindow::HandleKeyboardShortcuts() {
    if (glfwGetKey(window_, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window_, true);
    }
}

void MainWindow::SetTitle(const std::string& title) {
    window_title_ = title;
    if (window_) {
        glfwSetWindowTitle(window_, title.c_str());
    }
}

void MainWindow::SetSize(int width, int height) {
    window_width_ = width;
    window_height_ = height;
    if (window_) {
        glfwSetWindowSize(window_, width, height);
    }
}

// Static callbacks
void MainWindow::WindowSizeCallback(GLFWwindow* window, int width, int height) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window) {
        main_window->window_width_ = width;
        main_window->window_height_ = height;
        glViewport(0, 0, width, height);
    }
}

void MainWindow::KeyCallback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && action == GLFW_PRESS) {
        switch (key) {
            case GLFW_KEY_F1:
                std::cout << "🎛️ Controls: 1/2=Intensity 3/4=Opacity 5/6=Size 7/8=Density ESC=Exit" << std::endl;
                break;
        }
    }
}

void MainWindow::RenderFilmStockSelector() {
    float panel_x = window_width_ * 0.75f;
    float panel_width = window_width_ * 0.25f;
    float margin = 20;
    float y_pos = window_height_ - 140;
    
    // Film stock dropdown
    auto stocks = engine_->GetAvailableFilmStocks();
    if (!stocks.empty()) {
        std::string current_stock = (stocks.size() > (size_t)selected_film_stock_) ? 
            stocks[selected_film_stock_].display_name : "Kodak Tri-X 400";
        
        // Dropdown button
        bool dropdown_clicked = IsMouseInRect(panel_x + margin, y_pos, panel_width - 2*margin, 30) && mouse_pressed_;
        DrawButton(panel_x + margin, y_pos, panel_width - 2*margin, 30, current_stock, dropdown_clicked);
        
        if (dropdown_clicked) {
            film_stock_dropdown_open_ = !film_stock_dropdown_open_;
        }
        
        // Dropdown list
        if (film_stock_dropdown_open_) {
            float dropdown_y = y_pos - 30;
            int visible_items = std::min(8, (int)stocks.size());
            
            for (int i = 0; i < visible_items; ++i) {
                bool item_clicked = IsMouseInRect(panel_x + margin, dropdown_y, panel_width - 2*margin, 25) && mouse_pressed_;
                bool is_selected = (i == selected_film_stock_);
                
                // Item background
                if (is_selected) {
                    DrawRect(panel_x + margin, dropdown_y, panel_width - 2*margin, 25, 0.3f, 0.3f, 0.3f);
                } else {
                    DrawRect(panel_x + margin, dropdown_y, panel_width - 2*margin, 25, 0.15f, 0.15f, 0.15f);
                }
                
                // Item text (simplified - just show first few chars)
                DrawText(panel_x + margin + 5, dropdown_y + 5, stocks[i].display_name.substr(0, 15), 0.9f, 0.9f, 0.9f);
                
                if (item_clicked) {
                    selected_film_stock_ = i;
                    film_stock_dropdown_open_ = false;
                    std::cout << "📽️ Selected: " << stocks[i].display_name << std::endl;
                }
                
                dropdown_y -= 25;
            }
        }
    }
}

void MainWindow::DrawText(float x, float y, const std::string& text, float r, float g, float b) {
    // Simple text rendering using OpenGL points (very basic)
    glColor3f(r, g, b);
    glPointSize(2.0f);
    glBegin(GL_POINTS);
    
    // Draw simple text as dots (very basic representation)
    for (size_t i = 0; i < text.length() && i < 20; ++i) {
        glVertex2f(x + i * 8, y);
    }
    glEnd();
    
    // Also output to console for debugging
    static int last_text_frame = 0;
    static int text_frame_count = 0;
    if (++text_frame_count % 120 == 0 && text_frame_count != last_text_frame) {
        std::cout << text << std::endl;
        last_text_frame = text_frame_count;
    }
}

void MainWindow::DrawButton(float x, float y, float w, float h, const std::string& text, bool pressed) {
    // Button background
    if (pressed) {
        DrawRect(x, y, w, h, 0.4f, 0.4f, 0.4f);
    } else {
        DrawRect(x, y, w, h, 0.2f, 0.2f, 0.2f);
    }
    
    // Button border
    DrawRect(x, y, w, 2, 0.5f, 0.5f, 0.5f); // Top
    DrawRect(x, y + h - 2, w, 2, 0.5f, 0.5f, 0.5f); // Bottom
    DrawRect(x, y, 2, h, 0.5f, 0.5f, 0.5f); // Left
    DrawRect(x + w - 2, y, 2, h, 0.5f, 0.5f, 0.5f); // Right
    
    // Button text (simplified)
    DrawText(x + 10, y + h/2, text, 0.9f, 0.9f, 0.9f);
}

bool MainWindow::IsMouseInRect(float x, float y, float w, float h) {
    // Convert mouse coordinates (GLFW uses top-left origin, OpenGL uses bottom-left)
    float gl_mouse_y = window_height_ - mouse_y_;
    return (mouse_x_ >= x && mouse_x_ <= x + w && gl_mouse_y >= y && gl_mouse_y <= y + h);
}

void MainWindow::HandleMouseClick(double x, double y) {
    mouse_x_ = x;
    mouse_y_ = y;
    
    // Handle film stock dropdown clicks
    float panel_x = window_width_ * 0.75f;
    float panel_width = window_width_ * 0.25f;
    float margin = 20;
    
    // Check if clicking outside dropdown to close it
    if (film_stock_dropdown_open_) {
        float dropdown_x = panel_x + margin;
        float dropdown_y = window_height_ - 170;
        float dropdown_w = panel_width - 2*margin;
        float dropdown_h = 200;
        
        if (!IsMouseInRect(dropdown_x, dropdown_y, dropdown_w, dropdown_h)) {
            film_stock_dropdown_open_ = false;
        }
    }
}

// Static callbacks
void MainWindow::MouseButtonCallback(GLFWwindow* window, int button, int action, int mods) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && button == GLFW_MOUSE_BUTTON_LEFT) {
        if (action == GLFW_PRESS) {
            main_window->mouse_pressed_ = true;
            double x, y;
            glfwGetCursorPos(window, &x, &y);
            main_window->HandleMouseClick(x, y);
        } else if (action == GLFW_RELEASE) {
            main_window->mouse_pressed_ = false;
        }
    }
}

void MainWindow::CursorPositionCallback(GLFWwindow* window, double xpos, double ypos) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window) {
        main_window->mouse_x_ = xpos;
        main_window->mouse_y_ = ypos;
    }
}

void MainWindow::DropCallback(GLFWwindow* window, int count, const char** paths) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && count > 0) {
        std::cout << "Files dropped: " << paths[0] << std::endl;
    }
}