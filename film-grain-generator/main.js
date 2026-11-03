import { invoke } from '@tauri-apps/api/core';

// The density slider is mapped piecewise-linearly so each printed tick
// (0.5x, 1.0x, 2.0x, 3.0x, 5.0x) sits at its true position under the track.
const DENSITY_STOPS = [
    { pos: 0, value: 0.5 },
    { pos: 25, value: 1.0 },
    { pos: 50, value: 2.0 },
    { pos: 75, value: 3.0 },
    { pos: 100, value: 5.0 }
];

function densityFromSlider(pos) {
    for (let i = 0; i < DENSITY_STOPS.length - 1; i++) {
        const a = DENSITY_STOPS[i];
        const b = DENSITY_STOPS[i + 1];
        if (pos <= b.pos) {
            const t = (pos - a.pos) / (b.pos - a.pos);
            return a.value + t * (b.value - a.value);
        }
    }
    return DENSITY_STOPS[DENSITY_STOPS.length - 1].value;
}

function sliderFromDensity(value) {
    const last = DENSITY_STOPS[DENSITY_STOPS.length - 1];
    const v = Math.max(DENSITY_STOPS[0].value, Math.min(last.value, value));
    for (let i = 0; i < DENSITY_STOPS.length - 1; i++) {
        const a = DENSITY_STOPS[i];
        const b = DENSITY_STOPS[i + 1];
        if (v <= b.value) {
            const t = (v - a.value) / (b.value - a.value);
            return a.pos + t * (b.pos - a.pos);
        }
    }
    return last.pos;
}

// Backgrounds are drawn as flat fills or horizontal ramps; the old
// transparency checkerboard is gone in favour of the alpha-matte view.

const LUMINANCE_STOPS = ['#000000', '#404040', '#767676', '#c0c0c0', '#ffffff'];
const SKIN_STOPS = ['#3b2219', '#6b4433', '#9c6b4e', '#c68b59', '#e0ac69', '#f1d3b6'];

// Horizontal tonal ramp. Used for both the luminance reference and the skin range.
function drawRamp(ctx, width, height, stops) {
    const ramp = ctx.createLinearGradient(0, 0, width, 0);
    stops.forEach((colour, i) => ramp.addColorStop(i / (stops.length - 1), colour));
    ctx.fillStyle = ramp;
    ctx.fillRect(0, 0, width, height);
}

// Coverage view: alpha as greyscale, so grain structure can be judged without a
// background colour or blend mode getting in the way.
function alphaMatte(data, width, height) {
    const out = new Uint8ClampedArray(width * height * 4);
    for (let i = 0; i < width * height; i++) {
        const a = data[i * 4 + 3];
        out[i * 4] = a;
        out[i * 4 + 1] = a;
        out[i * 4 + 2] = a;
        out[i * 4 + 3] = 255;
    }
    return new ImageData(out, width, height);
}

