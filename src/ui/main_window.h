#pragma once

#include <memory>
#include <string>

// Forward declarations
class FilmGrainEngine;
class FilmStockLibrary;
class PreviewCanvas;
class ControlPanel;
struct GLFWwindow;
struct Image;
struct GrainParams;

/**
 * Main application window using Dear ImGui
 * Provides the primary user interface for FilmGrain Pro
 */
class MainWindow {
public:
    explicit MainWindow(FilmGrainEngine* engine);
    ~MainWindow();
    
    // Window lifecycle
    bool Initialize();
    void Shutdown();
    
    // Main loop functions
    void Update();
    void Render();
    bool ShouldClose() const;
    
    // Window management
    void SetTitle(const std::string& title);
    void SetSize(int width, int height);
    void Maximize();
    void Minimize();
    
private:
    // Core components
    FilmGrainEngine* engine_;
    GLFWwindow* window_;
    
    // UI state
    int selected_film_stock_;
    bool film_stock_dropdown_open_;
    double mouse_x_, mouse_y_;
    bool mouse_pressed_;
    
    // Window state
    int window_width_;
    int window_height_;
    std::string window_title_;
    bool show_demo_window_;
    bool show_metrics_window_;
    
    // UI state
    bool show_film_library_;
    bool show_preview_;
    bool show_controls_;
    bool show_status_bar_;
    
    // Internal methods
    bool InitializeGLFW();
    void InitializeImGui();
    void SetupDocking();
    void RenderMenuBar();
    void RenderStatusBar();
    void RenderMainDockSpace();
    void RenderProfessionalUI();
    void RenderControlPanel();
    void RenderMainPreview();
    void RenderFilmStockSelector();
    void DrawRect(float x, float y, float w, float h, float r, float g, float b);
    void DrawText(float x, float y, const std::string& text, float r = 1.0f, float g = 1.0f, float b = 1.0f);
    void DrawSlider(float x, float y, float w, float h, float value, float min_val, float max_val, const std::string& label);
    void DrawButton(float x, float y, float w, float h, const std::string& text, bool pressed = false);
    bool IsMouseInRect(float x, float y, float w, float h);
    
    // Mouse interaction
    void HandleMouseClick(double x, double y);
    static void MouseButtonCallback(GLFWwindow* window, int button, int action, int mods);
    static void CursorPositionCallback(GLFWwindow* window, double xpos, double ypos);
    void HandleKeyboardShortcuts();
    void LoadUISettings();
    void SaveUISettings();
    
    // Grain testing
    void ApplyGrainToTestImage(Image& test_image, const std::string& film_stock, const GrainParams& params);
    void UpdateGrainParameters();
    void CyclePreviousFilmStock();
    void CycleNextFilmStock();
    void ShowGrainControls();
    
    // Event handlers
    static void WindowSizeCallback(GLFWwindow* window, int width, int height);
    static void KeyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
    static void DropCallback(GLFWwindow* window, int count, const char** paths);
};