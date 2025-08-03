#include "main_window.h"
#include "../core/engine.h"
#include "../utils/image.h"
#include <iostream>

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl3.h>
#endif

#include <GLFW/glfw3.h>

// ImGui includes would go here when available
// #include <imgui.h>
// #include <imgui_impl_glfw.h>
// #include <imgui_impl_opengl3.h>

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
    , show_status_bar_(true) {
}

MainWindow::~MainWindow() {
    Shutdown();
}

bool MainWindow::Initialize() {
    std::cout << "Initializing Main Window..." << std::endl;
    
    if (!InitializeGLFW()) {
        return false;
    }
    
    // InitializeImGui() would be called here when ImGui is available
    
    std::cout << "Main Window initialized successfully" << std::endl;
    return true;
}

bool MainWindow::InitializeGLFW() {
    if (!glfwInit()) {
        std::cerr << "Failed to initialize GLFW" << std::endl;
        return false;
    }
    
    // Set GLFW window hints
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    
#ifdef __APPLE__
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
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
    // Clear the screen with a darker background
    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
    
    // Render test pattern and grain effects
    RenderTestPattern();
    RenderGrainControls();
    RenderStatusBar();
    
    glfwSwapBuffers(window_);
}

bool MainWindow::ShouldClose() const {
    return window_ ? glfwWindowShouldClose(window_) : true;
}

void MainWindow::RenderMenuBar() {
    // Simple text-based menu for now
    static bool show_info = true;
    if (show_info) {
        std::cout << "=== FilmGrain Pro - Grain Test Interface ===" << std::endl;
        std::cout << "Controls:" << std::endl;
        std::cout << "  ESC: Exit | Space: Apply grain | F1: Show controls" << std::endl;
        std::cout << "  1/2: Intensity ↓/↑ | 3/4: Opacity ↓/↑" << std::endl;
        std::cout << "  5/6: Size ↓/↑ | 7/8: Density ↓/↑" << std::endl;
        std::cout << "  9/0: Previous/Next film stock" << std::endl;
        std::cout << "========================================" << std::endl;
        show_info = false;
    }
}

void MainWindow::RenderMainDockSpace() {
    // Create a test pattern and apply grain effects
    static bool first_run = true;
    static Image test_image;
    static GrainParams grain_params;
    static std::string current_film_stock = "Kodak Tri-X 400";
    
    if (first_run) {
        // Create a test gradient image
        test_image.Allocate(800, 600, 3);
        for (int y = 0; y < test_image.height; ++y) {
            for (int x = 0; x < test_image.width; ++x) {
                uint8_t* pixel = test_image.GetPixelPtr(x, y);
                if (pixel) {
                    // Create a gradient pattern
                    uint8_t gray = static_cast<uint8_t>((x * 255) / test_image.width);
                    pixel[0] = gray;
                    pixel[1] = gray;
                    pixel[2] = gray;
                }
            }
        }
        
        // Set default grain parameters
        grain_params.intensity = 1.0f;
        grain_params.opacity = 0.6f;
        grain_params.size_multiplier = 1.0f;
        grain_params.density_multiplier = 1.0f;
        
        first_run = false;
        std::cout << "Created test pattern (800x600)" << std::endl;
    }
    
    // Apply grain effect when space is pressed
    if (glfwGetKey(window_, GLFW_KEY_SPACE) == GLFW_PRESS) {
        static bool space_pressed = false;
        if (!space_pressed) {
            ApplyGrainToTestImage(test_image, current_film_stock, grain_params);
            space_pressed = true;
        }
    } else {
        static bool space_pressed = false;
        space_pressed = false;
    }
}

void MainWindow::RenderStatusBar() {
    // Placeholder for status bar
    if (show_status_bar_) {
        auto stats = engine_->GetPerformanceStats();
        std::cout << "FPS: " << stats.fps << " Memory: " << stats.memory_usage_mb << "MB" << std::endl;
    }
}