class FilmGrainGenerator {
    constructor() {
        this.canvas = document.getElementById('grainCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.currentImageData = null;
        this.uploadedImage = null;
        this.uploadedImageWidth = 0;
        this.uploadedImageHeight = 0;
        this.updateTimeout = null;
        this.generationId = 0;
        this.loadingHideTimeout = null;
        this.loadingAnimTimeouts = [];
        this.regenerateLabel = document.getElementById('regenerateBtn').textContent;
        
        this.initializeControls();
    }
    
    async loadFilmStocks() {
        try {
            // Get categorized film stocks from backend
            const result = await invoke('get_categorized_film_stocks');
            const filmStockSelect = document.getElementById('filmStock');
            
            // Clear existing options
            filmStockSelect.innerHTML = '';
            
            // Add categorized film stocks with optgroups
            Object.keys(result).forEach(category => {
                const optgroup = document.createElement('optgroup');
                optgroup.label = category;
                
                // Sort manufacturers within each category
                const manufacturers = result[category];
                Object.keys(manufacturers).sort().forEach(manufacturer => {
                    const stocks = manufacturers[manufacturer].sort();
                    stocks.forEach(stockName => {
                        const option = document.createElement('option');
                        option.value = stockName;
                        option.textContent = stockName;
                        optgroup.appendChild(option);
                    });
                });
                
                filmStockSelect.appendChild(optgroup);
            });
            
            // Generate initial grain after loading stocks
            this.updateFilmInfo();
            this.captureControlDefaults();
            this.generateInitialGrain();
            
        } catch (error) {
            console.error('Failed to load film stocks:', error);
            // Fall back to generating with default stock
            this.captureControlDefaults();
            this.generateInitialGrain();
        }
    }

    // Snapshot the control values once the film list is populated, so "Reset All"
    // restores the same state the app started in.
    captureControlDefaults() {
        this.controlDefaults = {};
        document.querySelectorAll('.controls input, .controls select').forEach(el => {
            if (el.type === 'file' || !el.id) return;
            this.controlDefaults[el.id] = el.value;
        });
    }

    resetAllDefaults() {
        const defaults = this.controlDefaults;
        if (!defaults) return;

        Object.entries(defaults).forEach(([id, value]) => {
            const el = document.getElementById(id);
            if (el && el.type !== 'file') el.value = value;
        });

        // Background Response is implied by the background choice.
        document.getElementById('previewMode').dispatchEvent(new Event('change'));

        // Refresh the value labels. The canvas sliders also trigger a regeneration.
        ['grainDensity', 'grainSize', 'contrast', 'grainIntensity', 'filmAge', 'canvasWidth', 'canvasHeight']
            .forEach(id => {
                const el = document.getElementById(id);
                if (el) el.dispatchEvent(new Event('input'));
            });

        this.updateFilmInfo();
    }

    async updateFilmInfo() {
        const filmStock = document.getElementById('filmStock').value;
        if (!filmStock) return;
        
        try {
            const filmInfo = await invoke('get_film_info', { filmName: filmStock });
            
            document.getElementById('filmInfoTitle').textContent = filmStock;
            document.getElementById('filmInfoDescription').textContent = filmInfo.description;
            document.getElementById('filmInfoUses').textContent = filmInfo.primary_uses.join(', ');
            document.getElementById('filmInfoCharacteristics').textContent = filmInfo.characteristics.join(', ');
            document.getElementById('filmInfoUsers').textContent = filmInfo.famous_users.join(', ');
            document.getElementById('filmInfoEra').textContent = filmInfo.era;
            document.getElementById('filmInfoPrice').textContent = filmInfo.price_category;
            const src = (v) => (v === null || v === undefined || v === '') ? '—' : v;
            document.getElementById('filmInfoIso').textContent = src(filmInfo.iso);
            document.getElementById('filmInfoQuality').textContent = src(filmInfo.data_quality);
            document.getElementById('filmInfoGraininess').textContent = src(filmInfo.graininess);
            document.getElementById('filmInfoIndex').textContent = src(filmInfo.relative_index);
            document.getElementById('filmInfoMorphology').textContent = src(filmInfo.morphology);
            document.getElementById('filmInfoGrainSize').textContent = src(filmInfo.grain_size_um);
            document.getElementById('filmInfoClustering').textContent = src(filmInfo.clustering);
            document.getElementById('filmInfoPush').textContent = src(filmInfo.push_range);
            document.getElementById('filmInfoResolving').textContent = src(filmInfo.resolving_power);
            
            document.getElementById('filmInfoDetails').style.display = 'block';
            
        } catch (error) {
            console.error('Failed to load film info:', error);
            document.getElementById('filmInfoTitle').textContent = filmStock;
            document.getElementById('filmInfoDescription').textContent = 'Film information not available.';
            document.getElementById('filmInfoDetails').style.display = 'none';
        }
    }
    
    async initializeControls() {
        // Load available film stocks dynamically
        await this.loadFilmStocks();
        
        // Film stock selector
        const filmStock = document.getElementById('filmStock');
        filmStock.addEventListener('change', () => {
            this.updateFilmInfo();
            this.regenerateGrain();
        });
        
        // Sliders with real-time updates
        const sliders = [
            'grainIntensity', 'grainSize', 'contrast', 'grainDensity',
            'canvasWidth', 'canvasHeight', 'filmAge'
        ];
        
        
        sliders.forEach(sliderId => {
            const slider = document.getElementById(sliderId);
            const valueDisplay = document.getElementById(sliderId + 'Value');
            
            // Store default values
            const defaultValues = {
                'grainIntensity': 0,
                'grainSize': 1.0,
                'contrast': 100,
                'grainDensity': 25,
                'canvasWidth': 1024,
                'canvasHeight': 1024,
                'filmAge': 0
            };
            
            const updateDisplay = (value) => {
                let displayValue = value;
                if (sliderId === 'grainIntensity') {
                    displayValue = displayValue == '0' ? '0' : (displayValue > 0 ? '+' + displayValue : displayValue);
                } else if (sliderId === 'contrast') {
                    displayValue += '%';
                } else if (sliderId === 'grainSize') {
                    displayValue += 'x';
                } else if (sliderId === 'grainDensity') {
                    displayValue = densityFromSlider(parseFloat(displayValue)).toFixed(1) + 'x';
                } else if (sliderId === 'filmAge') {
                    displayValue = displayValue == '0' ? 'Fresh' : displayValue + 'y';
                }
                valueDisplay.textContent = displayValue;
            };
            
            slider.addEventListener('input', (e) => {
                updateDisplay(e.target.value);
                
                if (sliderId === 'canvasWidth' || sliderId === 'canvasHeight') {
                    this.updateCanvasSize();
                }
            });
            
            // Double-click to reset to default
            slider.addEventListener('dblclick', () => {
                const defaultValue = defaultValues[sliderId];
                if (defaultValue !== undefined) {
                    slider.value = defaultValue;
                    updateDisplay(defaultValue);
                    
                    if (sliderId === 'canvasWidth' || sliderId === 'canvasHeight') {
                        this.updateCanvasSize();
                    } else {
                        this.regenerateGrain();
                    }
                }
            });
            
            // Click value display to edit
            valueDisplay.addEventListener('click', () => {
                // Density edits happen in multiplier units (0.5x-5.0x), not slider position.
                const isDensity = sliderId === 'grainDensity';
                const sliderValue = parseFloat(slider.value);
                const currentValue = isDensity ? densityFromSlider(sliderValue) : sliderValue;
                const min = isDensity ? DENSITY_STOPS[0].value : parseFloat(slider.min);
                const max = isDensity ? DENSITY_STOPS[DENSITY_STOPS.length - 1].value : parseFloat(slider.max);
                const step = isDensity ? 0.1 : (parseFloat(slider.step) || 1);
                
                const input = document.createElement('input');
                input.type = 'number';
                input.value = currentValue;
                input.min = min;
                input.max = max;
                input.step = step;
                // Get exact position of the value display
                const rect = valueDisplay.getBoundingClientRect();
                
                input.style.width = valueDisplay.offsetWidth + 'px';
                input.style.height = valueDisplay.offsetHeight + 'px';
                input.style.background = '#333';
                input.style.border = '1px solid #555';
                input.style.color = 'white';
                input.style.padding = '2px 4px';
                input.style.borderRadius = '3px';
                input.style.fontSize = '12px';
                input.style.textAlign = 'right';
                input.style.position = 'fixed';
                input.style.left = rect.left + 'px';
                input.style.top = rect.top + 'px';
                input.style.zIndex = '10000';
                
                document.body.appendChild(input);
                input.focus();
                input.select();
                
                const finishEdit = () => {
                    let newValue = parseFloat(input.value);
                    if (isNaN(newValue)) newValue = currentValue;
                    newValue = Math.max(min, Math.min(max, newValue));
                    
                    const newSliderValue = isDensity ? sliderFromDensity(newValue) : newValue;
                    slider.value = newSliderValue;
                    updateDisplay(newSliderValue);
                    
                    input.remove();
                    
                    if (sliderId === 'canvasWidth' || sliderId === 'canvasHeight') {
                        this.updateCanvasSize();
                    } else {
                        this.regenerateGrain();
                    }
                };
                
                input.addEventListener('blur', finishEdit);
                input.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter') finishEdit();
                    if (e.key === 'Escape') {
                        input.remove();
                    }
                });
            });
            
            slider.addEventListener('change', () => this.regenerateGrain());
            
            // Real-time updates for immediate feedback (but show loading for heavy operations)
            if (sliderId !== 'canvasWidth' && sliderId !== 'canvasHeight') {
                slider.addEventListener('input', () => {
                    clearTimeout(this.updateTimeout);
                    this.updateTimeout = setTimeout(() => {
                        // Show loading for real-time updates too
                        this.showLoadingBar('Updating grain...');
                        setTimeout(() => this.regenerateGrain(), 10);
                    }, 300);
                });
            }
        });
        
