import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

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

// Same alpha-weighted, mean-centered dark grain as the gray MOV converter.
// Display code 512 as 50% RGB gray rather than applying a video range transform.
function grayGrainPreview(data) {
    let sum = 0;
    const valueAt = i => (0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2])
        / 255 * data[i + 3] / 255;
    for (let i = 0; i < data.length; i += 4) sum += valueAt(i);
    const mean = sum / (data.length / 4);
    const output = new Uint8ClampedArray(data.length);
    for (let i = 0; i < data.length; i += 4) {
        const y = Math.max(0, Math.min(1023, Math.round(512 - (valueAt(i) - mean) * 511)));
        const display = Math.round(y * 255 / 1023);
        output[i] = output[i + 1] = output[i + 2] = display;
        output[i + 3] = 255;
    }
    return output;
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
        this.loadAppSettings();
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
            this.generateInitialGrain();
            
        } catch (error) {
            console.error('Failed to load film stocks:', error);
            // Fall back to generating with default stock
            this.generateInitialGrain();
        }
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
            document.getElementById('filmInfoGraininess').textContent = src(filmInfo.graininess);
            document.getElementById('filmInfoMorphology').textContent = src(filmInfo.morphology);
            document.getElementById('filmInfoGrainSize').textContent = src(filmInfo.grain_size_um);
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
        
        document.getElementById('saveBtn').addEventListener('click', () => {
            this.saveImage();
        });
        document.getElementById('saveMovBtn').addEventListener('click', () => this.saveGrainMov());
        document.getElementById('saveSettingsBtn').addEventListener('click', () => this.saveAppSettings());
        document.getElementById('browseFfmpegBtn').addEventListener('click',
            () => this.browseSettingsDirectory('ffmpegDirectory', 'browseFfmpegBtn'));
        document.getElementById('browseOutputBtn').addEventListener('click',
            () => this.browseSettingsDirectory('outputDirectory', 'browseOutputBtn'));
        document.getElementById('previewMode').addEventListener('change', () => {
            if (this.currentImageData) this.displayGrainResult({
                data: this.currentImageData, width: this.canvas.width, height: this.canvas.height
            });
        });
        document.getElementById('movResolution').addEventListener('change', () => this.regenerateGrain());
        document.getElementById('movMode').addEventListener('change', () => {
            document.getElementById('previewMode').value =
                document.getElementById('movMode').value === 'transparent' ? 'original' : 'neutral_gray';
            if (this.currentImageData) this.displayGrainResult({
                data: this.currentImageData, width: this.canvas.width, height: this.canvas.height
            });
        });
        document.getElementById('previewZoom').addEventListener('change', () => this.updatePreviewView());
        window.addEventListener('resize', () => this.updatePreviewView());
        this.setupPreviewPanning();
        document.getElementById('cancelMovBtn').addEventListener('click', async () => {
            try {
                await invoke('cancel_grain_mov');
                document.getElementById('movStatus').textContent = 'Cancelling…';
            } catch (error) {
                document.getElementById('movStatus').textContent = String(error);
            }
        });

        document.getElementById('saveCompositeBtn').addEventListener('click', () => {
            this.saveCompositeImage();
        });

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
        this.regenerateGrain();
    }

    async generateInitialGrain() {
        await this.regenerateGrain();
    }
    
    async regenerateGrain() {
        const params = this.getMovParameters();
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
    
    getMovParameters() {
        const params = this.getGrainParameters();
        const resolution = document.getElementById('movResolution').value;
        if (resolution !== 'canvas') {
            [params.width, params.height] = resolution.split('x').map(Number);
        }
        return params;
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

    displayGrainResult(result) {
        const { data, width, height } = result;
        
        // Set canvas size
        this.canvas.width = width;
        this.canvas.height = height;
        
        // Create ImageData from the raw RGBA data
        const imageData = new ImageData(new Uint8ClampedArray(data), width, height);
        
        // If there's an uploaded image, use it as background instead of black
        if (document.getElementById('previewMode').value === 'neutral_gray') {
            this.ctx.putImageData(new ImageData(grayGrainPreview(data), width, height), 0, 0);
        } else if (this.uploadedImage && this.uploadedImageElement) {
            console.log('Drawing uploaded image as background');
            
            // Draw uploaded image as background
            this.ctx.drawImage(this.uploadedImageElement, 0, 0, this.canvas.width, this.canvas.height);
            
            // Create temporary canvas for grain to preserve transparency
            const tempCanvas = document.createElement('canvas');
            tempCanvas.width = width;
            tempCanvas.height = height;
            const tempCtx = tempCanvas.getContext('2d');
            tempCtx.putImageData(imageData, 0, 0);
            
            // Draw grain on top preserving its transparency
            this.ctx.drawImage(tempCanvas, 0, 0);
            
            console.log('Grain drawn on top of uploaded image');
        } else {
            // No uploaded image, use black background as before
            this.ctx.fillStyle = '#000000';
            this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
            
            // Draw the transparent grain on top
            this.ctx.putImageData(imageData, 0, 0);
        }
        
        // Store original transparent data for saving (unchanged)
        this.currentImageData = data;
        this.updatePreviewView();
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
                
                this.currentImageData = null;
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

    async loadAppSettings() {
        try {
            const settings = await invoke('get_app_settings');
            document.getElementById('ffmpegDirectory').value = settings.ffmpeg_directory;
            document.getElementById('outputDirectory').value = settings.output_directory;
        } catch (error) {
            document.getElementById('settingsStatus').textContent = String(error);
        }
    }

    async browseSettingsDirectory(field, buttonId) {
        const button = document.getElementById(buttonId);
        button.disabled = true;
        try {
            const path = await invoke('choose_settings_directory', {
                initial: document.getElementById(field).value
            });
            if (path !== null) {
                document.getElementById(field).value = path;
                document.getElementById('settingsStatus').textContent = 'Click Save Settings To Apply.';
            }
        } catch (error) {
            document.getElementById('settingsStatus').textContent = String(error);
        } finally {
            button.disabled = false;
        }
    }

    async saveAppSettings() {
        const button = document.getElementById('saveSettingsBtn');
        button.disabled = true;
        try {
            await invoke('save_app_settings', { settings: {
                ffmpeg_directory: document.getElementById('ffmpegDirectory').value,
                output_directory: document.getElementById('outputDirectory').value
            } });
            document.getElementById('settingsStatus').textContent = 'Settings Saved. Applies To The Next Export.';
        } catch (error) {
            document.getElementById('settingsStatus').textContent = String(error);
        } finally {
            button.disabled = false;
        }
    }

    async saveGrainMov() {
        if (this.exportingMov) return;
        const params = this.getMovParameters();
        const options = {
            fps: Number(document.getElementById('movFps').value),
            seconds: Number(document.getElementById('movSeconds').value),
            mode: document.getElementById('movMode').value
        };
        const status = document.getElementById('movStatus');
        if (!Number.isInteger(options.seconds) || options.seconds < 1 || options.seconds > 60) {
            status.textContent = 'Duration must be a whole number from 1 to 60 seconds.';
            return;
        }
        this.exportingMov = true;
        const button = document.getElementById('saveMovBtn');
        const cancel = document.getElementById('cancelMovBtn');
        const progress = document.getElementById('movProgress');
        button.disabled = true;
        button.textContent = 'Exporting MOV…';
        cancel.style.display = 'block';
        progress.style.display = 'block';
        progress.max = options.fps * options.seconds;
        progress.value = 0;
        status.textContent = 'Preparing MOV Export…';
        let unlisten;
        try {
            unlisten = await listen('grain-mov-progress', ({ payload }) => {
                progress.max = payload.total;
                progress.value = payload.frame;
                status.textContent = payload.frame === payload.total
                    ? 'Finalizing MOV…' : `Generating Frame ${payload.frame} / ${payload.total}`;
            });
            const path = await invoke('save_grain_mov', { params, options });
            status.textContent = 'Saved: ' + path;
        } catch (error) {
            status.textContent = String(error);
        } finally {
            if (unlisten) unlisten();
            this.exportingMov = false;
            button.disabled = false;
            button.textContent = '🎞 Save Grain MOV';
            cancel.style.display = 'none';
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
                width: this.canvas.width,
                height: this.canvas.height,
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
                grainWidth: this.canvas.width,
                grainHeight: this.canvas.height,
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
