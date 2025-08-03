#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <cmath>
#include <GLFW/glfw3.h>
#include "core/engine.h"
#include "utils/image.h"

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#include <OpenGL/glu.h>
#endif

// Simple UI class embedded directly
class SimpleUI {
public:
    FilmGrainEngine* engine;
    GLFWwindow* window;
    int width, height;
    
    // UI state
    float intensity = 1.0f;
    float opacity = 0.6f;
    float grain_size = 1.0f;
    float density = 1.0f;
    int selected_film = 0;
    bool dropdown_open = false;
    
    // Mouse
    double mx = 0, my = 0;
    bool mouse_down = false;
    bool mouse_was_down = false;
    
    // Film stocks
    std::vector<std::string> films;
    
    // Preview
    Image preview, grain;
    bool need_update = true;
    
    SimpleUI(FilmGrainEngine* eng) : engine(eng) {
        // Load film stocks
        auto stocks = engine->GetAvailableFilmStocks();
        for (const auto& stock : stocks) {
            films.push_back(stock.display_name);
        }
        if (films.empty()) {
            films = {"Kodak Tri-X 400", "Ilford HP5 Plus", "Kodak T-Max 400"};
        }
        
        // Create test image
        preview.Allocate(800, 600, 3);
        grain.Allocate(800, 600, 3);
        for (int y = 0; y < 600; ++y) {
            for (int x = 0; x < 800; ++x) {
                uint8_t* pixel = preview.GetPixelPtr(x, y);
                if (pixel) {
                    uint8_t gray = (x * 255) / 800;
                    pixel[0] = pixel[1] = pixel[2] = gray;
                }
            }
        }
        grain.data = preview.data;
        
        std::cout << "SimpleUI created with " << films.size() << " films" << std::endl;
    }
    
    void HandleMouse(double x, double y, bool pressed) {
        mouse_was_down = mouse_down;
        mx = x; my = y; mouse_down = pressed;
        
        float panel_x = width * 0.75f;
        float panel_w = width * 0.25f;
        
        // Dropdown
        float dropdown_x = panel_x + 20;
        float dropdown_y = 80;
        float dropdown_w = panel_w - 40;
        
        if (InRect(mx, my, dropdown_x, dropdown_y, dropdown_w, 40)) {
            if (pressed && !mouse_was_down) {
                dropdown_open = !dropdown_open;
                std::cout << "Dropdown toggled: " << dropdown_open << std::endl;
            }
        }
        
        // Dropdown items
        if (dropdown_open) {
            for (int i = 0; i < std::min(6, (int)films.size()); ++i) {
                float item_y = dropdown_y + 40 + i * 35;
                if (InRect(mx, my, dropdown_x, item_y, dropdown_w, 35)) {
                    if (pressed && !mouse_was_down) {
                        selected_film = i;
                        dropdown_open = false;
                        need_update = true;
                        std::cout << "Selected: " << films[i] << std::endl;
                    }
                }
            }
        }
        
        // Sliders
        float slider_x = panel_x + 20;
        float slider_w = panel_w - 40;
        
        HandleSlider(slider_x, 180, slider_w, &intensity, 0.0f, 3.0f, "Intensity");
        HandleSlider(slider_x, 260, slider_w, &opacity, 0.0f, 1.0f, "Opacity");
        HandleSlider(slider_x, 340, slider_w, &grain_size, 0.1f, 5.0f, "Size");
        HandleSlider(slider_x, 420, slider_w, &density, 0.1f, 3.0f, "Density");
        
        // Apply button
        if (InRect(mx, my, slider_x, height - 80, slider_w, 50)) {
            if (pressed && !mouse_was_down) {
                need_update = true;
                std::cout << "APPLY: " << films[selected_film] << " I:" << intensity 
                          << " O:" << opacity << " S:" << grain_size << " D:" << density << std::endl;
            }
        }
    }
    
    void HandleSlider(float x, float y, float w, float* value, float min_val, float max_val, const std::string& name) {
        if (InRect(mx, my, x, y, w, 30) && mouse_down) {
            float ratio = std::clamp((float)(mx - x) / w, 0.0f, 1.0f);
            float new_val = min_val + ratio * (max_val - min_val);
            if (std::abs(new_val - *value) > 0.01f) {
                *value = new_val;
                need_update = true;
                std::cout << name << ": " << *value << std::endl;
            }
        }
    }
    
    bool InRect(double px, double py, float x, float y, float w, float h) {
        return px >= x && px <= x + w && py >= y && py <= y + h;
    }
    