        // Storage type selector
        const storageType = document.getElementById('storageType');
        storageType.addEventListener('change', () => {
            this.regenerateGrain();
        });
        
        // Buttons
        document.getElementById('uploadBtn').addEventListener('click', () => {
            this.uploadImage();
        });

        document.getElementById('imageUpload').addEventListener('change', (e) => {
            this.handleImageUpload(e);
        });

        document.getElementById('regenerateBtn').addEventListener('click', () => {
            this.regenerateGrain();
        });

        document.getElementById('resetAllBtn').addEventListener('click', () => {
            this.resetAllDefaults();
        });
        
        document.getElementById('saveBtn').addEventListener('click', () => {
            this.saveImage();
        });

        document.getElementById('saveCompositeBtn').addEventListener('click', () => {
            this.saveCompositeImage();
        });

        document.getElementById('previewMode').addEventListener('change', () => {
            // Luminance response only makes sense when there is something visible
            // to respond to; black and the alpha matte default to flat.
            const mode = document.getElementById('previewMode').value;
            document.getElementById('previewBlend').value = (mode === 'black' || mode === 'alpha') ? 'flat' : 'luminance';
            if (this.currentImageData) this.displayGrainResult({
                data: this.currentImageData, width: this.canvas.width, height: this.canvas.height
            });
        });
        document.getElementById('previewBlend').addEventListener('change', () => {
            if (this.currentImageData) this.displayGrainResult({
                data: this.currentImageData, width: this.canvas.width, height: this.canvas.height
            });
        });
        document.getElementById('previewZoom').addEventListener('change', () => this.updatePreviewView());
        window.addEventListener('resize', () => this.updatePreviewView());
        this.setupPreviewPanning();

