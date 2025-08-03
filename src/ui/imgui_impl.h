#pragma once

#include <GLFW/glfw3.h>

namespace ImGuiImpl {
    bool Initialize(GLFWwindow* window);
    void Shutdown();
    void NewFrame();
    void Render();
}