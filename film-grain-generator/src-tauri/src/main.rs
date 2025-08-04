use serde::{Deserialize, Serialize};
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
struct GrainParams {
    film_stock: String,
    intensity: f32,
    size_multiplier: f32,
    contrast: f32,
    width: u32,
    height: u32,
    background: String,
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

#[tauri::command]
async fn generate_grain(params: GrainParams) -> Result<GrainResult, String> {
    let start_time = std::time::Instant::now();
    
    // Load film stock data (in a real app, this would be loaded once at startup)
    let film_stocks = load_film_stock_data()?;
    let stock = film_stocks.get(&params.film_stock)
        .ok_or_else(|| format!("Film stock '{}' not found", params.film_stock))?;
    
    // Generate grains using advanced algorithms
    let grains = generate_grains_advanced(stock, &params)?;
    
    // Render grains to image buffer using parallel processing
    let image_data = render_grains_parallel(&grains, &params, stock)?;
    
    let generation_time = start_time.elapsed().as_millis();
    
    Ok(GrainResult {
        data: image_data,
        width: params.width,
        height: params.height,
        generation_time_ms: generation_time,
        grain_count: grains.len(),
    })
}

fn load_film_stock_data() -> Result<HashMap<String, FilmStock>, String> {
    // Load realistic Fujifilm grain data
    let fuji_data = include_str!("../../../grainstuff.json");
    let fuji_json: serde_json::Value = serde_json::from_str(fuji_data)
        .map_err(|e| format!("Failed to parse grainstuff.json: {}", e))?;
    
    // Load original film stock data
    let json_data = include_str!("../../../stocks5.json");
    let stocks_json: serde_json::Value = serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse stocks5.json: {}", e))?;
    
    let mut stocks = HashMap::new();
    
    // Parse all film stocks from original JSON
    if let Some(obj) = stocks_json.as_object() {
        for (name, stock_data) in obj {
            if let Ok(mut film_stock) = parse_film_stock(name, stock_data) {
                // Override with realistic Fuji data if available
                if let Some(fuji_stock) = fuji_json.get(name) {
                    apply_realistic_fuji_data(&mut film_stock, fuji_stock);
                }
                stocks.insert(name.clone(), film_stock);
            }
        }
    }
    
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
    fuji400h.basic_info.name = "Fujifilm Pro 400H".to_string();
    fuji400h.basic_info.film_type = "color".to_string();
    fuji400h.size_metrics.density_per_mm2 = 850000;
    fuji400h.color_properties.rgb_ranges = vec![
        RgbRange { r: vec![210, 250], g: vec![220, 255], b: vec![200, 240], weight: 1.0 }
    ];
    stocks.insert("Fujifilm Pro 400H".to_string(), fuji400h);
    
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

fn generate_grains_advanced(stock: &FilmStock, params: &GrainParams) -> Result<Vec<Grain>, String> {
    let mut rng = thread_rng();
    let mut grains = Vec::new();
    
    // Ensure minimum visible grain count
    let base_grain_count = ((params.width * params.height) as f32 * params.intensity / 100.0 * 0.007) as usize;
    let min_grain_count = if params.intensity > 50.0 { 500 } else { 200 }; // Minimum visible grains
    let final_grain_count = base_grain_count.max(min_grain_count);
    println!("Generating {} grains for {} (base: {}, min: {})", final_grain_count, stock.basic_info.name, base_grain_count, min_grain_count);
    println!("Generating {} grains for {}", base_grain_count, stock.basic_info.name);
    
    // Generate grains with spatial correlation
    for _ in 0..final_grain_count {
        let x = rng.gen::<f32>() * params.width as f32;
        let y = rng.gen::<f32>() * params.height as f32;
        
        // Realistic size distribution - varied grain sizes
        let size_factor = {
            let rand_val = rng.gen::<f32>();
            if rand_val < 0.6 {
                // 60% average size grains
                rng.gen_range(0.8..1.2)
            } else if rand_val < 0.9 {
                // 30% smaller grains
                rng.gen_range(0.4..0.8)
            } else {
                // 10% larger grains
                rng.gen_range(1.2..1.8)
            }
        };
        
        // Simple size calculation with film stock variation
        let base_size = match stock.grain_structure.crystal_type.as_str() {
            "tabular" => 1.2, // T-grain: smaller, finer
            "cubic" => 1.8,   // Traditional grain: larger, chunkier  
            _ => 1.5,
        };
        let size = base_size * size_factor * params.size_multiplier;
        
        // Realistic opacity with variation
        let base_opacity = rng.gen_range(stock.visual_properties.opacity_range[0]..stock.visual_properties.opacity_range[1]);
        let opacity_variation = rng.gen_range(0.9..1.1); // ±10% variation
        let opacity = (base_opacity * (params.contrast / 100.0) * opacity_variation).min(1.0).max(0.1);
        
        // Force all grains to be circular for now - debug bowties
        let shape_factor = 1.0;
        
        grains.push(Grain {
            x,
            y,
            size,
            opacity,
            shape_factor,
        });
    }
    
    // Skip clustering for now to ensure it works
    // if stock.algorithmic_data.spatial_correlation > 0.1 {
    //     apply_basic_clustering(&mut grains, &mut rng, params.width, params.height);
    // }
    
    Ok(grains)
}

fn apply_basic_clustering(grains: &mut Vec<Grain>, rng: &mut ThreadRng, width: u32, height: u32) {
    let original_count = grains.len();
    let cluster_count = (original_count as f32 * 0.15) as usize; // 15% of grains form clusters
    
    for _ in 0..cluster_count {
        if grains.is_empty() { break; }
        
        let center_idx = rng.gen_range(0..grains.len());
        let center = grains[center_idx];
        
        // Add 1-2 grains near the center (small clusters)
        let cluster_size = rng.gen_range(1..3);
        for _ in 0..cluster_size {
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let distance = rng.gen_range(1.0..3.0) * center.size;
            
            let x = center.x + angle.cos() * distance;
            let y = center.y + angle.sin() * distance;
            
            if x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32 {
                grains.push(Grain {
                    x,
                    y,
                    size: center.size * rng.gen_range(0.8..1.2),
                    opacity: center.opacity * rng.gen_range(0.9..1.1),
                    shape_factor: center.shape_factor,
                });
            }
        }
    }
}

fn apply_spatial_clustering_old(grains: &mut Vec<Grain>, correlation: f32, rng: &mut ThreadRng) {
    if correlation < 0.1 { return; }
    
    let cluster_count = (grains.len() as f32 * correlation * 0.2) as usize;
    
    for _ in 0..cluster_count {
        if grains.len() < 2 { break; }
        
        let center_idx = rng.gen_range(0..grains.len());
        let center = grains[center_idx];
        
        // Realistic clustering based on film development physics
        let cluster_size = match correlation {
            c if c > 0.3 => rng.gen_range(3..8), // High correlation = larger clusters
            c if c > 0.2 => rng.gen_range(2..5), // Medium correlation
            _ => rng.gen_range(2..4),            // Low correlation
        };
        
        for i in 0..cluster_size {
            // Use Gaussian distribution for more realistic clustering
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let distance_factor = (-2.0 * rng.gen::<f32>().ln()).sqrt() * (2.0 * std::f32::consts::PI * rng.gen::<f32>()).cos();
            let distance = (distance_factor.abs() * center.size * 2.0).min(center.size * 8.0);
            
            let x = center.x + angle.cos() * distance;
            let y = center.y + angle.sin() * distance;
            
            if x >= 0.0 && y >= 0.0 && x < 2048.0 && y < 2048.0 {
                // Vary grain properties within cluster
                let size_variation = 1.0 + (rng.gen::<f32>() - 0.5) * 0.4;
                let opacity_variation = 1.0 + (rng.gen::<f32>() - 0.5) * 0.3;
                
                grains.push(Grain {
                    x,
                    y,
                    size: center.size * size_variation,
                    opacity: (center.opacity * opacity_variation).min(1.0).max(0.1),
                    shape_factor: center.shape_factor * rng.gen_range(0.8..1.2),
                });
            }
        }
    }
}

fn render_grains_parallel(grains: &[Grain], params: &GrainParams, stock: &FilmStock) -> Result<Vec<u8>, String> {
    println!("Rendering {} grains for film stock: {}", grains.len(), stock.basic_info.name);
    let width = params.width as usize;
    let height = params.height as usize;
    
    // Create image buffer
    let mut img: RgbaImage = ImageBuffer::new(params.width, params.height);
    
    // Always use transparent background for the actual image data
    for pixel in img.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]); // Always transparent
    }
    
    // Render grains in parallel chunks
    let chunk_height = (height / rayon::current_num_threads()).max(1);
    let chunks: Vec<_> = (0..height).step_by(chunk_height).collect();
    
    let grain_data: Vec<_> = chunks.par_iter().map(|&start_y| {
        let end_y = (start_y + chunk_height).min(height);
        let mut chunk_grains = Vec::new();
        
        for grain in grains {
            let grain_y = grain.y as usize;
            if grain_y >= start_y && grain_y < end_y {
                chunk_grains.push(*grain);
            }
        }
        
        (start_y, end_y, chunk_grains)
    }).collect();
    
    // Apply grain data to image
    for (start_y, end_y, chunk_grains) in grain_data {
        for grain in chunk_grains {
            render_single_grain(&mut img, &grain, stock, params);
        }
    }
    
    // Convert to raw RGBA bytes
    Ok(img.into_raw())
}

