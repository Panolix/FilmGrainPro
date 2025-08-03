cmake_minimum_required(VERSION 3.20)
project(FilmGrainPro VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find GLFW
find_package(glfw3 REQUIRED)

# macOS specific
if(APPLE)
    find_library(OPENGL_LIBRARY OpenGL)
    find_library(COCOA_LIBRARY Cocoa)
    find_library(IOKIT_LIBRARY IOKit)
    find_library(COREVIDEO_LIBRARY CoreVideo)
endif()

# Create executable
add_executable(FilmGrainPro src/main.cpp)

# Link libraries
target_link_libraries(FilmGrainPro glfw)

if(APPLE)
    target_link_libraries(FilmGrainPro 
        ${OPENGL_LIBRARY}
        ${COCOA_LIBRARY}
        ${IOKIT_LIBRARY}
        ${COREVIDEO_LIBRARY}
    )
endif()

# Compiler flags
target_compile_options(FilmGrainPro PRIVATE -Wall -Wextra)