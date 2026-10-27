# 🎬 FilmGrainPro

A professional desktop application for generating **authentic film grain overlays** with scientifically informed characteristics based on 37 real film stocks.

![Platform Support](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/License-CC_BY--NC--SA_4.0-blue)
![Built with](https://img.shields.io/badge/Built%20with-Rust%20%7C%20Tauri%20%7C%20JavaScript-orange)

## 📸 **App Overview**

![FilmGrainPro Interface](Images%20Readme/Overview.png)

*Professional interface featuring: **Left** - Film stock info panel with technical specifications, **Center** - Live grain generation canvas, **Right** - Precision toolbox with sliders and selectors, **Bottom** - Real-time performance metrics showing grain count and generation speed. (Screenshots predate the 2.2 grain-model rewrite; current output differs.)*

## ✨ Features

### 🎞️ **Authentic Film Simulation**
- **37 Real Film Stocks**: Complete database including Kodak, Fuji, Ilford, Agfa, and CineStill films
- **Scientifically Informed**: Grain amplitude driven by published RMS/PGI granularity where available
- **Realistic Grain Characteristics**: Film-specific sizes, shapes, densities, and clustering patterns
- **Film-Specific Colors**: Each stock has its own grain color signature

### ⚡ **High Performance**
- **Multi-threaded Rendering**: Utilizes all CPU cores for fast generation
- **Adjustable Density**: 0.5x-5.0x multiplier (roughly 10K-1M grains depending on film and canvas)
- **Fast Preview**: Grain regenerates when a slider is released, with live value readouts
- **Professional Quality**: Up to 2048x2048 resolution output

### 🎨 **Professional Controls**
- **Exposure Compensation**: -2 to +2 stops (simulates over/under exposure)
- **Grain Size**: 0.2x-5.0x size multiplier  
- **Grain Opacity**: 20-100% visibility control
- **Grain Density**: 0.5x-5.0x multiplier (film stock dependent)
- **Film Age & Storage**: 0-10 years of simulated aging, fridge or room temperature
- **Canvas Size**: 512-2048px output resolution

### 💾 **Export Options**
- **Transparent PNG**: Perfect for overlay use in photo/video editing
- **Composite Export**: Upload your own image and save image + grain together
- **High Resolution**: Up to 2048x2048 output
- **Professional Quality**: Ready for commercial design work

## 🎯 **What Makes It Realistic?**

This isn't just random noise - it's a **grain model grounded in published film data**:

### **Film Model (37 Film Stocks):**
- **Shape-Based Rendering**: Authentic crystal structures - Sigma (Fuji), Tabular (T-Max), Irregular (Tri-X), Cubic (HP5)
- **ISO-Based Behavior**: High ISO films (Delta 3200) show chaotic grain; low ISO films (Ektar 100) are ultra-uniform
- **Grain Size Model**: Log-normal distribution (~0.16-3.6 µm range) scaled by ISO and emulsion class. Manufacturer datasheets do not publish per-film grain size, so these are documented estimates (see `tools/ESTIMATION.md`)
- **Density From Grain Size**: Grain count is derived from each film's grain size instead of an invented grains-per-mm² figure, so finer emulsions place more, smaller grains
- **Per-Channel Colour**: Independent cyan/magenta/yellow variation for colour negative and slide films
- **Pattern Distribution**: Clustered vs random grain placement based on film chemistry
- **Edge Type Rendering**: Soft, sharp, hard, crystalline edges matching real film characteristics
- **Authentic Clustering**: Per-film cluster sizes (mostly 2-6 grains) and spatial patterns
- **Contrast Characteristics**: Film-specific opacity based on actual contrast levels
- **Color Accuracy**: Authentic color casts per film stock (CineStill 800T warm, Fuji green bias, etc.)
- **Aging Simulation**: Q10-based fog/colour-shift model with storage temperature effects (fridge vs room temp)
- **Exposure Effects**: Published push/pull data where available, otherwise class-based grain growth

### **37 Film Stocks Included:**

**Black & White Films (12):**
- **Kodak**: Tri-X 400, T-Max 100/400
- **Ilford**: HP5 Plus, Delta 100/400/3200, FP4 Plus, Pan F Plus 50
- **Fuji**: Acros 100, Neopan 400/1600

**Color Negative Films (19):**
- **Kodak**: Portra 160/400/800, Ektar 100, Gold 200, UltraMax 400, Vision3 50D/250D/500T
- **Fuji**: Pro 160S/400H, C200, Superia 400, Natura 1600
- **Agfa**: Vista 200/400
- **CineStill**: 50D, 400D, 800T (with authentic halation effects)

**Color Slide Films (6):**
- **Fuji**: Velvia 50/100, Provia 100F/400X
- **Kodak**: Ektachrome E100
- **Agfa**: CT Precisa 100

### **Authentic Grain Differences**

Each film stock produces distinctly different grain characteristics:

| Fuji Acros 100 (B&W) | Fuji Pro 400H (Color) |
|:---------------------:|:----------------------:|
| ![Fuji Acros 100](Images%20Readme/Fuji%20Acros%20100.png) | ![Fuji Pro 400H](Images%20Readme/Fuji%20Pro%20400H.png) |
| *Fine, sharp sigma-crystal grains with tight clustering* | *Larger, softer color grains with organic distribution* |

Notice the differences in **grain size**, **shape**, **color**, **density**, and **clustering patterns** - the same cues that distinguish these stocks on real film.

*Note: Density multiplier set to 5x for demonstration purposes to clearly show grain characteristics. Images are from an earlier build; the 2.2 model renders different (finer, less opaque) grain.*

## 🚀 **Quick Start**

### **Install a Release (Recommended):**

Download the installer for your platform from [Releases](../../releases):

- **macOS**: `.dmg`
- **Windows**: `.msi`
- **Linux**: `.AppImage`, `.deb`, or `.rpm`

> Release builds are unsigned. On macOS, right-click the app and choose **Open** the first time you launch it; on Windows, SmartScreen may ask you to confirm.

### **Build From Source:**

**macOS:**
```bash
# Double-click this file in Finder:
🎬 SIMPLE INSTALLER.command
```

**Windows / cross-platform:**
```cmd
# Double-click this file in Explorer, or run:
python "🎬 INSTALL FILM GRAIN GENERATOR.py"
```

### **Prerequisites (source build):**
- **Node.js 20+** ([Download](https://nodejs.org/))
- **Rust & Cargo** ([Install](https://rustup.rs/))
- **Python 3.8+** (used by the cross-platform installer; usually pre-installed on macOS)

The installer will check for these and guide you through any missing requirements.

## 📱 **How to Use**

1. **Launch** FilmGrainPro
2. **Select a film stock** (e.g., "Kodak Tri-X 400")
3. **Adjust settings** on the Basic tab:
   - **Grain Density Multiplier**: 0.5x-5.0x grain count
   - **Grain Size Multiplier**: 0.2x-5.0x grain size
   - **Grain Opacity**: 20-100% visibility
4. **Optional** — Advanced tab: exposure compensation (-2 to +2 stops), film age (0-10 years), storage temperature, canvas size (512-2048px)
5. **Click "Regenerate Grain"** to see results (double-click a slider to reset it, or click its value to type an exact number)
6. **Export**: "Save Grain Only" writes a transparent PNG; upload an image on the Output tab to composite it with the grain instead

The left-hand info panel shows the selected stock's sourced metrics (ISO, graininess, morphology, push range) where published.

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
- **Multi-core**: Uses all available CPU threads (Rayon)
- **High-Density Generation**: Up to ~1M grains at the top density multiplier on fine-grained stocks
- **Fast Preview**: Rendering runs off the UI thread, and stale results are discarded if settings change mid-render
- **Memory Efficient**: Tiled compositing into a single RGBA buffer
- **Advanced Clustering**: Per-film grain grouping based on film chemistry

### **Data & Model Documentation:**
- `tools/SIMULATION_MODEL.md` - how the sourced data drives rendering
- `tools/DATA_REQUIREMENTS.md` - which fields are datasheet, derived, or estimated
- `tools/ESTIMATION.md` - the ISO/morphology model for unmeasurable fields
- `tools/SOURCES.md` - source list and confidence levels

## 📊 **System Requirements**

### **Minimum:**
- **macOS**: 10.15+ (Catalina), **Windows**: 10/11, or **Linux**: any modern distro with WebKitGTK
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

This project is licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) - free for personal and educational use. Commercial use requires permission.

See the [LICENSE](LICENSE) file for complete terms.

## 🙏 **Acknowledgments**

- Film stock data compiled from technical specifications and community research
- Grain simulation algorithms based on photographic science literature
- UI design inspired by professional photo editing software

## 📞 **Support**

- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)
- **Changelog**: [RELEASE_NOTES.md](RELEASE_NOTES.md)

---

**Made with ❤️ for film photography enthusiasts and digital artists**

*Create authentic film grain overlays that actually look like real film - because your art deserves better than generic noise.*