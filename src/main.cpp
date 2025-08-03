#include <GLFW/glfw3.h>
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <cmath>

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#endif

// Film grain app that actually works like the HTML version
class FilmGrainApp {
public:
    GLFWwindow* window;
    int width = 1280, height = 720;
    
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
    std::vector<std::string> films = {
        "Kodak Tri-X 400",
        "Ilford HP5 Plus", 
        "Kodak T-Max 400",
        "Fuji Acros 100",
        "Kodak Portra 400",
        "Ilford Delta 400",
        "Kodak Ektar 100",
        "Fuji Velvia 50"
    };
    
    // Test image data
    std::vector<uint8_t> original_image;
    std::vector<uint8_t> grain_image;
    int img_width = 800, img_height = 600;
    
    FilmGrainApp() {
        // Create gradient test image
        original_image.resize(img_width * img_height * 3);
        grain_image.resize(img_width * img_height * 3);
        
        for (int y = 0; y < img_height; ++y) {
            for (int x = 0; x < img_width; ++x) {
                int idx = (y * img_width + x) * 3;
                uint8_t gray = (x * 255) / img_width;
                original_image[idx] = original_image[idx + 1] = original_image[idx + 2] = gray;
            }
        }
        grain_image = original_image;
        
        std::cout << "FilmGrainApp created with " << films.size() << " film stocks" << std::endl;
    }
    
    void HandleMouse(double x, double y, bool pressed) {
        mouse_was_down = mouse_down;
        mx = x; my = y; mouse_down = pressed;
        
        // Calculate layout
        float preview_w = width * 0.75f;
        float panel_x = preview_w;
        float panel_w = width - preview_w;
        
        // Dropdown area
        float dropdown_x = panel_x + 20;
        float dropdown_y = 100;
        float dropdown_w = panel_w - 40;
        float dropdown_h = 40;
        
        // Handle dropdown click
        if (InRect(mx, my, dropdown_x, dropdown_y, dropdown_w, dropdown_h)) {
            if (pressed && !mouse_was_down) {
                dropdown_open = !dropdown_open;
                std::cout << "Dropdown " << (dropdown_open ? "opened" : "closed") << std::endl;
            }
        }
        
        // Handle dropdown items
        if (dropdown_open) {
            for (int i = 0; i < std::min(6, (int)films.size()); ++i) {
                float item_y = dropdown_y + dropdown_h + i * 35;
                if (InRect(mx, my, dropdown_x, item_y, dropdown_w, 35)) {
                    if (pressed && !mouse_was_down) {
                        selected_film = i;
                        dropdown_open = false;
                        ApplyGrain();
                        std::cout << "Selected: " << films[i] << std::endl;
                    }
                }
            }
        }
        
        // Handle sliders
        float slider_x = panel_x + 20;
        float slider_w = panel_w - 40;
        
        HandleSlider(slider_x, 200, slider_w, &intensity, 0.0f, 3.0f, "Intensity");
        HandleSlider(slider_x, 280, slider_w, &opacity, 0.0f, 1.0f, "Opacity");
        HandleSlider(slider_x, 360, slider_w, &grain_size, 0.1f, 5.0f, "Grain Size");
        HandleSlider(slider_x, 440, slider_w, &density, 0.1f, 3.0f, "Density");
        
        // Apply button
        if (InRect(mx, my, slider_x, height - 80, slider_w, 50)) {
            if (pressed && !mouse_was_down) {
                ApplyGrain();
                std::cout << "🎬 APPLY GRAIN: " << films[selected_film] 
                          << " I:" << intensity << " O:" << opacity 
                          << " S:" << grain_size << " D:" << density << std::endl;
            }
        }
        
        // Close dropdown if clicking outside
        if (pressed && !mouse_was_down && dropdown_open) {
            if (!InRect(mx, my, dropdown_x, dropdown_y, dropdown_w, 250)) {
                dropdown_open = false;
            }
        }
    }
    
    void HandleSlider(float x, float y, float w, float* value, float min_val, float max_val, const std::string& name) {
        if (InRect(mx, my, x, y, w, 30) && mouse_down) {
            float ratio = std::clamp((float)(mx - x) / w, 0.0f, 1.0f);
            float new_val = min_val + ratio * (max_val - min_val);
            if (std::abs(new_val - *value) > 0.01f) {
                *value = new_val;
                ApplyGrain();
                std::cout << name << ": " << *value << std::endl;
            }
        }
    }
    
    bool InRect(double px, double py, float x, float y, float w, float h) {
        return px >= x && px <= x + w && py >= y && py <= y + h;
    }
    
