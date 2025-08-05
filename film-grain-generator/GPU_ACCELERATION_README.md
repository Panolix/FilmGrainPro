# GPU Acceleration for Film Grain Generator

## Overview

This implementation adds GPU acceleration support for the Film Grain Generator with multi-platform compatibility:

- **🍎 Apple Silicon (M1/M2/M3)**: Metal backend for optimal performance
- **🟢 NVIDIA GPUs**: CUDA backend (Windows/Linux)
- **🔴 AMD GPUs**: OpenCL backend (Linux)
- **🌐 Universal Fallback**: WebGPU backend (all platforms)
- **💻 CPU Fallback**: Original Rayon-based parallel CPU rendering

## Architecture

### Backend Priority Order
1. **Metal** (macOS) - Native Apple Silicon GPU acceleration
2. **CUDA** (Windows/Linux) - NVIDIA GPU acceleration
3. **OpenCL** (Linux) - AMD GPU acceleration  
4. **WebGPU** (All platforms) - Cross-platform GPU acceleration
5. **CPU** (Fallback) - Multi-threaded CPU rendering

### Key Components

#### 1. GPU Manager (`src/gpu/mod.rs`)
- Automatically detects and initializes the best available GPU backend
- Provides unified interface for grain rendering
- Handles graceful fallback to CPU if GPU fails

#### 2. WebGPU Backend (`src/gpu/wgpu_backend.rs`)
- Cross-platform GPU acceleration using WebGPU/wgpu
- Compute shader-based grain rendering
- Works on all modern GPUs (NVIDIA, AMD, Intel, Apple)

#### 3. Metal Backend (`src/gpu/metal_backend.rs`)
- Native Metal implementation for Apple Silicon
- Optimized for M1/M2/M3 chip architecture
- Direct GPU memory management for maximum performance

#### 4. Compute Shaders
- **WebGPU Shader** (`src/gpu/shaders/grain_compute.wgsl`): WGSL compute shader
- **Metal Shader** (`src/gpu/shaders/grain_compute.metal`): Metal compute shader
- Both implement the same grain rendering algorithm on GPU

## Performance Benefits

### Expected Performance Improvements
- **Apple Silicon (Metal)**: 5-10x faster than CPU
- **NVIDIA GPUs (CUDA)**: 3-8x faster than CPU
- **AMD GPUs (OpenCL)**: 3-6x faster than CPU
- **WebGPU**: 2-5x faster than CPU

### Performance Indicators
The UI now shows performance indicators:
- 🚀 GPU: < 100ms (GPU acceleration active)
- ⚡ Fast: < 500ms (Fast processing)
- 🔄 CPU: < 2000ms (CPU processing)
- 🐌 Slow: > 2000ms (Slow processing)

## Features

### Automatic GPU Detection
- Detects available GPU hardware at startup
- Displays active GPU backend in the UI
- Automatically falls back to CPU if GPU fails

### Realistic Grain Rendering
- Maintains all existing grain characteristics
- Preserves film stock authenticity
- Supports all grain parameters (size, opacity, clustering, etc.)

### Memory Efficient
- Streams grain data to GPU memory
- Optimized buffer management
- Minimal CPU-GPU transfer overhead

## Usage

### For Users
No changes required! The application automatically:
1. Detects your GPU hardware
2. Initializes the best available backend
3. Shows GPU info in the interface
4. Falls back to CPU if needed

### For Developers

#### Building with GPU Support
```bash
# Default build (includes GPU acceleration)
cargo build --release

# Build without GPU acceleration
cargo build --release --no-default-features

# Build with specific GPU backend
cargo build --release --features cuda
cargo build --release --features opencl
```

#### Adding New GPU Backends
1. Create new backend in `src/gpu/`
2. Implement `GpuGrainRenderer` trait
3. Add to `GpuManager::initialize_best_backend()`
4. Update `Cargo.toml` dependencies

## Technical Details

### Grain Data Structure
```rust
struct GpuGrain {
    position: [f32; 2],    // X, Y coordinates
    size: f32,             // Grain radius
    opacity: f32,          // Alpha value
    shape_factor: f32,     // Aspect ratio for realistic shapes
}
```

### Shader Algorithm
1. **Parallel Processing**: Each pixel processed by separate GPU thread
2. **Distance Calculation**: Compute distance from pixel to each grain center
3. **Shape Application**: Apply grain shape factor for realistic forms
4. **Edge Falloff**: Implement soft/hard edges based on film stock
5. **Alpha Blending**: Combine multiple overlapping grains

### Memory Layout
- Grain data: Structured buffer (read-only)
- Parameters: Uniform buffer (read-only)
- Output: 2D texture (write-only)
- Result: Copied back to CPU as RGBA bytes

## Troubleshooting

### GPU Not Detected
- Check GPU drivers are installed
- Verify GPU supports compute shaders
- Check console for GPU initialization messages

### Performance Issues
- Monitor GPU memory usage
- Check for driver updates
- Verify adequate GPU memory (>1GB recommended)

### Fallback to CPU
- Normal behavior if GPU unavailable
- Check console for specific error messages
- CPU rendering still fully functional

## Future Enhancements

### Planned Features
1. **Real-time Preview**: Live grain updates while adjusting parameters
2. **Batch Processing**: GPU-accelerated batch image processing
3. **Advanced Effects**: GPU-based halation and light interaction
4. **Memory Optimization**: Streaming for very large images
5. **Multi-GPU Support**: Distribute work across multiple GPUs

### Platform Expansion
- **Vulkan Backend**: For Linux/Windows performance
- **DirectX 12**: Native Windows GPU acceleration
- **Mobile GPUs**: iOS/Android support via Metal/Vulkan

## Dependencies

### Core GPU Dependencies
- `wgpu`: WebGPU implementation
- `bytemuck`: Safe type casting for GPU buffers
- `pollster`: Async runtime for GPU operations

### Platform-Specific
- `metal`: macOS Metal framework
- `cudarc`: CUDA runtime (optional)
- `opencl3`: OpenCL 3.0 (optional)

## License & Credits

This GPU acceleration implementation maintains compatibility with the original Film Grain Generator while adding significant performance improvements across all major GPU vendors and platforms.