    void Render() {
        glfwGetWindowSize(window, &width, &height);
        
        glViewport(0, 0, width, height);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0, width, height, 0, -1, 1);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        
        glDisable(GL_DEPTH_TEST);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        
        if (need_update) {
            UpdateGrain();
            need_update = false;
        }
        
        // Background
        DrawRect(0, 0, width, height, 0.067f, 0.067f, 0.067f);
        
        // Preview panel
        float preview_w = width * 0.75f;
        DrawRect(0, 0, preview_w, height, 0.04f, 0.04f, 0.04f);
        
        // Image
        float img_w = 600, img_h = 450;
        float img_x = (preview_w - img_w) / 2;
        float img_y = (height - img_h) / 2;
        DrawImage(img_x, img_y, img_w, img_h);
        
        // Control panel
        float panel_x = width * 0.75f;
        float panel_w = width * 0.25f;
        DrawRect(panel_x, 0, panel_w, height, 0.08f, 0.08f, 0.08f);
        DrawRect(panel_x, 0, 2, height, 0.3f, 0.3f, 0.3f);
        
        // Title
        DrawText(panel_x + 20, 30, "FILM GRAIN CONTROLS");
        
        // Dropdown
        float dropdown_x = panel_x + 20;
        float dropdown_w = panel_w - 40;
        DrawText(dropdown_x, 60, "Film Stock");
        
        bool dropdown_hover = InRect(mx, my, dropdown_x, 80, dropdown_w, 40);
        float bg = dropdown_hover ? 0.2f : 0.15f;
        DrawRect(dropdown_x, 80, dropdown_w, 40, bg, bg, bg);
        DrawBorder(dropdown_x, 80, dropdown_w, 40, 0.4f, 0.4f, 0.4f);
        
        if (selected_film < (int)films.size()) {
            DrawText(dropdown_x + 10, 95, films[selected_film]);
        }
        DrawText(dropdown_x + dropdown_w - 25, 95, "v");
        
        if (dropdown_open) {
            for (int i = 0; i < std::min(6, (int)films.size()); ++i) {
                float item_y = 120 + i * 35;
                bool item_hover = InRect(mx, my, dropdown_x, item_y, dropdown_w, 35);
                bool is_selected = (i == selected_film);
                
                float item_bg = is_selected ? 0.3f : (item_hover ? 0.2f : 0.12f);
                DrawRect(dropdown_x, item_y, dropdown_w, 35, item_bg, item_bg + 0.1f, item_bg + 0.2f);
                DrawText(dropdown_x + 10, item_y + 10, films[i]);
            }
        }
        
        // Sliders
        DrawSlider(panel_x + 20, 180, panel_w - 40, intensity, 0.0f, 3.0f, "Intensity");
        DrawSlider(panel_x + 20, 260, panel_w - 40, opacity, 0.0f, 1.0f, "Opacity");
        DrawSlider(panel_x + 20, 340, panel_w - 40, grain_size, 0.1f, 5.0f, "Grain Size");
        DrawSlider(panel_x + 20, 420, panel_w - 40, density, 0.1f, 3.0f, "Density");
        
