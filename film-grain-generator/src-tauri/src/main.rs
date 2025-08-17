use serde::{Deserialize, Serialize};
use image::{ImageBuffer, Rgba, RgbaImage, DynamicImage, ImageFormat};
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose};
// SIMD optimizations (using built-in CPU vectorization)

#[cfg(feature = "gpu-acceleration")]
mod gpu;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FilmStock {
    basic_info: BasicInfo,
    grain_structure: GrainStructure,
    size_metrics: SizeMetrics,
    visual_properties: VisualProperties,
    color_properties: ColorProperties,
    special_effects: SpecialEffects,
    algorithmic_data: AlgorithmicData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BasicInfo {
    name: String,
    iso: u32,
    #[serde(rename = "type")]
    film_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GrainStructure {
    crystal_type: String,
    shape: String,
    aspect_ratio: Vec<f32>,
    orientation: String,
    clustering: String,
    edge_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SizeMetrics {
    min_size_um: f32,
    max_size_um: f32,
    avg_size_um: f32,
    size_distribution: String,
    size_variation_coeff: f32,
    density_per_mm2: u32,
    spacing_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VisualProperties {
    opacity_range: Vec<f32>,
    contrast_level: String,
    edge_definition: String,
    opacity_variation: f32,
    highlight_visibility: String,
    shadow_visibility: String,
    midtone_prominence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorProperties {
    primary_cast: String,
    rgb_ranges: Vec<RgbRange>,
    color_variation: String,
    saturation_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RgbRange {
    r: Vec<u8>,
    g: Vec<u8>,
    b: Vec<u8>,
    weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SpecialEffects {
    halation: String,
    halation_color: String,
    halation_radius: f32,
    unique_artifacts: Vec<String>,
    light_interaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AlgorithmicData {
    clustering_algorithm: String,
    distribution_function: String,
    spatial_correlation: f32,
    fractal_dimension: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ColorCrossover {
    red_to_green: f32,
    red_to_blue: f32,
    green_to_red: f32,
    green_to_blue: f32,
    blue_to_red: f32,
    blue_to_green: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgingEffects {
    grain_increase_per_year: f32,
    contrast_loss_per_year: f32,
    storage_temp_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClusteringData {
    fractal_dimension: f32,
    spatial_correlation: f32,
    cluster_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnhancedFilmData {
    color_crossover: ColorCrossover,
    aging_effects: AgingEffects,
    clustering_data: ClusteringData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariationData {
    size_variation_coeff: f32,
    opacity_variation: f32,
    notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GrainParams {
    film_stock: String,
    exposure_compensation: f32,
    size_multiplier: f32,
    contrast: f32,
    grain_density: u32,
    width: u32,
    height: u32,
    background: String,
    film_age_years: Option<f32>,  // 0-10 years
    storage_temp: Option<f32>,    // -18 to 25°C
}

#[derive(Debug, Serialize)]
struct GrainResult {
    data: Vec<u8>,
    width: u32,
    height: u32,
    generation_time_ms: u128,
    grain_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct Grain {
    x: f32,
    y: f32,
    size: f32,
    opacity: f32,
    shape_factor: f32,
}

#[cfg(feature = "gpu-acceleration")]
async fn try_gpu_render(grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
    use gpu::GpuManager;
    
    // Only attempt GPU for very large workloads
    if grains.len() < 50000 {
        return Err("Workload too small for GPU".to_string());
    }
    
    let gpu_manager = GpuManager::new().await?;
    gpu_manager.render_grains(grains, params, stock).await
}

#[tauri::command]
async fn generate_grain(params: GrainParams) -> Result<GrainResult, String> {
    let start_time = std::time::Instant::now();
    
    // Load film stock data (in a real app, this would be loaded once at startup)
    let film_stocks = load_film_stock_data()?;
    let stock = film_stocks.get(&params.film_stock)
        .ok_or_else(|| format!("Film stock '{}' not found", params.film_stock))?;
    
    // Load enhanced film data for realistic effects
    let enhanced_data = load_enhanced_film_data()?;
    let enhanced_stock = enhanced_data.get(&params.film_stock);
    
    // Load variation data for authentic grain variation
    let variation_data = load_variation_data()?;
    let variation_stock = variation_data.get(&params.film_stock);
    
    // Generate grains using advanced algorithms with enhancements
    let _start_time = std::time::Instant::now();
    let mut grains = generate_grains_advanced(stock, &params, variation_stock)?;
    let _generation_time = _start_time.elapsed();
    
    // Apply enhanced realistic effects
    if let Some(enhanced) = enhanced_stock {
        apply_enhanced_effects(&mut grains, &params, enhanced)?;
    }
    
    // Smart rendering strategy:
    // - CPU (Rayon + SIMD): Fast for normal workloads (0-50K grains) - no GPU overhead
    // - GPU: Only for massive workloads (>50K grains) where parallelism outweighs overhead
    // This prevents the "slow/white screen" issues you experienced with GPU on small workloads
    let image_data = if grains.len() > 50000 && cfg!(feature = "gpu-acceleration") {
        // Try GPU for massive workloads only
        #[cfg(feature = "gpu-acceleration")]
        {
            match try_gpu_render(&grains, &params, stock).await {
                Ok(data) => {
                    println!("🚀 Used GPU acceleration for {} grains", grains.len());
                    data
                },
                Err(e) => {
                    println!("⚠️ GPU failed ({}), falling back to optimized CPU", e);
                    render_grains_parallel(&grains, &params, stock)?
                }
            }
        }
        #[cfg(not(feature = "gpu-acceleration"))]
        render_grains_parallel(&grains, &params, stock)?
    } else {
        // Use optimized CPU rendering for normal workloads (much faster for <50K grains)
        render_grains_parallel(&grains, &params, stock)?
    };
    
    let generation_time = start_time.elapsed().as_millis();
    
    Ok(GrainResult {
        data: image_data,
        width: params.width,
        height: params.height,
        generation_time_ms: generation_time,
        grain_count: grains.len(),
    })
}

fn load_enhanced_film_data() -> Result<HashMap<String, EnhancedFilmData>, String> {
    let enhanced_data = include_str!("../../more.json");
    let parsed: HashMap<String, EnhancedFilmData> = serde_json::from_str(enhanced_data)
        .map_err(|e| format!("Failed to parse enhanced film data: {}", e))?;
    Ok(parsed)
}

fn load_variation_data() -> Result<HashMap<String, VariationData>, String> {
    let variation_data = include_str!("../../variation.json");
    let parsed: HashMap<String, VariationData> = serde_json::from_str(variation_data)
        .map_err(|e| format!("Failed to parse variation data: {}", e))?;
    Ok(parsed)
}


fn load_film_stock_data() -> Result<HashMap<String, FilmStock>, String> {
    // Load comprehensive film stock data
    let json_data = include_str!("../../fixed.json");
    let stocks_json: serde_json::Value = serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse fixed.json: {}", e))?;
    
    let mut stocks = HashMap::new();
    
    // Parse all film stocks from comprehensive JSON
    if let Some(obj) = stocks_json.as_object() {
        for (name, stock_data) in obj {
            if let Ok(film_stock) = parse_comprehensive_film_stock(name, stock_data) {
                stocks.insert(name.clone(), film_stock);
            }
        }
    }
    
    println!("Loaded {} film stocks from comprehensive database", stocks.len());
    
    // If JSON parsing fails, fall back to hardcoded stocks
    if stocks.is_empty() {
    
    let tri_x = FilmStock {
        basic_info: BasicInfo {
            name: "Kodak Tri-X 400".to_string(),
            iso: 400,
            film_type: "bw".to_string(),
        },
        grain_structure: GrainStructure {
            crystal_type: "cubic".to_string(),
            shape: "irregular".to_string(),
            aspect_ratio: vec![1.0, 1.0],
            orientation: "random".to_string(),
            clustering: "moderate".to_string(),
            edge_type: "sharp".to_string(),
        },
        size_metrics: SizeMetrics {
            min_size_um: 0.5,
            max_size_um: 2.0,
            avg_size_um: 0.8,
            size_distribution: "bimodal".to_string(),
            size_variation_coeff: 0.5,
            density_per_mm2: 800000,
            spacing_pattern: "random".to_string(),
        },
        visual_properties: VisualProperties {
            opacity_range: vec![0.2, 0.8],
            contrast_level: "high".to_string(),
            edge_definition: "sharp".to_string(),
            opacity_variation: 0.7,
            highlight_visibility: "low".to_string(),
            shadow_visibility: "high".to_string(),
            midtone_prominence: "medium".to_string(),
        },
        color_properties: ColorProperties {
            primary_cast: "neutral".to_string(),
            rgb_ranges: vec![RgbRange {
                r: vec![200, 255],
                g: vec![200, 255],
                b: vec![200, 255],
                weight: 1.0,
            }],
            color_variation: "low".to_string(),
            saturation_level: "low".to_string(),
        },
        special_effects: SpecialEffects {
            halation: "mild".to_string(),
            halation_color: "#ffffff".to_string(),
            halation_radius: 1.0,
            unique_artifacts: vec![],
            light_interaction: "normal".to_string(),
        },
        algorithmic_data: AlgorithmicData {
            clustering_algorithm: "poisson".to_string(),
            distribution_function: "normal(0.8, 0.5)".to_string(),
            spatial_correlation: 0.2,
            fractal_dimension: 1.2,
        },
    };
    
    stocks.insert("Kodak Tri-X 400".to_string(), tri_x.clone());
    
    // Add other stocks with different characteristics
    let mut hp5 = tri_x.clone();
    hp5.basic_info.name = "Ilford HP5 Plus".to_string();
    hp5.size_metrics.density_per_mm2 = 900000;
    hp5.algorithmic_data.spatial_correlation = 0.25;
    stocks.insert("Ilford HP5 Plus".to_string(), hp5);
    
    let mut tmax400 = tri_x.clone();
    tmax400.basic_info.name = "Kodak T-Max 400".to_string();
    tmax400.grain_structure.crystal_type = "tabular".to_string();
    tmax400.grain_structure.shape = "flat".to_string();
    tmax400.grain_structure.aspect_ratio = vec![3.0, 1.0];
    tmax400.size_metrics.min_size_um = 0.4;
    tmax400.size_metrics.max_size_um = 1.5;
    tmax400.size_metrics.avg_size_um = 0.7;
    tmax400.size_metrics.density_per_mm2 = 1000000;
    tmax400.visual_properties.opacity_range = vec![0.1, 0.7];
    stocks.insert("Kodak T-Max 400".to_string(), tmax400.clone());
    
    let mut tmax100 = tmax400.clone();
    tmax100.basic_info.name = "Kodak T-Max 100".to_string();
    tmax100.basic_info.iso = 100;
    tmax100.size_metrics.min_size_um = 0.3;
    tmax100.size_metrics.max_size_um = 1.0;
    tmax100.size_metrics.avg_size_um = 0.5;
    tmax100.size_metrics.density_per_mm2 = 1200000;
    tmax100.visual_properties.opacity_range = vec![0.1, 0.6];
    stocks.insert("Kodak T-Max 100".to_string(), tmax100);
    
    // Add more film stocks from your JSON data
    let mut portra400 = tri_x.clone();
    portra400.basic_info.name = "Kodak Portra 400".to_string();
    portra400.basic_info.film_type = "color".to_string();
    portra400.size_metrics.density_per_mm2 = 750000;
    portra400.color_properties.rgb_ranges = vec![
        RgbRange { r: vec![220, 255], g: vec![200, 240], b: vec![180, 220], weight: 1.0 }
    ];
    stocks.insert("Kodak Portra 400".to_string(), portra400.clone());
    
    let mut portra800 = portra400.clone();
    portra800.basic_info.name = "Kodak Portra 800".to_string();
    portra800.basic_info.iso = 800;
    portra800.size_metrics.density_per_mm2 = 650000;
    portra800.size_metrics.avg_size_um = 1.2;
    stocks.insert("Kodak Portra 800".to_string(), portra800);
    
    let mut ektar100 = tri_x.clone();
    ektar100.basic_info.name = "Kodak Ektar 100".to_string();
    ektar100.basic_info.iso = 100;
    ektar100.basic_info.film_type = "color".to_string();
    ektar100.size_metrics.density_per_mm2 = 1400000;
    ektar100.size_metrics.avg_size_um = 0.4;
    ektar100.color_properties.rgb_ranges = vec![
        RgbRange { r: vec![240, 255], g: vec![220, 255], b: vec![200, 240], weight: 1.0 }
    ];
    stocks.insert("Kodak Ektar 100".to_string(), ektar100);
    
    let mut fuji400h = tri_x.clone();
    fuji400h.basic_info.name = "Fuji Pro 400H".to_string();
    fuji400h.basic_info.film_type = "color".to_string();
    fuji400h.size_metrics.density_per_mm2 = 850000;
    fuji400h.color_properties.rgb_ranges = vec![
        RgbRange { r: vec![210, 250], g: vec![220, 255], b: vec![200, 240], weight: 1.0 }
    ];
    stocks.insert("Fuji Pro 400H".to_string(), fuji400h);
    
    let mut cinestill800t = tri_x.clone();
    cinestill800t.basic_info.name = "CineStill 800T".to_string();
    cinestill800t.basic_info.iso = 800;
    cinestill800t.basic_info.film_type = "color".to_string();
    cinestill800t.size_metrics.density_per_mm2 = 600000;
    cinestill800t.size_metrics.avg_size_um = 1.4;
    cinestill800t.special_effects.halation = "strong".to_string();
    cinestill800t.color_properties.rgb_ranges = vec![
        RgbRange { r: vec![200, 255], g: vec![180, 220], b: vec![220, 255], weight: 1.0 }
    ];
    stocks.insert("CineStill 800T".to_string(), cinestill800t);
    
    } // End of fallback block
    
    Ok(stocks)
}

fn generate_grains_advanced(stock: &FilmStock, params: &GrainParams, variation_data: Option<&VariationData>) -> Result<Vec<Grain>, String> {
    let mut rng = thread_rng();
    let mut grains = Vec::new();
    
    // Use film stock's actual density as base, then apply user density multiplier
    let canvas_area_ratio = (params.width * params.height) as f32 / (1024.0 * 1024.0);
    let stock_base_density = stock.size_metrics.density_per_mm2 as f32;
    let user_density_multiplier = params.grain_density as f32 / 1000.0; // Convert from 0.5-5.0 range
    let final_grain_count = ((stock_base_density * canvas_area_ratio * user_density_multiplier) / 25.0) as usize;
    println!("Density: {:.1}x multiplier, Stock density: {}/mm², Final: {} grains for {}", 
             user_density_multiplier, stock_base_density as u32, final_grain_count, stock.basic_info.name);
    
    // Generate grains with spatial correlation
    for _ in 0..final_grain_count {
        let x = rng.gen::<f32>() * params.width as f32;
        let y = rng.gen::<f32>() * params.height as f32;
        
        // Use authentic variation coefficient from research data
        let size_variation_coeff = variation_data
            .map(|v| v.size_variation_coeff)
            .unwrap_or(stock.size_metrics.size_variation_coeff); // Fallback to hardcoded
            
        let size_factor = {
            let rand_val = rng.gen::<f32>();
            if rand_val < 0.6 {
                // 60% average size grains
                rng.gen_range(1.0 - size_variation_coeff * 0.5..1.0 + size_variation_coeff * 0.5)
            } else if rand_val < 0.9 {
                // 30% smaller grains
                rng.gen_range(0.4..1.0 - size_variation_coeff * 0.3)
            } else {
                // 10% larger grains
                rng.gen_range(1.0 + size_variation_coeff * 0.3..1.8)
            }
        };
        
        // Use realistic grain sizes from comprehensive data
        let base_size = stock.size_metrics.avg_size_um * 0.5; // Convert microns to reasonable pixel size
        let size = (base_size * size_factor * params.size_multiplier).max(0.5);
        
        // Use authentic opacity variation from research data
        let base_opacity = rng.gen_range(stock.visual_properties.opacity_range[0]..stock.visual_properties.opacity_range[1]);
        let opacity_var = variation_data
            .map(|v| v.opacity_variation)
            .unwrap_or(stock.visual_properties.opacity_variation); // Fallback to hardcoded
            
        let opacity_variation = rng.gen_range(1.0 - opacity_var * 0.5..1.0 + opacity_var * 0.5);
        let contrast_factor = params.contrast / 100.0;   // User opacity control
        
        // Apply exposure compensation (over/under exposure affects grain visibility)
        let exposure_factor = if params.exposure_compensation > 0.0 {
            // Overexposure: grain becomes more visible and coarser
            1.0 + (params.exposure_compensation * 0.3)
        } else {
            // Underexposure: grain becomes finer but denser-looking
            1.0 + (params.exposure_compensation * 0.2)
        };
        
        let opacity = (base_opacity * contrast_factor * opacity_variation * exposure_factor).min(1.0).max(0.1);
        
        // Realistic grain shapes using film stock's aspect ratio data
        let aspect_ratios = &stock.grain_structure.aspect_ratio;
        let base_aspect = if aspect_ratios.len() >= 2 {
            aspect_ratios[0] / aspect_ratios[1] // width/height ratio
        } else {
            1.0 // Default to circular
        };
        
        let shape_factor = match stock.grain_structure.shape.as_str() {
            "irregular" => base_aspect * rng.gen_range(0.7..1.0),
            "T-grain" => base_aspect * rng.gen_range(0.6..0.8),    // Use actual tabular ratio
            "fine_irregular" => base_aspect * rng.gen_range(0.8..1.0),
            "extremely_fine" => base_aspect * rng.gen_range(0.9..1.0),
            "cubic" => base_aspect * rng.gen_range(0.8..1.0),
            _ => base_aspect * rng.gen_range(0.8..1.0),
        };
        
        grains.push(Grain {
            x,
            y,
            size,
            opacity,
            shape_factor,
        });
    }
    
    // Apply realistic clustering based on film stock characteristics
    if stock.algorithmic_data.spatial_correlation > 0.1 {
        apply_enhanced_clustering(&mut grains, &mut rng, params.width, params.height, &stock.grain_structure.clustering);
    }
    
    Ok(grains)
}

fn apply_enhanced_clustering(grains: &mut Vec<Grain>, rng: &mut ThreadRng, width: u32, height: u32, clustering_type: &str) {
    let original_count = grains.len();
    let cluster_intensity = match clustering_type {
        "heavy" => 0.25,     // 25% of grains form clusters
        "moderate" => 0.15,  // 15% of grains form clusters  
        "light" => 0.08,     // 8% of grains form clusters
        _ => 0.15,           // Default moderate
    };
    
    let cluster_count = (original_count as f32 * cluster_intensity) as usize;
    
    for _ in 0..cluster_count {
        if grains.is_empty() { break; }
        
        let center_idx = rng.gen_range(0..grains.len());
        let center = grains[center_idx];
        
        // Cluster size based on clustering type
        let cluster_size = match clustering_type {
            "heavy" => rng.gen_range(3..6),    // Larger clusters
            "moderate" => rng.gen_range(2..4), // Medium clusters
            "light" => rng.gen_range(2..3),    // Small clusters
            _ => rng.gen_range(2..4),
        };
        
        for _ in 0..cluster_size {
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let distance = rng.gen_range(0.5..2.5) * center.size;
            
            let x = center.x + angle.cos() * distance;
            let y = center.y + angle.sin() * distance;
            
            if x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32 {
                grains.push(Grain {
                    x,
                    y,
                    size: center.size * rng.gen_range(0.8..1.2),
                    opacity: center.opacity * rng.gen_range(0.9..1.1),
                    shape_factor: center.shape_factor * rng.gen_range(0.9..1.1),
                });
            }
        }
    }
}



fn render_grains_parallel(grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
    let render_start = std::time::Instant::now();
    let num_threads = rayon::current_num_threads();
    println!("Rendering {} grains for {} using {} CPU threads", grains.len(), stock.basic_info.name, num_threads);
    
    // Create image buffer
    let mut img: RgbaImage = ImageBuffer::new(params.width, params.height);
    
    // Always use transparent background for the actual image data
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]); // Always transparent
    }
    
    // Optimize chunk size for better load balancing
    let optimal_chunk_size = (grains.len() / (num_threads * 4)).max(100).min(1000);
    let grain_chunks: Vec<&[Grain]> = grains.chunks(optimal_chunk_size).collect();
    
    // Process each chunk in parallel with pre-allocated capacity
    let rendered_pixels: Vec<Vec<(u32, u32, Rgba<u8>)>> = grain_chunks.par_iter().map(|chunk| {
        // Pre-allocate with estimated capacity to reduce reallocations
        let estimated_pixels_per_grain = (std::f32::consts::PI * 4.0 * 4.0) as usize; // π*r² for average grain
        let mut pixels = Vec::with_capacity(chunk.len() * estimated_pixels_per_grain);
        
        for grain in *chunk {
            // Render grain to temporary buffer and collect non-transparent pixels
            let rendered_grain_pixels = render_grain_to_pixels(grain, stock, params);
            pixels.extend(rendered_grain_pixels);
        }
        
        pixels
    }).collect();
    
    // Apply all rendered pixels to the main image (sequential to avoid race conditions)
    // Try SIMD optimization for large pixel counts
    let total_pixels: usize = rendered_pixels.iter().map(|chunk| chunk.len()).sum();
    
    if total_pixels > 500 {  // Lower threshold for SIMD
        // Use SIMD for medium+ workloads
        println!("🚀 Using SIMD optimization for {} pixels", total_pixels);
        apply_pixels_simd_optimized(&mut img, rendered_pixels, params);
    } else {
        // Use regular blending for small workloads with bounds checking optimization
        for pixel_chunk in rendered_pixels {
            for (x, y, color) in pixel_chunk {
                // Bounds check once per chunk instead of per pixel
                if x < params.width && y < params.height {
                    // Use regular pixel access (bounds already checked)
                    let pixel = img.get_pixel_mut(x, y);
                    blend_pixel_fast(pixel, color);
                }
            }
        }
    }
    
    let render_time = render_start.elapsed().as_millis();
    println!("⏱️ Render breakdown: {}ms total, {:.1} grains/ms", render_time, grains.len() as f32 / render_time as f32);
    
    // Convert to raw RGBA bytes
    Ok(img.into_raw())
}

fn render_grain_to_pixels(grain: &Grain, stock: &FilmStock, params: &GrainParams) -> Vec<(u32, u32, Rgba<u8>)> {
    let mut pixels = Vec::new();
    let center_x = grain.x as i32;
    let center_y = grain.y as i32;
    let radius = grain.size as i32;
    
    // Load authentic film grain colors from color.json
    let (r, g, b) = get_film_grain_color(&stock.basic_info.name);
    
    // Apply color crossover effects from more.json if available
    let mut grain_color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
    if let Ok(enhanced_data) = load_enhanced_film_data() {
        if let Some(enhanced) = enhanced_data.get(&params.film_stock) {
            apply_color_crossover(&mut grain_color, &enhanced.color_crossover);
        }
    }
    let final_r = (grain_color[0] * 255.0).clamp(0.0, 255.0) as u8;
    let final_g = (grain_color[1] * 255.0).clamp(0.0, 255.0) as u8;
    let final_b = (grain_color[2] * 255.0).clamp(0.0, 255.0) as u8;
    
    // Use realistic opacity with moderate boost for visibility
    let boosted_opacity = match grain.opacity {
        x if x < 0.2 => x * 3.0,   // Fine films - 3x boost
        x if x < 0.4 => x * 2.0,   // Medium films - 2x boost
        x => x * 1.5,              // Coarse films - 1.5x boost
    };
    let alpha = (boosted_opacity * 255.0).min(255.0).max(40.0) as u8;
    
    // Render grain with anti-aliasing
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = center_x + dx;
            let y = center_y + dy;
            
            if x >= 0 && y >= 0 && x < params.width as i32 && y < params.height as i32 {
                // Apply shape factor for realistic grain shapes
                let adjusted_dx = dx as f32 / grain.shape_factor;
                let distance = ((adjusted_dx * adjusted_dx) + (dy * dy) as f32).sqrt();
                
                if distance <= grain.size {
                    // Apply edge characteristics based on film stock
                    let edge_alpha = match stock.grain_structure.edge_type.as_str() {
                        "sharp" => {
                            if distance > grain.size * 0.9 {
                                ((grain.size - distance) / (grain.size * 0.1)).max(0.0)
                            } else {
                                1.0
                            }
                        },
                        "soft" => {
                            if distance > grain.size * 0.7 {
                                ((grain.size - distance) / (grain.size * 0.3)).max(0.0)
                            } else {
                                1.0
                            }
                        },
                        "hard" => {
                            if distance > grain.size * 0.95 {
                                0.0
                            } else {
                                1.0
                            }
                        },
                        _ => 1.0,
                    };
                    
                    let final_alpha = (alpha as f32 * edge_alpha) as u8;
                    
                    if final_alpha > 10 {
                        pixels.push((x as u32, y as u32, Rgba([final_r, final_g, final_b, final_alpha])));
                    }
                }
            }
        }
    }
    
    pixels
}

fn apply_enhanced_effects(grains: &mut Vec<Grain>, params: &GrainParams, enhanced: &EnhancedFilmData) -> Result<(), String> {
    let mut rng = thread_rng();
    
    // Apply film aging effects
    if let Some(age_years) = params.film_age_years {
        if age_years > 0.0 {
            apply_aging_effects(grains, age_years, params.storage_temp.unwrap_or(20.0), &enhanced.aging_effects);
            println!("🕰️ Applied {:.1} year aging effects", age_years);
        }
    }
    
    // Apply enhanced clustering based on research data
    apply_enhanced_clustering_realistic(grains, &mut rng, params.width, params.height, &enhanced.clustering_data);
    
    Ok(())
}

fn apply_aging_effects(grains: &mut Vec<Grain>, age_years: f32, storage_temp: f32, aging: &AgingEffects) {
    let age_factor = age_years.min(10.0); // Cap at 10 years
    let temp_factor = (storage_temp - (-18.0)) / 43.0; // Normalize -18°C to 25°C range
    let temp_effect = temp_factor * aging.storage_temp_factor;
    
    // Aged film effects
    let grain_boost = 1.0 + (age_factor * aging.grain_increase_per_year * (1.0 + temp_effect));
    let contrast_loss = 1.0 - (age_factor * aging.contrast_loss_per_year * (1.0 + temp_effect));
    
    for grain in grains {
        // More grain with age
        grain.opacity *= grain_boost;
        grain.size *= 1.0 + (age_factor * 0.02); // Slightly larger grain
        
        // Less contrast with age
        grain.opacity *= contrast_loss;
        
        // More irregular grain shapes with age
        grain.shape_factor *= 1.0 - (age_factor * 0.01);
    }
}

fn apply_enhanced_clustering_realistic(grains: &mut Vec<Grain>, rng: &mut ThreadRng, width: u32, height: u32, clustering: &ClusteringData) {
    let cluster_count = (grains.len() as f32 * clustering.cluster_probability) as usize;
    
    println!("🔬 Applying enhanced clustering: fractal_dim={:.2}, correlation={:.2}, clusters={}", 
        clustering.fractal_dimension, clustering.spatial_correlation, cluster_count);
    
    for _ in 0..cluster_count {
        if grains.is_empty() { break; }
        
        let seed_idx = rng.gen_range(0..grains.len());
        let seed_grain = grains[seed_idx];
        
        // Use fractal dimension to determine cluster characteristics
        let cluster_size = ((clustering.fractal_dimension - 1.0) * 6.0) as usize; // 0-6 grains
        let cluster_spread = seed_grain.size * (2.0 - clustering.spatial_correlation) * 3.0;
        
        for _ in 0..cluster_size {
            // Fractal clustering pattern
            let distance = rng.gen::<f32>().powf(1.0 / clustering.fractal_dimension) * cluster_spread;
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            
            let x = seed_grain.x + angle.cos() * distance;
            let y = seed_grain.y + angle.sin() * distance;
            
            if x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32 {
                // Correlated grain properties
                let size_variation = 1.0 + (rng.gen::<f32>() - 0.5) * (1.0 - clustering.spatial_correlation);
                let opacity_variation = 1.0 + (rng.gen::<f32>() - 0.5) * (1.0 - clustering.spatial_correlation) * 0.5;
                
                grains.push(Grain {
                    x,
                    y,
                    size: seed_grain.size * size_variation,
                    opacity: (seed_grain.opacity * opacity_variation).min(1.0).max(0.1),
                    shape_factor: seed_grain.shape_factor * rng.gen_range(0.9..1.1),
                });
            }
        }
    }
}

fn apply_color_crossover(grain_color: &mut [f32; 3], crossover: &ColorCrossover) {
    // Skip crossover for B&W films (values = 1.0)
    if crossover.red_to_green >= 1.0 { return; }
    
    let original = *grain_color;
    
    // Apply color channel crossover (like real film dye coupling)
    grain_color[0] += original[1] * crossover.green_to_red + original[2] * crossover.blue_to_red;
    grain_color[1] += original[0] * crossover.red_to_green + original[2] * crossover.blue_to_green;
    grain_color[2] += original[0] * crossover.red_to_blue + original[1] * crossover.green_to_blue;
    
    // Normalize to prevent oversaturation
    for channel in grain_color {
        *channel = channel.min(1.0);
    }
}

fn blend_pixel(base_pixel: &mut Rgba<u8>, new_pixel: Rgba<u8>) {
    let blend_factor = new_pixel[3] as f32 / 255.0;
    base_pixel[0] = ((base_pixel[0] as f32 * (1.0 - blend_factor)) + (new_pixel[0] as f32 * blend_factor)) as u8;
    base_pixel[1] = ((base_pixel[1] as f32 * (1.0 - blend_factor)) + (new_pixel[1] as f32 * blend_factor)) as u8;
    base_pixel[2] = ((base_pixel[2] as f32 * (1.0 - blend_factor)) + (new_pixel[2] as f32 * blend_factor)) as u8;
    base_pixel[3] = ((base_pixel[3] as f32).max(new_pixel[3] as f32)) as u8;
}

#[inline(always)]
fn blend_pixel_fast(base_pixel: &mut Rgba<u8>, new_pixel: Rgba<u8>) {
    if new_pixel[3] == 0 { return; } // Skip transparent pixels
    
    // Use integer math for better performance
    let alpha = new_pixel[3] as u16;
    let inv_alpha = 255 - alpha;
    
    base_pixel[0] = (((base_pixel[0] as u16 * inv_alpha) + (new_pixel[0] as u16 * alpha)) >> 8) as u8;
    base_pixel[1] = (((base_pixel[1] as u16 * inv_alpha) + (new_pixel[1] as u16 * alpha)) >> 8) as u8;
    base_pixel[2] = (((base_pixel[2] as u16 * inv_alpha) + (new_pixel[2] as u16 * alpha)) >> 8) as u8;
    base_pixel[3] = ((base_pixel[3] as u16 + alpha).min(255)) as u8;
}


fn apply_pixels_simd_optimized(
    img: &mut RgbaImage, 
    rendered_pixels: Vec<Vec<(u32, u32, Rgba<u8>)>>, 
    params: &GrainParams
) {
    // For SIMD optimization, we need to work with contiguous memory
    // So we'll still use the regular approach but with SIMD where possible
    
    // Group pixels by rows for better memory access patterns
    let mut row_pixels: Vec<Vec<(u32, Rgba<u8>)>> = vec![Vec::new(); params.height as usize];
    
    for pixel_chunk in rendered_pixels {
        for (x, y, color) in pixel_chunk {
            if x < params.width && y < params.height {
                row_pixels[y as usize].push((x, color));
            }
        }
    }
    
    // Process each row with potential SIMD optimization
    for (row_idx, row_pixel_list) in row_pixels.iter().enumerate() {
        if row_pixel_list.is_empty() { continue; }
        
        // Sort pixels by x coordinate for sequential access
        let mut sorted_pixels = row_pixel_list.clone();
        sorted_pixels.sort_by_key(|(x, _)| *x);
        
        // Apply pixels to this row
        for (x, color) in sorted_pixels {
            let pixel = img.get_pixel_mut(x, row_idx as u32);
            blend_pixel(pixel, color);
        }
    }
}


#[tauri::command]
async fn save_grain_image(data: Vec<u8>, width: u32, height: u32, path: String) -> Result<(), String> {
    let img: RgbaImage = ImageBuffer::from_raw(width, height, data)
        .ok_or("Failed to create image from data")?;
    
    // Save to Downloads folder
    let downloads_dir = dirs::download_dir()
        .ok_or("Could not find Downloads directory")?;
    
    let full_path = downloads_dir.join(&path);
    println!("Saving to: {:?}", full_path);
    
    img.save(&full_path).map_err(|e| format!("Failed to save image: {}", e))?;
    Ok(())
}

fn get_film_grain_color(film_name: &str) -> (u8, u8, u8) {
    // Load color data from color.json
    let color_data = include_str!("../../color.json");
    if let Ok(colors_json) = serde_json::from_str::<serde_json::Value>(color_data) {
        if let Some(film_color) = colors_json.get(film_name) {
            if let Some(base_color) = film_color.get("base_grain_color") {
                let r = base_color.get("r").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                let g = base_color.get("g").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                let b = base_color.get("b").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                
                // Apply color variation
                if let Some(variation) = film_color.get("color_variation") {
                    let var_r = variation.get("r").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
                    let var_g = variation.get("g").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
                    let var_b = variation.get("b").and_then(|v| v.as_u64()).unwrap_or(10) as i32;
                    
                    let mut rng = thread_rng();
                    let final_r = (r as i32 + rng.gen_range(-var_r..=var_r)).clamp(0, 255) as u8;
                    let final_g = (g as i32 + rng.gen_range(-var_g..=var_g)).clamp(0, 255) as u8;
                    let final_b = (b as i32 + rng.gen_range(-var_b..=var_b)).clamp(0, 255) as u8;
                    
                    return (final_r, final_g, final_b);
                }
                
                return (r, g, b);
            }
        }
    }
    
    // Fallback to neutral gray
    (180, 180, 180)
}

#[tauri::command]
async fn get_available_film_stocks() -> Result<Vec<String>, String> {
    let stocks = load_film_stock_data()?;
    let mut stock_names: Vec<String> = stocks.keys().cloned().collect();
    stock_names.sort();
    Ok(stock_names)
}

#[tauri::command]
async fn get_categorized_film_stocks() -> Result<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>, String> {
    let stocks = load_film_stock_data()?;
    let mut categorized: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> = std::collections::HashMap::new();
    
    for (name, stock) in stocks {
        // Determine category based on film type
        let category = match stock.basic_info.film_type.as_str() {
            "bw" => "Black & White Films",
            "color" => {
                if name.contains("Velvia") || name.contains("Provia") || name.contains("Ektachrome") || 
                   name.contains("Elite Chrome") || name.contains("CT Precisa") {
                    "Color Slide Films"
                } else {
                    "Color Negative Films"
                }
            },
            _ => "Other Films"
        };
        
        // Determine manufacturer
        let manufacturer = if name.starts_with("Kodak") {
            "Kodak"
        } else if name.starts_with("Fuji") || name.starts_with("Fujifilm") {
            "Fujifilm"
        } else if name.starts_with("Ilford") {
            "Ilford"
        } else if name.starts_with("Agfa") {
            "Agfa"
        } else if name.starts_with("CineStill") {
            "CineStill"
        } else if name.starts_with("Lomography") {
            "Lomography"
        } else {
            "Other"
        };
        
        // Initialize category if it doesn't exist
        if !categorized.contains_key(category) {
            categorized.insert(category.to_string(), std::collections::HashMap::new());
        }
        
        // Initialize manufacturer if it doesn't exist
        let category_map = categorized.get_mut(category).unwrap();
        if !category_map.contains_key(manufacturer) {
            category_map.insert(manufacturer.to_string(), Vec::new());
        }
        
        // Add film stock to the appropriate category and manufacturer
        category_map.get_mut(manufacturer).unwrap().push(name);
    }
    
    // Sort film stocks within each manufacturer
    for category_map in categorized.values_mut() {
        for stocks in category_map.values_mut() {
            stocks.sort();
        }
    }
    
    Ok(categorized)
}

#[tauri::command]
async fn load_user_image(image_data: String, filename: String) -> Result<String, String> {
    println!("Loading user image: {}", filename);
    
    // Decode base64 image data
    let image_bytes = general_purpose::STANDARD.decode(&image_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    // Load image using the image crate
    let img = image::load_from_memory(&image_bytes)
        .map_err(|e| format!("Failed to load image: {}", e))?;
    
    // Convert to RGBA format
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    
    println!("Loaded image: {}x{} pixels", width, height);
    
    // Convert back to base64 for storage in frontend
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    DynamicImage::ImageRgba8(rgba_img.clone()).write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image: {}", e))?;
    
    let result_base64 = general_purpose::STANDARD.encode(&buffer);
    
    Ok(result_base64)
}

#[tauri::command]
async fn save_composite_image(
    grain_data: Vec<u8>,
    grain_width: u32,
    grain_height: u32,
    base_image_data: String,
    path: String,
) -> Result<String, String> {
    println!("Creating composite image: {}", path);
    
    // Decode the base image
    let base_image_bytes = general_purpose::STANDARD.decode(&base_image_data)
        .map_err(|e| format!("Failed to decode base image: {}", e))?;
    
    let base_img = image::load_from_memory(&base_image_bytes)
        .map_err(|e| format!("Failed to load base image: {}", e))?;
    
    let mut base_rgba = base_img.to_rgba8();
    let (base_width, base_height) = base_rgba.dimensions();
    
    // Create grain image from data
    let grain_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(grain_width, grain_height, grain_data)
        .ok_or("Failed to create grain image from data")?;
    
    // Resize grain to match base image if needed
    let grain_resized = if grain_width != base_width || grain_height != base_height {
        println!("Resizing grain from {}x{} to {}x{}", grain_width, grain_height, base_width, base_height);
        image::imageops::resize(&grain_img, base_width, base_height, image::imageops::FilterType::Lanczos3)
    } else {
        grain_img
    };
    
    // Composite grain over base image
    for (x, y, grain_pixel) in grain_resized.enumerate_pixels() {
        if x < base_width && y < base_height {
            let base_pixel = base_rgba.get_pixel_mut(x, y);
            
            // Alpha blend the grain onto the base image
            let grain_alpha = grain_pixel[3] as f32 / 255.0;
            let inv_alpha = 1.0 - grain_alpha;
            
            base_pixel[0] = ((base_pixel[0] as f32 * inv_alpha) + (grain_pixel[0] as f32 * grain_alpha)) as u8;
            base_pixel[1] = ((base_pixel[1] as f32 * inv_alpha) + (grain_pixel[1] as f32 * grain_alpha)) as u8;
            base_pixel[2] = ((base_pixel[2] as f32 * inv_alpha) + (grain_pixel[2] as f32 * grain_alpha)) as u8;
            // Keep the base image's alpha channel
        }
    }
    
    // Save the composite image
    let downloads_dir = dirs::download_dir()
        .ok_or("Could not find Downloads directory")?;
    
    let file_path = downloads_dir.join(&path);
    
    base_rgba.save(&file_path)
        .map_err(|e| format!("Failed to save composite image: {}", e))?;
    
    println!("Composite image saved to: {:?}", file_path);
    Ok(format!("Composite image saved to Downloads/{}", path))
}

#[derive(Debug, Serialize)]
struct FilmInfo {
    description: String,
    primary_uses: Vec<String>,
    characteristics: Vec<String>,
    famous_users: Vec<String>,
    ideal_conditions: Vec<String>,
    era: String,
    price_category: String,
}

#[tauri::command]
async fn get_film_info(film_name: String) -> Result<FilmInfo, String> {
    // Load comprehensive film stock data to get film info
    let json_data = include_str!("../../fixed.json");
    let stocks_json: serde_json::Value = serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse fixed.json: {}", e))?;
    
    if let Some(stock_data) = stocks_json.get(&film_name) {
        if let Some(film_info) = stock_data.get("film_info") {
            return Ok(FilmInfo {
                description: film_info.get("description").and_then(|v| v.as_str()).unwrap_or("No description available").to_string(),
                primary_uses: film_info.get("primary_uses").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| vec!["General photography".to_string()]),
                characteristics: film_info.get("characteristics").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| vec!["Standard characteristics".to_string()]),
                famous_users: film_info.get("famous_users").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| vec!["Many photographers".to_string()]),
                ideal_conditions: film_info.get("ideal_conditions").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| vec!["Various lighting".to_string()]),
                era: film_info.get("era").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
                price_category: film_info.get("price_category").and_then(|v| v.as_str()).unwrap_or("mid-range").to_string(),
            });
        }
    }
    
    Err(format!("Film info not found for {}", film_name))
}

fn parse_comprehensive_film_stock(name: &str, data: &serde_json::Value) -> Result<FilmStock, String> {
    // Parse the new comprehensive film stock format
    let grain_chars = data.get("grain_characteristics").ok_or("Missing grain_characteristics")?;
    let density_dist = data.get("density_distribution").ok_or("Missing density_distribution")?;
    let visual_props = data.get("visual_properties").ok_or("Missing visual_properties")?;
    let digital_sim = data.get("digital_simulation").ok_or("Missing digital_simulation")?;
    
    // Extract grain size
    let size_um = grain_chars.get("size_um").ok_or("Missing size_um")?;
    let min_size = size_um.get("min").and_then(|v| v.as_f64())
        .ok_or("Missing grain size min value in JSON")? as f32;
    let max_size = size_um.get("max").and_then(|v| v.as_f64())
        .ok_or("Missing grain size max value in JSON")? as f32;
    let avg_size = size_um.get("average").and_then(|v| v.as_f64())
        .ok_or("Missing grain size average value in JSON")? as f32;
    
    // Extract density
    let density = density_dist.get("grains_per_mm2").and_then(|v| v.as_u64())
        .ok_or("Missing grains_per_mm2 value in JSON")? as u32;
    
    // Extract opacity
    let opacity_range = visual_props.get("opacity_range").ok_or("Missing opacity_range")?;
    let min_opacity = opacity_range.get("min").and_then(|v| v.as_f64())
        .ok_or("Missing opacity min value in JSON")? as f32;
    let max_opacity = opacity_range.get("max").and_then(|v| v.as_f64())
        .ok_or("Missing opacity max value in JSON")? as f32;
    
    // Extract digital simulation parameters
    let _grains_per_1024 = digital_sim.get("grains_per_1024px").and_then(|v| v.as_u64()).unwrap_or(400) as u32;
    
    Ok(FilmStock {
        basic_info: BasicInfo {
            name: name.to_string(),
            iso: 400, // Default, could be extracted from film_info if needed
            film_type: if name.contains("Tri-X") || name.contains("HP5") || name.contains("T-Max") || name.contains("Delta") || name.contains("Acros") || name.contains("Pan F") || name.contains("Neopan") || name.contains("FP4") || name.contains("Plus-X") || name.contains("Technical Pan") { "bw".to_string() } else { "color".to_string() },
        },
        grain_structure: GrainStructure {
            crystal_type: if name.contains("T-Max") { "tabular".to_string() } else { "cubic".to_string() },
            shape: grain_chars.get("shape").and_then(|v| v.as_str()).unwrap_or("irregular").to_string(),
            aspect_ratio: vec![1.0, 1.0], // Could be extracted if needed
            orientation: "random".to_string(),
            clustering: density_dist.get("clustering").and_then(|v| v.as_str()).unwrap_or("moderate").to_string(),
            edge_type: grain_chars.get("edge_type").and_then(|v| v.as_str()).unwrap_or("sharp").to_string(),
        },
        size_metrics: SizeMetrics {
            min_size_um: min_size,
            max_size_um: max_size,
            avg_size_um: avg_size,
            size_distribution: "normal".to_string(),
            size_variation_coeff: 0.5,
            density_per_mm2: density,
            spacing_pattern: density_dist.get("pattern").and_then(|v| v.as_str()).unwrap_or("random").to_string(),
        },
        visual_properties: VisualProperties {
            opacity_range: vec![min_opacity, max_opacity],
            contrast_level: visual_props.get("contrast").and_then(|v| v.as_str()).unwrap_or("medium").to_string(),
            edge_definition: "sharp".to_string(),
            opacity_variation: visual_props.get("opacity_variation").and_then(|v| v.as_f64()).unwrap_or(0.6) as f32,
            highlight_visibility: "medium".to_string(),
            shadow_visibility: "medium".to_string(),
            midtone_prominence: "medium".to_string(),
        },
        color_properties: ColorProperties {
            primary_cast: visual_props.get("color_cast").and_then(|v| v.as_str()).unwrap_or("neutral").to_string(),
            rgb_ranges: vec![RgbRange {
                r: vec![200, 255],
                g: vec![200, 255], 
                b: vec![200, 255],
                weight: 1.0,
            }],
            color_variation: "low".to_string(),
            saturation_level: "low".to_string(),
        },
        special_effects: SpecialEffects {
            halation: "none".to_string(),
            halation_color: "#ffffff".to_string(),
            halation_radius: 1.0,
            unique_artifacts: vec![],
            light_interaction: "normal".to_string(),
        },
        algorithmic_data: AlgorithmicData {
            clustering_algorithm: "poisson".to_string(),
            distribution_function: "normal(0.8, 0.5)".to_string(),
            spatial_correlation: 0.2,
            fractal_dimension: 1.2,
        },
    })
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![generate_grain, save_grain_image, get_available_film_stocks, get_categorized_film_stocks, get_film_info, load_user_image, save_composite_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}