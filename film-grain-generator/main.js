import { invoke } from '@tauri-apps/api/core';

class FilmGrainGenerator {
    constructor() {
        this.canvas = document.getElementById('grainCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.currentImageData = null;
        this.updateTimeout = null;
        
        this.initializeControls();
    }
    
    async loadFilmStocks() {
        try {
            // Get available film stocks from backend
            const result = await invoke('get_available_film_stocks');
            const filmStockSelect = document.getElementById('filmStock');
            
            // Clear existing options
            filmStockSelect.innerHTML = '';
            
            // Add all film stocks
            result.forEach(stockName => {
                const option = document.createElement('option');
                option.value = stockName;
                option.textContent = stockName;
                filmStockSelect.appendChild(option);
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
            'grainIntensity', 'grainSize', 'contrast', 
            'canvasWidth', 'canvasHeight'
        ];
        
        // Max grain count selector
        const maxGrainCount = document.getElementById('maxGrainCount');
        maxGrainCount.addEventListener('change', () => this.regenerateGrain());
        
        
        sliders.forEach(sliderId => {
            const slider = document.getElementById(sliderId);
            const valueDisplay = document.getElementById(sliderId + 'Value');
            
            slider.addEventListener('input', (e) => {
                let displayValue = e.target.value;
                if (sliderId === 'grainIntensity' || sliderId === 'contrast') {
                    displayValue += '%';
                } else if (sliderId === 'grainSize') {
                    displayValue += 'x';
                }
                valueDisplay.textContent = displayValue;
                
                if (sliderId === 'canvasWidth' || sliderId === 'canvasHeight') {
                    this.updateCanvasSize();
                }
            });
            
            slider.addEventListener('change', () => this.regenerateGrain());
            
            // Real-time updates for immediate feedback
            if (sliderId !== 'canvasWidth' && sliderId !== 'canvasHeight') {
                slider.addEventListener('input', () => {
                    clearTimeout(this.updateTimeout);
                    this.updateTimeout = setTimeout(() => this.regenerateGrain(), 300);
                });
            }
        });
        
        // Buttons
        document.getElementById('regenerateBtn').addEventListener('click', () => {
            this.regenerateGrain();
        });
        
        document.getElementById('saveBtn').addEventListener('click', () => {
            this.saveImage();
        });
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
            // Show loading state
            const regenerateBtn = document.getElementById('regenerateBtn');
            const originalText = regenerateBtn.textContent;
            regenerateBtn.textContent = 'Generating...';
            regenerateBtn.disabled = true;
            
            // Call Rust backend for high-performance grain generation
            const result = await invoke('generate_grain', { params });
            
            // Convert the raw data to ImageData and display
            this.displayGrainResult(result);
            
            // Update performance info
            this.updatePerformanceInfo(result);
            
            regenerateBtn.textContent = originalText;
            regenerateBtn.disabled = false;
            
        } catch (error) {
            console.error('Error generating grain:', error);
            alert('Error generating grain: ' + error);
        }
    }
    
    getGrainParameters() {
        return {
            film_stock: document.getElementById('filmStock').value,
            intensity: parseFloat(document.getElementById('grainIntensity').value),
            size_multiplier: parseFloat(document.getElementById('grainSize').value),
            contrast: parseFloat(document.getElementById('contrast').value),
            width: parseInt(document.getElementById('canvasWidth').value),
            height: parseInt(document.getElementById('canvasHeight').value),
            background: "transparent",
            max_grain_count: parseInt(document.getElementById('maxGrainCount')?.value || 20000)
        };
    }
    
    displayGrainResult(result) {
        const { data, width, height } = result;
        
        // Create ImageData from the raw RGBA data
        const imageData = new ImageData(new Uint8ClampedArray(data), width, height);
        
        // Clear canvas and draw black background for visibility
        this.ctx.fillStyle = '#000000';
        this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        
        // Draw the transparent grain on top
        this.ctx.putImageData(imageData, 0, 0);
        
        // Store original transparent data for saving
        this.currentImageData = data;
    }
    
    updatePerformanceInfo(result) {
        const info = document.getElementById('performanceInfo');
        info.textContent = `Generation time: ${result.generation_time_ms}ms | Grains: ${result.grain_count.toLocaleString()}`;
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
}

// Initialize the application when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    new FilmGrainGenerator();
});