        // Tab functionality
        this.initializeTabs();
    }
    
    initializeTabs() {
        const tabButtons = document.querySelectorAll('.tab-button');
        const tabPanels = document.querySelectorAll('.tab-panel');
        
        tabButtons.forEach(button => {
            button.addEventListener('click', () => {
                const targetTab = button.getAttribute('data-tab');
                
                // Remove active class from all buttons and panels
                tabButtons.forEach(btn => btn.classList.remove('active'));
                tabPanels.forEach(panel => panel.classList.remove('active'));
                
                // Add active class to clicked button and corresponding panel
                button.classList.add('active');
                document.getElementById(targetTab + '-tab').classList.add('active');
            });
        });
    }
    
    autoResizeCanvasToImage(imageWidth, imageHeight) {
        // Calculate optimal canvas size while maintaining aspect ratio
        const maxCanvasSize = 2048; // Maximum dimension
        const minCanvasSize = 512;  // Minimum dimension
        
        const aspectRatio = imageWidth / imageHeight;
        let newWidth, newHeight;
        
        if (imageWidth > imageHeight) {
            // Landscape orientation
            newWidth = Math.min(imageWidth, maxCanvasSize);
            newHeight = Math.round(newWidth / aspectRatio);
        } else {
            // Portrait orientation
            newHeight = Math.min(imageHeight, maxCanvasSize);
            newWidth = Math.round(newHeight * aspectRatio);
        }
        
        // Ensure minimum size
        if (newWidth < minCanvasSize) {
            newWidth = minCanvasSize;
            newHeight = Math.round(newWidth / aspectRatio);
        }
        if (newHeight < minCanvasSize) {
            newHeight = minCanvasSize;
            newWidth = Math.round(newHeight * aspectRatio);
        }
        
        // Update the canvas size controls
        document.getElementById('canvasWidth').value = newWidth;
        document.getElementById('canvasHeight').value = newHeight;
        document.getElementById('canvasWidthValue').textContent = newWidth;
        document.getElementById('canvasHeightValue').textContent = newHeight;
        
        console.log(`📐 Auto-resized canvas: ${imageWidth}×${imageHeight} → ${newWidth}×${newHeight} (${aspectRatio.toFixed(2)}:1)`);
        
        // Trigger canvas update
        this.updateCanvasSize();
    }
    
    updateCanvasSize() {
        const width = parseInt(document.getElementById('canvasWidth').value);
        const height = parseInt(document.getElementById('canvasHeight').value);
        
        this.canvas.width = width;
        this.canvas.height = height;
        
        // Regenerate grain with new size
        this.regenerateGrain();
    }
    
    async generateInitialGrain() {
        await this.regenerateGrain();
    }
    
    async regenerateGrain() {
        const params = this.getGrainParameters();
        const regenerateBtn = document.getElementById('regenerateBtn');
        const requestId = ++this.generationId;

        try {
            // Show loading state immediately
            regenerateBtn.textContent = 'Generating...';
            regenerateBtn.disabled = true;
            
            // Show loading bar immediately and force UI update
            this.showLoadingBar('Generating grain...');
            
            // Force browser to render the loading bar before heavy computation
            await new Promise(resolve => setTimeout(resolve, 50));

            // A newer request has superseded this one while waiting.
            if (requestId !== this.generationId) return;

            // Call Rust backend for high-performance grain generation
            const result = await invoke('generate_grain', { params });

            // Ignore results from superseded requests so stale grain never wins.
            if (requestId !== this.generationId) return;
            
            // Convert the raw data to ImageData and display
            this.displayGrainResult(result);
            
            // Update performance info
            this.updatePerformanceInfo(result);
            
            // Hide loading bar
            this.hideLoadingBar();
            
            regenerateBtn.textContent = this.regenerateLabel;
            regenerateBtn.disabled = false;
            
        } catch (error) {
            if (requestId !== this.generationId) return;
            console.error('Error generating grain:', error);
            this.hideLoadingBar();
            regenerateBtn.textContent = this.regenerateLabel;
            regenerateBtn.disabled = false;
            alert('Error generating grain: ' + error);
        }
    }
    
    showLoadingBar(text = 'Loading...') {
        const loadingBar = document.getElementById('loadingBar');
        const loadingText = document.getElementById('loadingText');
        const loadingProgress = document.getElementById('loadingProgress');
        
        // Cancel any pending hide or in-flight progress animation from a previous run
        if (this.loadingHideTimeout) {
            clearTimeout(this.loadingHideTimeout);
            this.loadingHideTimeout = null;
        }
        this.loadingAnimTimeouts.forEach(clearTimeout);
        this.loadingAnimTimeouts = [];
        
        loadingText.textContent = text;
        loadingProgress.style.width = '0%';
        loadingBar.style.display = 'block';
        
        // Force immediate display
        loadingBar.style.opacity = '1';
        
        // Start progress animation immediately
        requestAnimationFrame(() => {
            loadingProgress.style.width = '20%';
            
            this.loadingAnimTimeouts.push(setTimeout(() => {
                loadingProgress.style.width = '40%';
            }, 100));
            this.loadingAnimTimeouts.push(setTimeout(() => {
                loadingProgress.style.width = '70%';
            }, 200));
            this.loadingAnimTimeouts.push(setTimeout(() => {
                loadingProgress.style.width = '85%';
            }, 300));
        });
    }
    
    hideLoadingBar() {
        const loadingBar = document.getElementById('loadingBar');
        const loadingProgress = document.getElementById('loadingProgress');
        
        // Complete the progress bar
        loadingProgress.style.width = '100%';
        
        // Hide after a brief moment
        if (this.loadingHideTimeout) {
            clearTimeout(this.loadingHideTimeout);
        }
        this.loadingHideTimeout = setTimeout(() => {
            loadingBar.style.display = 'none';
            this.loadingHideTimeout = null;
        }, 200);
    }
    
    getGrainParameters() {
        return {
            film_stock: document.getElementById('filmStock').value,
            exposure_compensation: parseFloat(document.getElementById('grainIntensity').value),
            size_multiplier: parseFloat(document.getElementById('grainSize').value),
            contrast: parseFloat(document.getElementById('contrast').value),
            grain_density: Math.round(densityFromSlider(parseFloat(document.getElementById('grainDensity').value)) * 1000),
            width: parseInt(document.getElementById('canvasWidth').value),
            height: parseInt(document.getElementById('canvasHeight').value),
            background: 'transparent',
            film_age_years: parseFloat(document.getElementById('filmAge')?.value || 0),
            storage_temp: parseFloat(document.getElementById('storageType')?.value || 20)
        };
    }
    
    displayGrainResult(result) {
        const { data, width, height } = result;

        this.canvas.width = width;
        this.canvas.height = height;

        const ctx = this.ctx;
        const mode = document.getElementById('previewMode').value;
        const respond = document.getElementById('previewBlend').value === 'luminance';

        ctx.globalAlpha = 1;
        ctx.globalCompositeOperation = 'source-over';

        if (mode === 'alpha') {
            ctx.putImageData(alphaMatte(data, width, height), 0, 0);
        } else {
            if (mode === 'image' && this.uploadedImage && this.uploadedImageElement) {
                ctx.drawImage(this.uploadedImageElement, 0, 0, width, height);
            } else if (mode === 'ramp') {
                drawRamp(ctx, width, height, LUMINANCE_STOPS);
            } else if (mode === 'skin') {
                drawRamp(ctx, width, height, SKIN_STOPS);
            } else if (mode === 'gray18') {
                ctx.fillStyle = '#767676';
                ctx.fillRect(0, 0, width, height);
            } else {
                ctx.fillStyle = '#000000';
                ctx.fillRect(0, 0, width, height);
            }

            // putImageData ignores blend modes, so stage the grain on a scratch canvas.
            const grainLayer = document.createElement('canvas');
            grainLayer.width = width;
            grainLayer.height = height;
            grainLayer.getContext('2d').putImageData(
                new ImageData(new Uint8ClampedArray(data), width, height), 0, 0);

            ctx.globalCompositeOperation = respond ? 'overlay' : 'source-over';
            ctx.drawImage(grainLayer, 0, 0);
            ctx.globalCompositeOperation = 'source-over';
        }

        // Store original transparent data for saving (unchanged)
        this.currentImageData = data;
        this.updatePreviewView();
    }

    updatePreviewView() {
        const viewport = document.getElementById('previewViewport');
        const native = document.getElementById('previewZoom').value === 'native';
        const ratio = window.devicePixelRatio || 1;
        const scale = native ? 1 / ratio : Math.min(
            viewport.clientWidth / this.canvas.width,
            viewport.clientHeight / this.canvas.height, 1 / ratio);
        this.canvas.style.width = `${this.canvas.width * scale}px`;
        this.canvas.style.height = `${this.canvas.height * scale}px`;
        viewport.classList.toggle('native', native);
        document.getElementById('previewInfo').textContent =
            `${this.canvas.width} × ${this.canvas.height} | ${Math.round(scale * ratio * 100)}% | ` +
            (native ? 'One Image Pixel Per Display Pixel' : 'Fit — Fine Grain May Be Smoothed');
    }

    setupPreviewPanning() {
        const viewport = document.getElementById('previewViewport');
        let drag = null;
        const stop = (event) => {
            if (!drag || event.pointerId !== drag.id) return;
            drag = null;
            viewport.classList.remove('dragging');
            if (viewport.hasPointerCapture(event.pointerId)) viewport.releasePointerCapture(event.pointerId);
        };
        viewport.addEventListener('pointerdown', (event) => {
            if (drag || event.button !== 0 || document.getElementById('previewZoom').value !== 'native') return;
            drag = { id: event.pointerId, x: event.clientX, y: event.clientY,
                left: viewport.scrollLeft, top: viewport.scrollTop };
            viewport.setPointerCapture(event.pointerId);
            viewport.classList.add('dragging');
            event.preventDefault();
        });
        viewport.addEventListener('pointermove', (event) => {
            if (!drag || event.pointerId !== drag.id) return;
            if (document.getElementById('previewZoom').value !== 'native') { stop(event); return; }
            viewport.scrollLeft = drag.left - (event.clientX - drag.x);
            viewport.scrollTop = drag.top - (event.clientY - drag.y);
        });
        viewport.addEventListener('pointerup', stop);
        viewport.addEventListener('pointercancel', stop);
        viewport.addEventListener('lostpointercapture', stop);
    }
    
    updatePerformanceInfo(result) {
        const info = document.getElementById('performanceInfo');
        info.innerHTML = `Generation time: ${result.generation_time_ms}ms | Grains: ${result.grain_count.toLocaleString()}`;
        
        // Add performance indicator (CPU multi-threaded rendering)
        const perfIndicator = result.generation_time_ms < 100 ? '🚀 Instant' : 
                             result.generation_time_ms < 500 ? '⚡ Fast' : 
                             result.generation_time_ms < 2000 ? '🔄 Normal' : '🐌 Slow';
        info.innerHTML += ` | ${perfIndicator}`;
    }
    
    uploadImage() {
        document.getElementById('imageUpload').click();
    }

    async handleImageUpload(event) {
        const file = event.target.files[0];
        if (!file) return;

        try {
            // Show loading for image upload
            this.showLoadingBar('Loading image...');
            
            // Simple approach - load image directly in browser
            const img = new Image();
            img.onload = () => {
                console.log('Image loaded successfully:', file.name);
                
                // Store the image element for later use
                this.uploadedImageElement = img;
                this.uploadedImage = true; // Flag to indicate image is loaded
                
                // Store image dimensions and auto-resize canvas
                this.uploadedImageWidth = img.naturalWidth;
                this.uploadedImageHeight = img.naturalHeight;
                this.autoResizeCanvasToImage(img.naturalWidth, img.naturalHeight);
                
                // Update canvas to show the image immediately
                const params = this.getGrainParameters();
                this.canvas.width = params.width;
                this.canvas.height = params.height;
                
                // Draw the uploaded image as background
                this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
                this.ctx.drawImage(img, 0, 0, this.canvas.width, this.canvas.height);
                
                // Show the composite save button
                document.getElementById('saveCompositeBtn').style.display = 'inline-block';
                
                // Update button text to show image is loaded
                document.getElementById('uploadBtn').textContent = `📁 ${file.name}`;
                
                console.log('Image displayed on canvas');
                
                // Hide loading bar
                this.hideLoadingBar();
                
                // Regenerate grain on top of the image
                this.regenerateGrain();
            };
            
            img.onerror = () => {
                console.error('Failed to load image');
                this.hideLoadingBar();
                alert('Failed to load image');
            };
            
            // Load image from file
            const reader = new FileReader();
            reader.onload = (e) => {
                // Keep the raw base64 (no data-URL prefix) for the Rust composite save
                this.uploadedImageBase64 = String(e.target.result).split(',')[1] || e.target.result;
                img.src = e.target.result;
            };
            reader.readAsDataURL(file);

            // Reset the input so selecting the same file again still fires 'change'
            event.target.value = '';
            
        } catch (error) {
            console.error('Error loading image:', error);
            alert('Error loading image: ' + error);
        }
    }

    async saveImage() {
        if (!this.currentImageData) {
            alert('No grain image to save');
            return;
        }
        
        try {
            const params = this.getGrainParameters();
            
            // Generate filename with timestamp and parameters
            const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
            const filename = `grain_${params.film_stock.replace(/\s+/g, '_')}_${timestamp}.png`;
            
            // Save to Downloads folder (simplified approach)
            await invoke('save_grain_image', {
                data: this.currentImageData,
                width: params.width,
                height: params.height,
                path: filename
            });
            
            alert(`Grain image saved as ${filename}!`);
            
        } catch (error) {
            console.error('Error saving image:', error);
            alert('Error saving image: ' + error);
        }
    }

    async saveCompositeImage() {
        if (!this.currentImageData || !this.uploadedImage) {
            alert('Please upload an image and generate grain first');
            return;
        }
        
        try {
            const params = this.getGrainParameters();
            
            // Generate filename with timestamp and parameters
            const timestamp = new Date().toISOString().replace(/[:.]/g, '-');
            const filename = `composite_${params.film_stock.replace(/\s+/g, '_')}_${timestamp}.png`;
            
            // Composite image with grain in Rust backend
            await invoke('save_composite_image', {
                grainData: this.currentImageData,
                grainWidth: params.width,
                grainHeight: params.height,
                baseImageData: this.uploadedImageBase64,
                path: filename
            });
            
            alert(`Composite image saved as ${filename}!`);
            
        } catch (error) {
            console.error('Error saving composite image:', error);
            alert('Error saving composite image: ' + error);
        }
    }
}

// Initialize the application when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new FilmGrainGenerator();
});