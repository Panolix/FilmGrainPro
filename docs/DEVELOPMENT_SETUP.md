# FilmGrain Pro - Development Setup Guide

## 🛠️ Prerequisites

### Required Software
- **C++ Compiler**: GCC 10+, Clang 12+, or MSVC 2019+
- **CMake**: Version 3.20 or higher
- **Git**: For version control
- **Python 3.8+**: For build scripts and tools

### Required Libraries
- **Vulkan SDK**: Latest version from LunarG
- **OpenGL**: System OpenGL (usually pre-installed)
- **GLFW**: Window management
- **OpenCV**: Image processing
- **Dear ImGui**: UI framework (included as submodule)
- **GLM**: Math library (included as submodule)

## 🚀 Quick Start

### 1. Clone Repository
```bash
git clone https://github.com/yourorg/FilmGrainPro.git
cd FilmGrainPro
git submodule update --init --recursive
```

### 2. Install Dependencies

#### Windows (vcpkg)
```bash
# Install vcpkg
git clone https://github.com/Microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat

# Install dependencies
.\vcpkg install vulkan glfw3 opencv[core,imgproc] --triplet x64-windows
```

#### macOS (Homebrew)
```bash
# Install Homebrew dependencies
brew install cmake glfw opencv vulkan-headers vulkan-loader

# Install Vulkan SDK manually from LunarG
# Download from: https://vulkan.lunarg.com/sdk/home
```

#### Linux (Ubuntu/Debian)
```bash
# Install system dependencies
sudo apt update
sudo apt install build-essential cmake git

# Install graphics libraries
sudo apt install libvulkan-dev vulkan-tools vulkan-validationlayers
sudo apt install libglfw3-dev libgl1-mesa-dev

# Install OpenCV
sudo apt install libopencv-dev
```

### 3. Build Project
```bash
# Create build directory
mkdir build && cd build

# Configure with CMake
cmake .. -DCMAKE_BUILD_TYPE=Debug

# Build (use -j for parallel compilation)
cmake --build . -j$(nproc)
```

### 4. Run Application
```bash
# From build directory
./bin/FilmGrainPro

# Or with debug information
gdb ./bin/FilmGrainPro
```

## 🏗️ Project Structure

```
FilmGrainPro/
├── src/                    # Source code
│   ├── core/              # Core engine
│   ├── ui/                # User interface
│   ├── plugins/           # Plugin system
│   └── utils/             # Utilities
├── assets/                # Resources
│   ├── patterns/          # Grain patterns
│   ├── presets/           # Film stock presets
│   ├── shaders/           # GPU shaders
│   └── ui/                # UI resources
├── tests/                 # Test suites
├── tools/                 # Development tools
├── docs/                  # Documentation
└── third_party/           # External libraries
```

## 🔧 Development Workflow

### Code Style
- **Standard**: C++20 with modern features
- **Formatting**: clang-format with provided .clang-format
- **Naming**: snake_case for variables, PascalCase for classes
- **Documentation**: Doxygen-style comments

### Git Workflow
```bash
# Create feature branch
git checkout -b feature/grain-renderer-optimization

# Make changes and commit
git add .
git commit -m "Optimize grain rendering performance"

# Push and create pull request
git push origin feature/grain-renderer-optimization
```

### Testing
```bash
# Run unit tests
cd build
ctest --verbose

# Run specific test
./tests/unit/test_grain_renderer

# Run performance benchmarks
./tests/performance/benchmark_grain_processing
```

### Debugging
```bash
# Debug build with symbols
cmake .. -DCMAKE_BUILD_TYPE=Debug -DENABLE_ASAN=ON

# Run with AddressSanitizer
./bin/FilmGrainPro

# Profile with Valgrind (Linux)
valgrind --tool=callgrind ./bin/FilmGrainPro
```

## 🎨 UI Development

### ImGui Integration
- UI code is in `src/ui/` directory
- Use immediate mode paradigm
- Custom widgets in `src/ui/widgets/`
- Styling in `assets/ui/style.json`

### Adding New Panels
```cpp
// 1. Create header file in src/ui/
class NewPanel {
public:
    void Render();
private:
    // Panel state
};

// 2. Add to MainWindow
std::unique_ptr<NewPanel> new_panel_;

// 3. Render in main loop
if (ImGui::Begin("New Panel")) {
    new_panel_->Render();
}
ImGui::End();
```

## 🔌 Plugin Development

### Creating a Plugin
```cpp
// Implement IFilmGrainPlugin interface
class MyPlugin : public IFilmGrainPlugin {
public:
    std::string GetName() const override { return "My Plugin"; }
    bool Initialize(IPluginHost* host) override;
    void OnPostProcess(Image& output) override;
};

// Export plugin
extern "C" {
    IFilmGrainPlugin* CreatePlugin() {
        return new MyPlugin();
    }
}
```

### Plugin Build System
```cmake
# Add to plugins/CMakeLists.txt
add_library(my_plugin SHARED my_plugin.cpp)
target_link_libraries(my_plugin FilmGrainCore)
```

## 📊 Performance Optimization

### Profiling Tools
- **Windows**: Visual Studio Profiler, Intel VTune
- **macOS**: Instruments, Xcode Profiler
- **Linux**: perf, Valgrind, gprof

### GPU Debugging
- **RenderDoc**: Graphics debugging
- **Nsight Graphics**: NVIDIA GPU profiling
- **Vulkan Validation Layers**: API validation

### Memory Management
```cpp
// Use custom memory pools
auto image = MemoryManager::Instance().AllocateImage(width, height);

// RAII for GPU resources
class VulkanBuffer {
    ~VulkanBuffer() { vkDestroyBuffer(device, buffer, nullptr); }
};
```

## 🐛 Common Issues

### Build Issues
1. **Vulkan not found**: Install Vulkan SDK and set VULKAN_SDK environment variable
2. **OpenCV linking errors**: Ensure OpenCV is built with same compiler
3. **ImGui compilation errors**: Update submodules with `git submodule update`

### Runtime Issues
1. **GPU driver crashes**: Update graphics drivers
2. **Memory leaks**: Run with AddressSanitizer enabled
3. **Performance issues**: Check GPU utilization and memory usage

### Platform-Specific
- **Windows**: Use Visual Studio 2019+ for best debugging experience
- **macOS**: May need to disable Metal validation for performance
- **Linux**: Install proprietary GPU drivers for best performance

## 📚 Additional Resources

- [Vulkan Tutorial](https://vulkan-tutorial.com/)
- [Dear ImGui Documentation](https://github.com/ocornut/imgui)
- [OpenCV Documentation](https://docs.opencv.org/)
- [CMake Best Practices](https://cliutils.gitlab.io/modern-cmake/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

For questions or issues, please check the existing issues or create a new one.