        // Apply button
        bool button_hover = InRect(mx, my, panel_x + 20, height - 80, panel_w - 40, 50);
        float br = button_hover ? 0.3f : 0.2f;
        float bg_btn = button_hover ? 0.5f : 0.3f;
        float bb = button_hover ? 0.7f : 0.4f;
        DrawRect(panel_x + 20, height - 80, panel_w - 40, 50, br, bg_btn, bb);
        DrawText(panel_x + 60, height - 60, "APPLY GRAIN");
    }
    
    void DrawRect(float x, float y, float w, float h, float r, float g, float b) {
        glColor3f(r, g, b);
        glBegin(GL_QUADS);
        glVertex2f(x, y);
        glVertex2f(x + w, y);
        glVertex2f(x + w, y + h);
        glVertex2f(x, y + h);
        glEnd();
    }
    
    void DrawBorder(float x, float y, float w, float h, float r, float g, float b) {
        glColor3f(r, g, b);
        glLineWidth(2);
        glBegin(GL_LINE_LOOP);
        glVertex2f(x, y);
        glVertex2f(x + w, y);
        glVertex2f(x + w, y + h);
        glVertex2f(x, y + h);
        glEnd();
    }
    
    void DrawText(float x, float y, const std::string& text) {
        glColor3f(0.9f, 0.9f, 0.9f);
        for (size_t i = 0; i < text.length(); ++i) {
            float char_x = x + i * 8;
            glBegin(GL_QUADS);
            glVertex2f(char_x, y);
            glVertex2f(char_x + 6, y);
            glVertex2f(char_x + 6, y + 12);
            glVertex2f(char_x, y + 12);
            glEnd();
        }
    }
    
    void DrawImage(float x, float y, float w, float h) {
        int step = 3;
        for (int py = 0; py < h; py += step) {
            for (int px = 0; px < w; px += step) {
                int src_x = (px * grain.width) / w;
                int src_y = (py * grain.height) / h;
                const uint8_t* pixel = grain.GetPixelPtr(src_x, src_y);
                if (pixel) {
                    float r = pixel[0] / 255.0f;
                    float g = pixel[1] / 255.0f;
                    float b = pixel[2] / 255.0f;
                    DrawRect(x + px, y + py, step, step, r, g, b);
                }
            }
        }
    }
    
    void DrawSlider(float x, float y, float w, float value, float min_val, float max_val, const std::string& label) {
        DrawText(x, y - 20, label);
        DrawRect(x, y, w, 30, 0.2f, 0.2f, 0.2f);
        
        float fill_ratio = (value - min_val) / (max_val - min_val);
        float fill_w = fill_ratio * w;
        DrawRect(x, y, fill_w, 30, 0.4f, 0.6f, 0.8f);
        
        float handle_x = x + fill_w - 8;
        DrawRect(handle_x, y - 3, 16, 36, 0.9f, 0.9f, 0.9f);
        
        char val_str[32];
        snprintf(val_str, sizeof(val_str), "%.2f", value);
        DrawText(x + w + 10, y + 8, val_str);
    }
    
    void UpdateGrain() {
        grain.data = preview.data;
        for (size_t i = 0; i < grain.data.size(); i += 3) {
            float brightness = grain.data[i] / 255.0f;
            float noise = ((rand() % 2000) / 1000.0f - 1.0f);
            noise *= intensity * opacity * (0.2f + brightness * 0.8f);
            noise *= grain_size * 0.15f;
            
            for (int c = 0; c < 3; ++c) {
                int val = grain.data[i + c] + (int)(noise * 255 * density);
                grain.data[i + c] = std::clamp(val, 0, 255);
            }
        }
    }
};

// Global UI instance
static SimpleUI* g_ui = nullptr;

static bool mouse_pressed = false;

void mouse_button_callback(GLFWwindow* window, int button, int action, int mods) {
    if (button == GLFW_MOUSE_BUTTON_LEFT) {
        mouse_pressed = (action == GLFW_PRESS);
        double x, y;
        glfwGetCursorPos(window, &x, &y);
        if (g_ui) g_ui->HandleMouse(x, y, mouse_pressed);
    }
}

void cursor_position_callback(GLFWwindow* window, double xpos, double ypos) {
    if (g_ui) g_ui->HandleMouse(xpos, ypos, mouse_pressed);
}

void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GLFW_TRUE);
    }
}

int main() {
    std::cout << "🎬 Starting FilmGrain Pro..." << std::endl;
    
    // Initialize GLFW
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }
    
    // Create window
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 2);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    
    GLFWwindow* window = glfwCreateWindow(1280, 720, "FilmGrain Pro - Professional Film Grain Simulator", nullptr, nullptr);
    if (!window) {
        std::cerr << "Failed to create window" << std::endl;
        glfwTerminate();
        return -1;
    }
    
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1); // Enable vsync
    
    // Set callbacks
    glfwSetMouseButtonCallback(window, mouse_button_callback);
    glfwSetCursorPosCallback(window, cursor_position_callback);
    glfwSetKeyCallback(window, key_callback);
    
    // Initialize engine
    FilmGrainEngine engine;
    if (!engine.Initialize()) {
        std::cerr << "Failed to initialize engine" << std::endl;
        glfwTerminate();
        return -1;
    }
    
    // Initialize UI
    g_ui = new SimpleUI(&engine);
    g_ui->window = window;
    
    std::cout << "✅ FilmGrain Pro initialized successfully!" << std::endl;
    std::cout << "🎮 Controls:" << std::endl;
    std::cout << "   - Click dropdown to select film stocks" << std::endl;
    std::cout << "   - Drag sliders to adjust grain parameters" << std::endl;
    std::cout << "   - Click APPLY GRAIN to see effects" << std::endl;
    std::cout << "   - ESC to exit" << std::endl;
    
    // Main loop
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        
        // Clear screen
        glClearColor(0.067f, 0.067f, 0.067f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        
        // Render UI
        if (g_ui) g_ui->Render();
        
        glfwSwapBuffers(window);
    }
    
    // Cleanup
    delete g_ui;
    engine.Shutdown();
    glfwTerminate();
    
    std::cout << "👋 FilmGrain Pro closed" << std::endl;
    return 0;
}