#include "main_window.h"
#include "html_like_ui.h"
#include "../core/engine.h"
#include <iostream>

#ifdef __APPLE__
#define GL_SILENCE_DEPRECATION
#include <OpenGL/gl.h>
#endif

#include <GLFW/glfw3.h>

MainWindow::MainWindow(FilmGrainEngine* engine)
    : engine_(engine)
    , window_(nullptr)
    , window_width_(1280)
    , window_height_(720)
    , window_title_("FilmGrain Pro - Professional Film Grain Simulator")
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
    std::cout << "Initializing FilmGrain Pro..." << std::endl;
    
    if (!InitializeGLFW()) {
        return false;
    }
    
    // Initialize HTML-like UI
    html_ui_ = std::make_unique<HTMLLikeUI>(engine_);
    if (!html_ui_->Initialize(window_)) {
        std::cerr << "Failed to initialize HTML-like UI" << std::endl;
        return false;
    }
    
    std::cout << "FilmGrain Pro initialized successfully" << std::endl;
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
    // Clear with dark background
    glClearColor(0.067f, 0.067f, 0.067f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
    
    // Render HTML-like UI
    if (html_ui_) {
        html_ui_->Render();
    }
    
    glfwSwapBuffers(window_);
}

bool MainWindow::ShouldClose() const {
    return window_ ? glfwWindowShouldClose(window_) : true;
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
                std::cout << "🎛️ FilmGrain Pro - Professional Film Grain Simulator" << std::endl;
                break;
        }
    }
}

void MainWindow::MouseButtonCallback(GLFWwindow* window, int button, int action, int mods) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && main_window->html_ui_) {
        double x, y;
        glfwGetCursorPos(window, &x, &y);
        bool pressed = (action == GLFW_PRESS);
        main_window->html_ui_->HandleMouse(x, y, pressed);
    }
}

void MainWindow::CursorPositionCallback(GLFWwindow* window, double xpos, double ypos) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && main_window->html_ui_) {
        // Get current mouse button state
        bool pressed = (glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS);
        main_window->html_ui_->HandleMouse(xpos, ypos, pressed);
    }
}

void MainWindow::DropCallback(GLFWwindow* window, int count, const char** paths) {
    MainWindow* main_window = static_cast<MainWindow*>(glfwGetWindowUserPointer(window));
    if (main_window && count > 0) {
        std::cout << "Files dropped: " << paths[0] << std::endl;
    }
}