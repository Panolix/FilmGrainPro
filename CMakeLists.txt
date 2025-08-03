cmake_minimum_required(VERSION 3.20)
project(FilmGrainPro VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Ensure proper C++ stdlib on macOS
if(APPLE)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++")
endif()

# Build configuration
set(CMAKE_BUILD_TYPE Release)

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# macOS specific settings
if(APPLE)
    set(CMAKE_OSX_DEPLOYMENT_TARGET "10.15")
    find_library(OPENGL_LIBRARY OpenGL)
    find_library(COCOA_LIBRARY Cocoa)
    find_library(IOKIT_LIBRARY IOKit)
    find_library(COREVIDEO_LIBRARY CoreVideo)
endif()

# Find packages
find_package(glfw3 REQUIRED)

# Include directories
include_directories(src)

# Source files
set(CORE_SOURCES
    src/core/engine.cpp
    src/core/film_stock_database.cpp
    src/core/color_processor.cpp
    src/core/pattern_library.cpp
    src/core/grain_renderer.cpp
)

set(UI_SOURCES
    src/ui/main_window.cpp
    src/ui/html_like_ui.cpp
)

set(UTILS_SOURCES
    src/utils/image.cpp
    src/utils/performance_monitor.cpp
)

set(ALL_SOURCES ${CORE_SOURCES} ${UI_SOURCES} ${UTILS_SOURCES})

# Create executable
add_executable(FilmGrainPro src/main.cpp ${ALL_SOURCES})

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
target_compile_options(FilmGrainPro PRIVATE 
    -Wall -Wextra -Wpedantic
    -Wno-unused-parameter
)

# Copy JSON data files to build directory
file(COPY ${CMAKE_SOURCE_DIR}/stocks5.json DESTINATION ${CMAKE_BINARY_DIR}/bin/)
file(COPY ${CMAKE_SOURCE_DIR}/advanced-shapeetc.json DESTINATION ${CMAKE_BINARY_DIR}/bin/)