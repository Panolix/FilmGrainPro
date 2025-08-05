import { invoke } from '@tauri-apps/api/core';

class FilmGrainGenerator {
    constructor() {
        this.canvas = document.getElementById('grainCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.currentImageData = null;
        this.uploadedImage = null;
        this.uploadedImageWidth = 0;
        this.uploadedImageHeight = 0;
        this.updateTimeout = null;
        
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
            
            // Skip GPU info loading for now to avoid overhead
            // this.loadGpuInfo();
            
            // Generate initial grain after loading stocks
            this.updateFilmInfo();
            this.generateInitialGrain();
            
        } catch (error) {
            console.error('Failed to load film stocks:', error);
            // Fall back to generating with default stock
            this.generateInitialGrain();
        }
    }

    async loadGpuInfo() {
        try {
            const gpuInfo = await invoke('get_gpu_info');
            const infoElement = document.getElementById('generationInfo');
            if (infoElement) {
                // Add GPU info to the generation info display
                const gpuInfoDiv = document.createElement('div');
                gpuInfoDiv.style.fontSize = '12px';
                gpuInfoDiv.style.color = '#888';
                gpuInfoDiv.style.marginTop = '5px';
                gpuInfoDiv.textContent = `🚀 ${gpuInfo}`;
                infoElement.appendChild(gpuInfoDiv);
            }
            console.log('GPU Info:', gpuInfo);
        } catch (error) {
            console.error('Failed to get GPU info:', error);
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
                'grainDensity': 1000,
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
                    displayValue = (parseFloat(displayValue) / 1000.0).toFixed(1) + 'x';
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
                const currentValue = slider.value;
                const min = parseFloat(slider.min);
                const max = parseFloat(slider.max);
                const step = parseFloat(slider.step) || 1;
                
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
                    
                    slider.value = newValue;
                    updateDisplay(newValue);
                    
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
        
        try {
            // Show loading state immediately
            const regenerateBtn = document.getElementById('regenerateBtn');
            const originalText = regenerateBtn.textContent;
            regenerateBtn.textContent = 'Generating...';
            regenerateBtn.disabled = true;
            
            // Show loading bar immediately and force UI update
            this.showLoadingBar('Generating grain...');
            
            // Force browser to render the loading bar before heavy computation
            await new Promise(resolve => setTimeout(resolve, 50));
            
            // Call Rust backend for high-performance grain generation
            const result = await invoke('generate_grain', { params });
            
            // Convert the raw data to ImageData and display
            this.displayGrainResult(result);
            
            // Update performance info
            this.updatePerformanceInfo(result);
            
            // Hide loading bar
            this.hideLoadingBar();
            
            regenerateBtn.textContent = originalText;
            regenerateBtn.disabled = false;
            
        } catch (error) {
            console.error('Error generating grain:', error);
            this.hideLoadingBar();
            regenerateBtn.disabled = false;
            alert('Error generating grain: ' + error);
        }
    }
    
    showLoadingBar(text = 'Loading...') {
        const loadingBar = document.getElementById('loadingBar');
        const loadingText = document.getElementById('loadingText');
        const loadingProgress = document.getElementById('loadingProgress');
        
        loadingText.textContent = text;
        loadingProgress.style.width = '0%';
        loadingBar.style.display = 'block';
        
        // Force immediate display
        loadingBar.style.opacity = '1';
        
        // Start progress animation immediately
        requestAnimationFrame(() => {
            loadingProgress.style.width = '20%';
            
            setTimeout(() => {
                loadingProgress.style.width = '40%';
            }, 100);
            setTimeout(() => {
                loadingProgress.style.width = '70%';
            }, 200);
            setTimeout(() => {
                loadingProgress.style.width = '85%';
            }, 300);
        });
    }
    
    hideLoadingBar() {
        const loadingBar = document.getElementById('loadingBar');
        const loadingProgress = document.getElementById('loadingProgress');
        
        // Complete the progress bar
        loadingProgress.style.width = '100%';
        
        // Hide after a brief moment
        setTimeout(() => {
            loadingBar.style.display = 'none';
        }, 200);
    }
    
    getGrainParameters() {
        return {
            film_stock: document.getElementById('filmStock').value,
            exposure_compensation: parseFloat(document.getElementById('grainIntensity').value),
            size_multiplier: parseFloat(document.getElementById('grainSize').value),
            contrast: parseFloat(document.getElementById('contrast').value),
            grain_density: parseInt(document.getElementById('grainDensity').value), // Direct multiplier value
            width: parseInt(document.getElementById('canvasWidth').value),
            height: parseInt(document.getElementById('canvasHeight').value),
            background: 'transparent',
            film_age_years: parseFloat(document.getElementById('filmAge')?.value || 0),
            storage_temp: parseFloat(document.getElementById('storageType')?.value || 20)
        };
    }
    
    displayGrainResult(result) {
        const { data, width, height } = result;
        
        // Set canvas size
        this.canvas.width = width;
        this.canvas.height = height;
        
        // Create ImageData from the raw RGBA data
        const imageData = new ImageData(new Uint8ClampedArray(data), width, height);
        
        // If there's an uploaded image, use it as background instead of black
        if (this.uploadedImage && this.uploadedImageElement) {
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
    }
    
    updatePerformanceInfo(result) {
        const info = document.getElementById('performanceInfo');
        info.innerHTML = `Generation time: ${result.generation_time_ms}ms | Grains: ${result.grain_count.toLocaleString()}`;
        
        // Add performance indicator
        const perfIndicator = result.generation_time_ms < 100 ? '🚀 GPU' : 
                             result.generation_time_ms < 500 ? '⚡ Fast' : 
                             result.generation_time_ms < 2000 ? '🔄 CPU' : '🐌 Slow';
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
                img.src = e.target.result;
            };
            reader.readAsDataURL(file);
            
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
                baseImageData: this.uploadedImage,
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