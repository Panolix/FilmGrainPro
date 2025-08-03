#pragma once

#include <memory>
#include <string>

// Forward declarations
class FilmGrainEngine;
class FilmStockLibrary;
class PreviewCanvas;
class ControlPanel;
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
    
    // UI panels
    std::unique_ptr<FilmStockLibrary> film_library_;
    std::unique_ptr<PreviewCanvas> preview_canvas_;
    std::unique_ptr<ControlPanel> control_panel_;
    
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
    void InitializeGLFW();
    void InitializeImGui();
    void SetupDocking();
    void RenderMenuBar();
    void RenderStatusBar();
    void RenderMainDockSpace();
    void HandleKeyboardShortcuts();
    void LoadUISettings();
    void SaveUISettings();
    
    // Event handlers
    static void WindowSizeCallback(GLFWwindow* window, int width, int height);
    static void KeyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
    static void DropCallback(GLFWwindow* window, int count, const char** paths);
};