void MainWindow::HandleKeyboardShortcuts() {
    static GrainParams* current_params = nullptr;
    static std::string* current_stock = nullptr;
    static Image* test_image = nullptr;
    
    // Get references to static variables from RenderMainDockSpace
    // This is a simple way to access them - in a real app we'd use member variables
    
    // Handle keyboard shortcuts
    if (glfwGetKey(window_, GLFW_KEY_ESCAPE) == GLFW_PRESS) {
        glfwSetWindowShouldClose(window_, true);
    }
    
    // Grain parameter controls
    static bool key_pressed[10] = {false}; // Track key states to prevent rapid firing
    
    // Intensity controls (1/2 keys)
    if (glfwGetKey(window_, GLFW_KEY_1) == GLFW_PRESS && !key_pressed[0]) {
        std::cout << "🔽 Decreasing intensity..." << std::endl;
        key_pressed[0] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_1) == GLFW_RELEASE) {
        key_pressed[0] = false;
    }
    
    if (glfwGetKey(window_, GLFW_KEY_2) == GLFW_PRESS && !key_pressed[1]) {
        std::cout << "🔼 Increasing intensity..." << std::endl;
        key_pressed[1] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_2) == GLFW_RELEASE) {
        key_pressed[1] = false;
    }
    
    // Opacity controls (3/4 keys)
    if (glfwGetKey(window_, GLFW_KEY_3) == GLFW_PRESS && !key_pressed[2]) {
        std::cout << "🔽 Decreasing opacity..." << std::endl;
        key_pressed[2] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_3) == GLFW_RELEASE) {
        key_pressed[2] = false;
    }
    
    if (glfwGetKey(window_, GLFW_KEY_4) == GLFW_PRESS && !key_pressed[3]) {
        std::cout << "🔼 Increasing opacity..." << std::endl;
        key_pressed[3] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_4) == GLFW_RELEASE) {
        key_pressed[3] = false;
    }
    
    // Size controls (5/6 keys)
    if (glfwGetKey(window_, GLFW_KEY_5) == GLFW_PRESS && !key_pressed[4]) {
        std::cout << "🔽 Decreasing grain size..." << std::endl;
        key_pressed[4] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_5) == GLFW_RELEASE) {
        key_pressed[4] = false;
    }
    
    if (glfwGetKey(window_, GLFW_KEY_6) == GLFW_PRESS && !key_pressed[5]) {
        std::cout << "🔼 Increasing grain size..." << std::endl;
        key_pressed[5] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_6) == GLFW_RELEASE) {
        key_pressed[5] = false;
    }
    
    // Density controls (7/8 keys)
    if (glfwGetKey(window_, GLFW_KEY_7) == GLFW_PRESS && !key_pressed[6]) {
        std::cout << "🔽 Decreasing grain density..." << std::endl;
        key_pressed[6] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_7) == GLFW_RELEASE) {
        key_pressed[6] = false;
    }
    
    if (glfwGetKey(window_, GLFW_KEY_8) == GLFW_PRESS && !key_pressed[7]) {
        std::cout << "🔼 Increasing grain density..." << std::endl;
        key_pressed[7] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_8) == GLFW_RELEASE) {
        key_pressed[7] = false;
    }
    
    // Film stock cycling (9/0 keys)
    if (glfwGetKey(window_, GLFW_KEY_9) == GLFW_PRESS && !key_pressed[8]) {
        std::cout << "⬅️ Previous film stock..." << std::endl;
        CyclePreviousFilmStock();
        key_pressed[8] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_9) == GLFW_RELEASE) {
        key_pressed[8] = false;
    }
    
    if (glfwGetKey(window_, GLFW_KEY_0) == GLFW_PRESS && !key_pressed[9]) {
        std::cout << "➡️ Next film stock..." << std::endl;
        CycleNextFilmStock();
        key_pressed[9] = true;
    } else if (glfwGetKey(window_, GLFW_KEY_0) == GLFW_RELEASE) {
        key_pressed[9] = false;
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
        // Handle key presses
        switch (key) {
            case GLFW_KEY_F1:
                main_window->ShowGrainControls();
                break;
            case GLFW_KEY_F11:
                // Toggle fullscreen (placeholder)
                break;
        }
    }
}

void MainWindow::ApplyGrainToTestImage(Image& test_image, const std::string& film_stock, const GrainParams& params) {
    std::cout << "Applying " << film_stock << " grain..." << std::endl;
    std::cout << "  Intensity: " << params.intensity << std::endl;
    std::cout << "  Opacity: " << params.opacity << std::endl;
    std::cout << "  Size: " << params.size_multiplier << std::endl;
    std::cout << "  Density: " << params.density_multiplier << std::endl;
    
    // Get the film stock
    const FilmStock* stock = engine_->GetFilmStock(film_stock);
    if (stock) {
        // Create output image
        Image output_image;
        engine_->ProcessImage(test_image, output_image, film_stock, params);
        std::cout << "✅ Grain applied successfully!" << std::endl;
    } else {
        std::cout << "❌ Film stock not found: " << film_stock << std::endl;
        
        // List available stocks
        auto stocks = engine_->GetAvailableFilmStocks();
        std::cout << "Available stocks (" << stocks.size() << "):" << std::endl;
        for (size_t i = 0; i < std::min(size_t(5), stocks.size()); ++i) {
            std::cout << "  - " << stocks[i].display_name << std::endl;
        }
    }
}