fn render_single_grain(img: &mut RgbaImage, grain: &Grain, stock: &FilmStock, params: &GrainParams) {
    let center_x = grain.x as i32;
    let center_y = grain.y as i32;
    let radius = grain.size as i32; // Fix: was grain.size * 2.0, causing oversized grains
    
    // Realistic color with subtle variation
    let color_variation = thread_rng().gen_range(0.95..1.05); // ±5% color variation
    let r = (200.0 * color_variation) as u8;
    let g = (200.0 * color_variation) as u8;
    let b = (200.0 * color_variation) as u8;
    
    // Boost opacity for visibility while maintaining relative differences
    let boosted_opacity = match grain.opacity {
        x if x < 0.15 => x * 5.0,  // Very fine films (Acros, Velvia) - 5x boost
        x if x < 0.25 => x * 3.0,  // Fine films - 3x boost  
        x => x * 2.0,              // Coarser films - 2x boost
    };
    let alpha = (boosted_opacity * 200.0).min(255.0).max(50.0) as u8;
    
    // Render grain with anti-aliasing
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = center_x + dx;
            let y = center_y + dy;
            
            if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                
                if distance <= grain.size {
                    // Simple circular grain - no complex shape distortion
                    let pixel = img.get_pixel_mut(x as u32, y as u32);
                    let blend_factor = alpha as f32 / 255.0;
                    pixel[0] = ((pixel[0] as f32 * (1.0 - blend_factor)) + (r as f32 * blend_factor)) as u8;
                    pixel[1] = ((pixel[1] as f32 * (1.0 - blend_factor)) + (g as f32 * blend_factor)) as u8;
                    pixel[2] = ((pixel[2] as f32 * (1.0 - blend_factor)) + (b as f32 * blend_factor)) as u8;
                    pixel[3] = ((pixel[3] as f32).max(alpha as f32)) as u8;
                }
            }
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

