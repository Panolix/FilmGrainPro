# Film Grain Generator

A professional desktop application for generating authentic film grain overlays. Create realistic grain patterns based on 40+ actual film stocks with scientifically accurate characteristics.

![Film Grain Generator](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![Rust](https://img.shields.io/badge/Built%20with-Rust%20%7C%20Tauri-orange)

## Features

### 🎬 Authentic Film Simulation
- **40+ Film Stocks**: Kodak, Fuji, Ilford, Agfa, and specialty cinema films
- **Realistic Grain Characteristics**: Based on actual film stock data
- **Film-Specific Colors**: Authentic grain colors for each stock
- **Educational Info Panels**: Learn about each film's history and characteristics

### ⚡ High Performance
- **Multi-threaded Rendering**: Utilizes all CPU cores for fast generation
- **Adjustable Density**: 10K-50K grains with performance controls
- **Real-time Preview**: Instant slider updates
- **Professional Quality**: Up to 2048x2048 resolution output

### 🎨 Professional Controls
- **Grain Intensity**: 10-200% effect strength
- **Grain Size**: 0.2x-5.0x size multiplier
- **Grain Opacity**: 20-100% visibility control
- **Canvas Size**: 512-2048px output resolution
- **Max Grain Density**: Performance vs. quality balance

### 💾 Export Options
- **Transparent PNG**: Perfect for overlay use
- **High Resolution**: Up to 2048x2048 output
- **Professional Quality**: Ready for design work

## Installation

### Prerequisites
- **macOS**: 10.15+ (Catalina or later)
- **Windows**: Windows 10/11
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 100MB free space

### Option 1: Download Release (Recommended)
1. Go to [Releases](https://github.com/yourusername/film-grain-generator/releases)
2. Download the latest version for your platform:
   - **macOS**: `Film-Grain-Generator_x.x.x_aarch64.dmg` (Apple Silicon) or `Film-Grain-Generator_x.x.x_x64.dmg` (Intel)
   - **Windows**: `Film-Grain-Generator_x.x.x_x64.msi`
3. Install and run

### Option 2: Build from Source
See [INSTALL.md](INSTALL.md) for detailed build instructions.

## Quick Start

1. **Launch the application**
2. **Select a film stock** from the dropdown (e.g., "Kodak Tri-X 400")
3. **Adjust grain settings** using the sliders
4. **Set max grain density** based on your system performance
5. **Click "Regenerate Grain"** to see the result
6. **Click "Save PNG"** to export your grain overlay

## Film Stocks Included

### Black & White Films
- Kodak Tri-X 400, T-Max 400/100/3200, Plus-X 125
- Ilford HP5 Plus, FP4 Plus, Delta 100/400/3200, Pan F Plus 50
- Fuji Acros 100/II, Neopan 400/1600

### Color Negative Films
- Kodak Portra 160/400/800, Ektar 100, Gold 200, UltraMax 400
- Fuji Pro 400H/160S, Superia 400, C200, Natura 1600
- Agfa Vista 200/400

### Color Slide Films
- Kodak Ektachrome E100, Elite Chrome 100
- Fuji Velvia 50/100, Provia 100F/400X
- Agfa CT Precisa 100

### Specialty/Cinema Films
- Kodak Vision3 50D/200T/500T
- CineStill 50D/800T

## Usage Tips

### Performance Settings
- **Fast Preview**: Use 10K-20K grains while adjusting settings
- **Final Output**: Use 30K-50K grains for professional results
- **Real-time Adjustments**: Lower grain density for responsive sliders

### Realistic Settings
- **Subtle Film Look**: Intensity 30-50%, Size 1.0x, Opacity 50-70%
- **Classic Film**: Intensity 60-80%, Size 1.5x, Opacity 70-90%
- **Heavy Grain**: Intensity 100-150%, Size 2.0x+, Opacity 80-100%

### Film Stock Characteristics
- **Fine Grain**: Velvia 50, T-Max 100, Acros 100
- **Medium Grain**: Pro 400H, Portra 400, T-Max 400
- **Coarse Grain**: Tri-X 400, HP5 Plus, Delta 3200

## Technical Details

### Built With
- **Rust**: High-performance backend
- **Tauri**: Cross-platform desktop framework
- **Multi-threading**: Parallel grain rendering
- **Scientific Accuracy**: Based on real film stock data

### System Requirements
- **CPU**: Multi-core processor recommended
- **Memory**: 4GB RAM (8GB for high grain densities)
- **Graphics**: Any modern GPU (future GPU acceleration planned)

## Contributing

Contributions are welcome! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup
See [INSTALL.md](INSTALL.md) for development environment setup.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Film stock data compiled from technical specifications and community research
- Grain simulation algorithms based on photographic science literature
- UI design inspired by professional photo editing software

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/film-grain-generator/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/film-grain-generator/discussions)
- **Documentation**: [Wiki](https://github.com/yourusername/film-grain-generator/wiki)

---

**Made with ❤️ for film photography enthusiasts and digital artists**