void MainWindow::UpdateGrainParameters() {
    // This will be expanded with actual sliders later
    std::cout << "Grain parameters updated" << std::endl;
}

void MainWindow::CyclePreviousFilmStock() {
    auto stocks = engine_->GetAvailableFilmStocks();
    if (!stocks.empty()) {
        static size_t current_index = 0;
        if (current_index == 0) {
            current_index = stocks.size() - 1;
        } else {
            current_index--;
        }
        std::cout << "📽️ Selected: " << stocks[current_index].display_name << std::endl;
    }
}

void MainWindow::CycleNextFilmStock() {
    auto stocks = engine_->GetAvailableFilmStocks();
    if (!stocks.empty()) {
        static size_t current_index = 0;
        current_index = (current_index + 1) % stocks.size();
        std::cout << "📽️ Selected: " << stocks[current_index].display_name << std::endl;
    }
}

void MainWindow::RenderTestPattern() {
    // Simple OpenGL rendering of test pattern
    static bool pattern_created = false;
    static float pattern_data[800 * 600 * 3];
    
    if (!pattern_created) {
        // Create a gradient test pattern
        for (int y = 0; y < 600; ++y) {
            for (int x = 0; x < 800; ++x) {
                int idx = (y * 800 + x) * 3;
                float gray = (float)x / 800.0f;
                pattern_data[idx] = gray;     // R
                pattern_data[idx + 1] = gray; // G  
                pattern_data[idx + 2] = gray; // B
            }
        }
        pattern_created = true;
    }
    
    // Render the pattern using OpenGL
    glRasterPos2f(-1.0f, -1.0f);
    glDrawPixels(800, 600, GL_RGB, GL_FLOAT, pattern_data);
}

void MainWindow::RenderGrainControls() {
    // Display current grain parameters as text overlay
    static float intensity = 1.0f;
    static float opacity = 0.6f;
    static float size = 1.0f;
    static float density = 1.0f;
    static int current_stock = 0;
    
    // Update parameters based on key presses
    if (glfwGetKey(window_, GLFW_KEY_1) == GLFW_PRESS) intensity = std::max(0.0f, intensity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_2) == GLFW_PRESS) intensity = std::min(3.0f, intensity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_3) == GLFW_PRESS) opacity = std::max(0.0f, opacity - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_4) == GLFW_PRESS) opacity = std::min(1.0f, opacity + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_5) == GLFW_PRESS) size = std::max(0.1f, size - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_6) == GLFW_PRESS) size = std::min(5.0f, size + 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_7) == GLFW_PRESS) density = std::max(0.1f, density - 0.01f);
    if (glfwGetKey(window_, GLFW_KEY_8) == GLFW_PRESS) density = std::min(3.0f, density + 0.01f);
    
    // Simple text rendering using OpenGL (basic approach)
    glColor3f(1.0f, 1.0f, 1.0f); // White text
    
    // For now, just output to console with current values
    static int frame_count = 0;
    if (++frame_count % 60 == 0) { // Update every 60 frames
        std::cout << "\r🎛️ Intensity:" << intensity << " Opacity:" << opacity 
                  << " Size:" << size << " Density:" << density << "    " << std::flush;
    }
}

void MainWindow::ShowGrainControls() {
    std::cout << "\n🎛️ Current Grain Settings:" << std::endl;
    std::cout << "  Intensity: [1/2 keys] | Opacity: [3/4 keys]" << std::endl;
    std::cout << "  Size: [5/6 keys] | Density: [7/8 keys]" << std::endl;
    std::cout << "  Film Stock: [9/0 keys] | Apply: [Space]" << std::endl;
    std::cout << "========================================\n" << std::endl;
}

void MainWindow::DropCallback(GLFWwindow* window, int count, const char** paths) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && count > 0) {
        std::cout << "Files dropped: " << paths[0] << std::endl;
        // Handle file drops - load images
    }
}