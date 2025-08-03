#pragma once

#include <memory>
#include <string>

#include <memory>

// Forward declarations
class FilmGrainEngine;
class HTMLLikeUI;
struct GLFWwindow;

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
    
    // HTML-like UI
    std::unique_ptr<HTMLLikeUI> html_ui_;
    
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
    
    // Removed old grain testing methods
    
    // Event handlers
    static void WindowSizeCallback(GLFWwindow* window, int width, int height);
    static void KeyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
    static void DropCallback(GLFWwindow* window, int count, const char** paths);
};