#[tauri::command]
async fn get_available_film_stocks() -> Result<Vec<String>, String> {
    let stocks = load_film_stock_data()?;
    let mut stock_names: Vec<String> = stocks.keys().cloned().collect();
    stock_names.sort();
    Ok(stock_names)
}

fn parse_film_stock(name: &str, data: &serde_json::Value) -> Result<FilmStock, String> {
    let basic_info = data.get("basic_info").ok_or("Missing basic_info")?;
    let grain_structure = data.get("grain_structure").ok_or("Missing grain_structure")?;
    let size_metrics = data.get("size_metrics").ok_or("Missing size_metrics")?;
    let visual_properties = data.get("visual_properties").ok_or("Missing visual_properties")?;
    let color_properties = data.get("color_properties").ok_or("Missing color_properties")?;
    let special_effects = data.get("special_effects").ok_or("Missing special_effects")?;
    let algorithmic_data = data.get("algorithmic_data").ok_or("Missing algorithmic_data")?;
    
    Ok(FilmStock {
        basic_info: BasicInfo {
            name: name.to_string(),
            iso: basic_info.get("iso").and_then(|v| v.as_u64()).unwrap_or(400) as u32,
            film_type: basic_info.get("type").and_then(|v| v.as_str()).unwrap_or("bw").to_string(),
        },
        grain_structure: GrainStructure {
            crystal_type: grain_structure.get("crystal_type").and_then(|v| v.as_str()).unwrap_or("cubic").to_string(),
            shape: grain_structure.get("shape").and_then(|v| v.as_str()).unwrap_or("irregular").to_string(),
            aspect_ratio: grain_structure.get("aspect_ratio").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64()).map(|f| f as f32).collect())
                .unwrap_or(vec![1.0, 1.0]),
            orientation: grain_structure.get("orientation").and_then(|v| v.as_str()).unwrap_or("random").to_string(),
            clustering: grain_structure.get("clustering").and_then(|v| v.as_str()).unwrap_or("moderate").to_string(),
            edge_type: grain_structure.get("edge_type").and_then(|v| v.as_str()).unwrap_or("sharp").to_string(),
        },
        size_metrics: SizeMetrics {
            min_size_um: size_metrics.get("min_size_um").and_then(|v| v.as_f64()).unwrap_or(0.5) as f32,
            max_size_um: size_metrics.get("max_size_um").and_then(|v| v.as_f64()).unwrap_or(2.0) as f32,
            avg_size_um: size_metrics.get("avg_size_um").and_then(|v| v.as_f64()).unwrap_or(0.8) as f32,
            size_distribution: size_metrics.get("size_distribution").and_then(|v| v.as_str()).unwrap_or("normal").to_string(),
            size_variation_coeff: size_metrics.get("size_variation_coeff").and_then(|v| v.as_f64()).unwrap_or(0.5) as f32,
            density_per_mm2: size_metrics.get("density_per_mm2").and_then(|v| v.as_u64()).unwrap_or(800000) as u32,
            spacing_pattern: size_metrics.get("spacing_pattern").and_then(|v| v.as_str()).unwrap_or("random").to_string(),
        },
        visual_properties: VisualProperties {
            opacity_range: visual_properties.get("opacity_range").and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_f64()).map(|f| f as f32).collect())
                .unwrap_or(vec![0.2, 0.8]),
            contrast_level: visual_properties.get("contrast_level").and_then(|v| v.as_str()).unwrap_or("medium").to_string(),
            edge_definition: visual_properties.get("edge_definition").and_then(|v| v.as_str()).unwrap_or("sharp").to_string(),
            opacity_variation: visual_properties.get("opacity_variation").and_then(|v| v.as_f64()).unwrap_or(0.6) as f32,
            highlight_visibility: visual_properties.get("highlight_visibility").and_then(|v| v.as_str()).unwrap_or("medium").to_string(),
            shadow_visibility: visual_properties.get("shadow_visibility").and_then(|v| v.as_str()).unwrap_or("medium").to_string(),
            midtone_prominence: visual_properties.get("midtone_prominence").and_then(|v| v.as_str()).unwrap_or("medium").to_string(),
        },
        color_properties: ColorProperties {
            primary_cast: color_properties.get("primary_cast").and_then(|v| v.as_str()).unwrap_or("neutral").to_string(),
            rgb_ranges: vec![RgbRange {
                r: vec![200, 255],
                g: vec![200, 255],
                b: vec![200, 255],
                weight: 1.0,
            }],
            color_variation: color_properties.get("color_variation").and_then(|v| v.as_str()).unwrap_or("low").to_string(),
            saturation_level: color_properties.get("saturation_level").and_then(|v| v.as_str()).unwrap_or("low").to_string(),
        },
        special_effects: SpecialEffects {
            halation: special_effects.get("halation").and_then(|v| v.as_str()).unwrap_or("none").to_string(),
            halation_color: special_effects.get("halation_color").and_then(|v| v.as_str()).unwrap_or("#ffffff").to_string(),
            halation_radius: special_effects.get("halation_radius").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
            unique_artifacts: vec![],
            light_interaction: special_effects.get("light_interaction").and_then(|v| v.as_str()).unwrap_or("normal").to_string(),
        },
        algorithmic_data: AlgorithmicData {
            clustering_algorithm: algorithmic_data.get("clustering_algorithm").and_then(|v| v.as_str()).unwrap_or("poisson").to_string(),
            distribution_function: algorithmic_data.get("distribution_function").and_then(|v| v.as_str()).unwrap_or("normal(0.8, 0.5)").to_string(),
            spatial_correlation: algorithmic_data.get("spatial_correlation").and_then(|v| v.as_f64()).unwrap_or(0.2) as f32,
            fractal_dimension: algorithmic_data.get("fractal_dimension").and_then(|v| v.as_f64()).unwrap_or(1.2) as f32,
        },
    })
}

