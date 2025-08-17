# 🎬 Film Grain Generator

A professional desktop application for generating **authentic film grain overlays** with scientifically accurate characteristics based on exactly 35 real film stocks.

![Platform Support](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/License-Proprietary-red)
![Built with](https://img.shields.io/badge/Built%20with-Rust%20%7C%20Tauri%20%7C%20JavaScript-orange)

## 📸 **App Overview**

![Film Grain Generator Interface](Images%20Readme/Overview.png)

*Professional interface featuring: **Left** - Film stock info panel with technical specifications, **Center** - Live grain generation canvas, **Right** - Precision toolbox with sliders and selectors, **Bottom** - Real-time performance metrics showing grain count and generation speed*

## ✨ Features

### 🎞️ **Authentic Film Simulation**
- **35 Real Film Stocks**: Kodak, Fuji, Ilford, Agfa, and specialty cinema films
- **Scientifically Accurate**: Based on actual film stock technical specifications
- **Realistic Grain Characteristics**: Authentic sizes, shapes, densities, and clustering patterns
- **Film-Specific Colors**: Each stock has its unique grain color signature

### ⚡ **High Performance**
- **Multi-threaded Rendering**: Utilizes all CPU cores for fast generation
- **Adjustable Density**: 10K-200K grains (0.5x-5.0x multiplier) with real-time performance controls
- **Instant Preview**: Real-time slider updates with optimized rendering
- **Professional Quality**: Up to 2048x2048 resolution output

### 🎨 **Professional Controls**
- **Exposure Compensation**: -2 to +2 stops (simulates over/under exposure)
- **Grain Size**: 0.2x-5.0x size multiplier  
- **Grain Opacity**: 20-100% visibility control
- **Grain Density**: 0.5x-5.0x multiplier (film stock dependent)
- **Canvas Size**: 512-2048px output resolution

### 💾 **Export Options**
- **Transparent PNG**: Perfect for overlay use in photo/video editing
- **High Resolution**: Up to 2048x2048 output
- **Professional Quality**: Ready for commercial design work

## 🎯 **What Makes It Realistic?**

This isn't just random noise - it's **scientifically accurate film grain simulation**:

### **Authentic Film Stock Data:**
- **Grain Shapes**: Irregular (Tri-X), Tabular (T-Max), Sigma (Fuji), Core-Shell (Color films)
- **Real Sizes**: Measured in micrometers from actual film specifications
- **Density Values**: Based on grains per mm² from technical datasheets
- **Edge Types**: Sharp, soft, or hard edges matching real film characteristics
- **Clustering Patterns**: Realistic grain grouping based on film development chemistry

### **Film Stocks Included:**
- **Black & White**: Kodak Tri-X 400, T-Max 100/400/3200, Ilford HP5 Plus, Delta series, Fuji Acros
- **Color Negative**: Kodak Portra 160/400/800, Ektar 100, Fuji Pro 400H, Superia series
- **Color Slide**: Kodak Ektachrome, Fuji Velvia 50/100, Provia 100F
- **Cinema**: Kodak Vision3 series, CineStill 50D/800T

### **Authentic Grain Differences**

Each film stock produces distinctly different grain characteristics:

| Fuji Acros 100 (B&W) | Fuji Pro 400H (Color) |
|:---------------------:|:----------------------:|
| ![Fuji Acros 100](Images%20Readme/Fuji%20Acros%20100.png) | ![Fuji Pro 400H](Images%20Readme/Fuji%20Pro%20400H.png) |
| *Fine, sharp tabular grains with tight clustering* | *Larger, softer color grains with organic distribution* |

Notice the differences in **grain size**, **shape**, **color**, **density**, and **clustering patterns** - exactly as they appear in real film stocks!

*Note: Density multiplier set to 5x for demonstration purposes to clearly show grain characteristics*

## 🚀 **Quick Start**

### **One-Click Installation:**

**macOS:**
```bash
# Double-click this file in Finder:
🎬 SIMPLE INSTALLER.command
```

**Windows:**
```cmd
# Double-click this file in Explorer:
🎬 INSTALL FILM GRAIN GENERATOR.py
```

**Or use the cross-platform installer:**
```bash
python3 "🎬 INSTALL FILM GRAIN GENERATOR.py"
```

### **Prerequisites:**
- **Python 3.6+** (usually pre-installed on macOS)
- **Node.js 16+** ([Download](https://nodejs.org/))
- **Rust & Cargo** ([Install](https://rustup.rs/))

The installer will check for these and guide you through any missing requirements.

## 📱 **How to Use**

1. **Launch** Film Grain Generator
2. **Select a film stock** (e.g., "Kodak Tri-X 400")
3. **Adjust settings**:
   - **Density**: Choose grain count (10K-50K)
   - **Intensity**: Control grain visibility
   - **Size**: Scale grain size
   - **Opacity**: Fine-tune transparency
4. **Click "Regenerate Grain"** to see results
5. **Export as PNG** for use in your projects

## 🎨 **Usage Examples**

### **Photography:**
- Add authentic film look to digital photos
- Match grain to specific film stocks you've used
- Create consistent grain across photo series

### **Video/Cinema:**
- Add film grain to digital footage
- Match specific cinema film stocks (Vision3, etc.)
- Create vintage or artistic effects

### **Design:**
- Texture overlays for graphic design
- Authentic film aesthetic for branding
- Print design with realistic film grain

## 🔧 **Technical Details**

### **Built With:**
- **Backend**: Rust (high-performance grain generation)
- **Frontend**: JavaScript + HTML/CSS
- **Framework**: Tauri (cross-platform desktop)
- **Rendering**: Multi-threaded parallel processing
- **Data**: JSON-based film stock database

### **Performance:**
- **Multi-core**: Uses all available CPU threads
- **Memory Efficient**: Optimized grain rendering algorithms
- **Real-time**: Instant preview updates
- **Scalable**: Handles up to 50K+ grains smoothly

## 📊 **System Requirements**

### **Minimum:**
- **macOS**: 10.15+ (Catalina) or **Windows**: 10/11
- **Memory**: 4GB RAM
- **Storage**: 100MB free space
- **CPU**: Any modern multi-core processor

### **Recommended:**
- **Memory**: 8GB+ RAM (for high grain densities)
- **CPU**: 4+ cores (for faster rendering)

## 🤝 **Contributing**

Contributions are welcome! Areas for improvement:
- Additional film stock data
- New grain algorithms
- UI/UX enhancements
- Performance optimizations

## 📄 **License**

**Proprietary License** - This software is free for personal and educational use only. All rights reserved.

**🚫 STRICTLY PROHIBITED:**
- Selling or charging for the software
- Modifying and redistributing  
- Commercial use of any kind
- Creating derivative works for distribution
- Reverse engineering for commercial purposes

**✅ ALLOWED:**
- Personal, non-commercial use
- Educational use in academic settings
- Learning from source code (viewing only)

**⚖️ ENFORCEMENT:** Violations will result in legal action for copyright infringement.

For commercial licensing, contact the author. See [LICENSE](LICENSE) file for complete terms.

## 🙏 **Acknowledgments**

- Film stock data compiled from technical specifications and community research
- Grain simulation algorithms based on photographic science literature
- UI design inspired by professional photo editing software

## 📞 **Support**

- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)

---

**Made with ❤️ for film photography enthusiasts and digital artists**

*Create authentic film grain overlays that actually look like real film - because your art deserves better than generic noise.*