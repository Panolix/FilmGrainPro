cmake_minimum_required(VERSION 3.20)
project(FilmGrainPro VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Build configuration
set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type")
set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release" "RelWithDebInfo")

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Find packages
find_package(Vulkan REQUIRED)
find_package(OpenGL REQUIRED)
find_package(glfw3 REQUIRED)
find_package(OpenCV REQUIRED)

# Include directories
include_directories(src)
include_directories(third_party/imgui)
include_directories(third_party/glm)

# Source files
file(GLOB_RECURSE SOURCES 
    "src/core/*.cpp"
    "src/ui/*.cpp" 
    "src/plugins/*.cpp"
    "src/utils/*.cpp"
    "third_party/imgui/*.cpp"
)

# Create executable
add_executable(FilmGrainPro src/main.cpp ${SOURCES})

# Link libraries
target_link_libraries(FilmGrainPro 
    Vulkan::Vulkan
    OpenGL::GL
    glfw
    ${OpenCV_LIBS}
)

# Compiler flags
if(MSVC)
    target_compile_options(FilmGrainPro PRIVATE /W4)
else()
    target_compile_options(FilmGrainPro PRIVATE -Wall -Wextra -Wpedantic)
endif()

# Copy assets to build directory
file(COPY assets DESTINATION ${CMAKE_BINARY_DIR})

# Install targets
install(TARGETS FilmGrainPro DESTINATION bin)
install(DIRECTORY assets DESTINATION share/filmgrainpro)