    void ApplyGrain() {
        // Reset to original
        grain_image = original_image;
        
        // Apply grain effect
        for (int i = 0; i < img_width * img_height * 3; i += 3) {
            float brightness = grain_image[i] / 255.0f;
            
            // Generate grain noise
            float noise = ((rand() % 2000) / 1000.0f - 1.0f);
            noise *= intensity * opacity * (0.2f + brightness * 0.8f);
            noise *= grain_size * 0.15f;
            
            // Apply to all channels
            for (int c = 0; c < 3; ++c) {
                int val = grain_image[i + c] + (int)(noise * 255 * density);
                grain_image[i + c] = std::clamp(val, 0, 255);
            }
        }
    }
    
    void Render() {
        glfwGetWindowSize(window, &width, &height);
        
        // Setup OpenGL
        glViewport(0, 0, width, height);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glOrtho(0, width, height, 0, -1, 1); // Top-left origin like HTML
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        
        glDisable(GL_DEPTH_TEST);
        glEnable(GL_BLEND);
        glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
        
        // Dark background like HTML
        glClearColor(0.067f, 0.067f, 0.067f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        
        // Calculate layout
        float preview_w = width * 0.75f;
        float panel_x = preview_w;
        float panel_w = width - preview_w;
        
        // Preview panel (left 75%)
        DrawRect(0, 0, preview_w, height, 0.04f, 0.04f, 0.04f);
        
        // Draw image in center
        float display_w = 600, display_h = 450;
        float img_x = (preview_w - display_w) / 2;
        float img_y = (height - display_h) / 2;
        DrawImage(img_x, img_y, display_w, display_h);
        
        // White border around image
        glColor3f(1.0f, 1.0f, 1.0f);
        glLineWidth(2);
        glBegin(GL_LINE_LOOP);
        glVertex2f(img_x - 2, img_y - 2);
        glVertex2f(img_x + display_w + 2, img_y - 2);
        glVertex2f(img_x + display_w + 2, img_y + display_h + 2);
        glVertex2f(img_x - 2, img_y + display_h + 2);
        glEnd();
        
        // Control panel (right 25%)
        DrawRect(panel_x, 0, panel_w, height, 0.08f, 0.08f, 0.08f);
        
        // Left border
        DrawRect(panel_x, 0, 2, height, 0.3f, 0.3f, 0.3f);
        
        // Title
        DrawText(panel_x + 20, 30, "FILM GRAIN CONTROLS", 1.5f);
        
        // Film stock dropdown
        DrawText(panel_x + 20, 80, "Film Stock", 1.0f);
        
        float dropdown_x = panel_x + 20;
        float dropdown_y = 100;
        float dropdown_w = panel_w - 40;
        float dropdown_h = 40;
        
        bool dropdown_hover = InRect(mx, my, dropdown_x, dropdown_y, dropdown_w, dropdown_h);
        float bg_color = dropdown_hover ? 0.2f : 0.15f;
        
        DrawRect(dropdown_x, dropdown_y, dropdown_w, dropdown_h, bg_color, bg_color, bg_color);
        
        // Dropdown border
        glColor3f(0.4f, 0.4f, 0.4f);
        glLineWidth(2);
        glBegin(GL_LINE_LOOP);
        glVertex2f(dropdown_x, dropdown_y);
        glVertex2f(dropdown_x + dropdown_w, dropdown_y);
        glVertex2f(dropdown_x + dropdown_w, dropdown_y + dropdown_h);
        glVertex2f(dropdown_x, dropdown_y + dropdown_h);
        glEnd();
        
        // Current selection
        DrawText(dropdown_x + 10, dropdown_y + 12, films[selected_film], 1.0f);
        DrawText(dropdown_x + dropdown_w - 25, dropdown_y + 12, "v", 1.0f);
        
        // Dropdown list
        if (dropdown_open) {
            for (int i = 0; i < std::min(6, (int)films.size()); ++i) {
                float item_y = dropdown_y + dropdown_h + i * 35;
                bool item_hover = InRect(mx, my, dropdown_x, item_y, dropdown_w, 35);
                bool is_selected = (i == selected_film);
                
                float r = is_selected ? 0.3f : (item_hover ? 0.2f : 0.12f);
                float g = is_selected ? 0.4f : (item_hover ? 0.2f : 0.12f);
                float b = is_selected ? 0.5f : (item_hover ? 0.2f : 0.12f);
                
                DrawRect(dropdown_x, item_y, dropdown_w, 35, r, g, b);
                DrawText(dropdown_x + 10, item_y + 10, films[i], 1.0f);
            }
        }
        
        // Sliders
        DrawSlider(panel_x + 20, 200, panel_w - 40, intensity, 0.0f, 3.0f, "Intensity");
        DrawSlider(panel_x + 20, 280, panel_w - 40, opacity, 0.0f, 1.0f, "Opacity");
        DrawSlider(panel_x + 20, 360, panel_w - 40, grain_size, 0.1f, 5.0f, "Grain Size");
        DrawSlider(panel_x + 20, 440, panel_w - 40, density, 0.1f, 3.0f, "Density");
        
        // Apply button
        float button_x = panel_x + 20;
        float button_y = height - 80;
        float button_w = panel_w - 40;
        float button_h = 50;
        
        bool button_hover = InRect(mx, my, button_x, button_y, button_w, button_h);
        float br = button_hover ? 0.3f : 0.2f;
        float bg = button_hover ? 0.5f : 0.3f;
        float bb = button_hover ? 0.7f : 0.4f;
        
        DrawRect(button_x, button_y, button_w, button_h, br, bg, bb);
        DrawText(button_x + button_w/2 - 50, button_y + 18, "APPLY GRAIN", 1.2f);
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
    
    void DrawText(float x, float y, const std::string& text, float scale = 1.0f) {
        glColor3f(0.9f, 0.9f, 0.9f);
        float char_w = 8 * scale;
        float char_h = 12 * scale;
        
        for (size_t i = 0; i < text.length(); ++i) {
            float char_x = x + i * char_w;
            glBegin(GL_QUADS);
            glVertex2f(char_x, y);
            glVertex2f(char_x + char_w * 0.7f, y);
            glVertex2f(char_x + char_w * 0.7f, y + char_h);
            glVertex2f(char_x, y + char_h);
            glEnd();
        }
    }
    
    void DrawImage(float x, float y, float w, float h) {
        int step = 2; // Pixel size for performance
        
        for (int py = 0; py < h; py += step) {
            for (int px = 0; px < w; px += step) {
                int src_x = (px * img_width) / w;
                int src_y = (py * img_height) / h;
                int idx = (src_y * img_width + src_x) * 3;
                
                if (idx < (int)grain_image.size() - 2) {
                    float r = grain_image[idx] / 255.0f;
                    float g = grain_image[idx + 1] / 255.0f;
                    float b = grain_image[idx + 2] / 255.0f;
                    DrawRect(x + px, y + py, step, step, r, g, b);
                }
            }
        }
    }
    
    void DrawSlider(float x, float y, float w, float value, float min_val, float max_val, const std::string& label) {
        // Label
        DrawText(x, y - 25, label, 1.0f);
        
        // Track
        DrawRect(x, y, w, 30, 0.2f, 0.2f, 0.2f);
        
        // Fill
        float fill_ratio = (value - min_val) / (max_val - min_val);
        float fill_w = fill_ratio * w;
        DrawRect(x, y, fill_w, 30, 0.4f, 0.6f, 0.8f);
        
        // Handle
        float handle_x = x + fill_w - 8;
        DrawRect(handle_x, y - 3, 16, 36, 0.9f, 0.9f, 0.9f);
        
        // Value
        char val_str[32];
        snprintf(val_str, sizeof(val_str), "%.2f", value);
        DrawText(x + w + 10, y + 8, val_str, 0.8f);
    }
};

// Global app instance
FilmGrainApp* g_app = nullptr;

void mouse_callback(GLFWwindow* window, int button, int action, int mods) {
    if (button == GLFW_MOUSE_BUTTON_LEFT && g_app) {
        double x, y;
        glfwGetCursorPos(window, &x, &y);
        g_app->HandleMouse(x, y, action == GLFW_PRESS);
    }
}

void cursor_callback(GLFWwindow* window, double x, double y) {
    if (g_app) {
        bool pressed = (glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS);
        g_app->HandleMouse(x, y, pressed);
    }
}

void key_callback(GLFWwindow* window, int key, int scancode, int action, int mods) {
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS) {
        glfwSetWindowShouldClose(window, GLFW_TRUE);
    }
}

int main() {
    std::cout << "🎬 Starting FilmGrain Pro..." << std::endl;
    
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return -1;
    }
    
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 2);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
    
    GLFWwindow* window = glfwCreateWindow(1280, 720, "FilmGrain Pro - Professional Film Grain Simulator", nullptr, nullptr);
    if (!window) {
        std::cerr << "Failed to create window" << std::endl;
        glfwTerminate();
        return -1;
    }
    
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);
    
    glfwSetMouseButtonCallback(window, mouse_callback);
    glfwSetCursorPosCallback(window, cursor_callback);
    glfwSetKeyCallback(window, key_callback);
    
    g_app = new FilmGrainApp();
    g_app->window = window;
    g_app->ApplyGrain(); // Initial grain application
    
    std::cout << "✅ FilmGrain Pro ready!" << std::endl;
    std::cout << "🎮 Click dropdown to select film stocks" << std::endl;
    std::cout << "🎛️ Drag sliders to adjust grain parameters" << std::endl;
    std::cout << "🎬 Click APPLY GRAIN to see effects" << std::endl;
    
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
        g_app->Render();
        glfwSwapBuffers(window);
    }
    
    delete g_app;
    glfwTerminate();
    return 0;
}