fn apply_realistic_fuji_data(film_stock: &mut FilmStock, fuji_data: &serde_json::Value) {
    // Apply realistic grain parameters from grainstuff.json
    if let Some(digital_sim) = fuji_data.get("digital_simulation") {
        if let Some(grains_per_1024) = digital_sim.get("grains_per_1024px").and_then(|v| v.as_u64()) {
            // Convert grains per 1024px to density per mm²
            let density_per_mm2 = (grains_per_1024 as f32 * 1000000.0 / (1024.0 * 1024.0)) as u32;
            film_stock.size_metrics.density_per_mm2 = density_per_mm2;
            println!("Updated {} density to {} grains per mm² ({} per 1024px)", 
                     film_stock.basic_info.name, density_per_mm2, grains_per_1024);
        }
        
        if let Some(pixel_range) = digital_sim.get("pixel_size_range").and_then(|v| v.as_array()) {
            if pixel_range.len() >= 2 {
                if let (Some(min), Some(max)) = (pixel_range[0].as_f64(), pixel_range[1].as_f64()) {
                    // Convert pixel sizes to microns (approximate)
                    film_stock.size_metrics.min_size_um = min as f32 / 10.0; // Rough conversion
                    film_stock.size_metrics.max_size_um = max as f32 / 10.0;
                    film_stock.size_metrics.avg_size_um = (min as f32 + max as f32) / 20.0;
                }
            }
        }
        
        if let Some(opacity_range) = digital_sim.get("recommended_opacity").and_then(|v| v.as_array()) {
            if opacity_range.len() >= 2 {
                if let (Some(min), Some(max)) = (opacity_range[0].as_f64(), opacity_range[1].as_f64()) {
                    film_stock.visual_properties.opacity_range = vec![min as f32, max as f32];
                }
            }
        }
    }
    
    // Apply grain shape characteristics
    if let Some(grain_shape) = fuji_data.get("grain_shape").and_then(|v| v.as_str()) {
        film_stock.grain_structure.shape = grain_shape.to_string();
    }
    
    if let Some(edge_type) = fuji_data.get("edge_type").and_then(|v| v.as_str()) {
        film_stock.grain_structure.edge_type = edge_type.to_string();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![generate_grain, save_grain_image, get_available_film_stocks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}