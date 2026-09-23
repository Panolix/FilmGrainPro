use serde::{Deserialize, Serialize};
use image::{ImageBuffer, Rgba, RgbaImage};
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use base64::{Engine as _, engine::general_purpose};

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
    #[serde(default)]
    cluster_size: String,
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
struct EnhancedFilmData {
    color_crossover: ColorCrossover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariationData {
    size_variation_coeff: f32,
    opacity_variation: f32,
    notes: String,
}

// Sourced grain model (from grain_model.json; manufacturer datasheet values where
// available, otherwise a clearly-labelled estimate). Used to drive grain amplitude
// from real graininess measurements instead of arbitrary constants.
#[derive(Debug, Clone, Deserialize)]
struct GraininessMetric {
    metric: Option<String>,
    value: Option<f64>,
    scale: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GrainMorphology {
    family: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GrainSizeModel {
    min: f32,
    mean: f32,
    max: f32,
    sigma: Option<f32>,
}

// Datasheet push-processing data (from grain_model.json)
#[derive(Debug, Clone, Deserialize)]
struct PushData {
    rated: Option<u32>,
    ei: Option<Vec<u32>>,
    stops: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
struct EiRange {
    max: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
struct GrainModel {
    iso: Option<u32>,
    #[serde(rename = "type")]
    film_type: Option<String>,
    graininess: Option<GraininessMetric>,
    morphology: Option<GrainMorphology>,
    grain_size_um: Option<GrainSizeModel>,
    push: Option<PushData>,
    ei_range: Option<EiRange>,
}

#[derive(Debug, Clone, Deserialize)]
struct GrainModelFile {
    films: HashMap<String, GrainModel>,
}

fn load_grain_models() -> Result<HashMap<String, GrainModel>, String> {
    let data = include_str!("../../grain_model.json");
    let parsed: GrainModelFile = serde_json::from_str(data)
        .map_err(|e| format!("Failed to parse grain_model.json: {}", e))?;
    Ok(parsed.films)
}

fn grain_models() -> &'static HashMap<String, GrainModel> {
    static CACHE: std::sync::OnceLock<HashMap<String, GrainModel>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| load_grain_models().unwrap_or_default())
}

// The film data files are parsed once and cached, instead of on every generate_grain call.
fn film_stocks_cached() -> &'static HashMap<String, FilmStock> {
    static CACHE: std::sync::OnceLock<HashMap<String, FilmStock>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| load_film_stock_data().unwrap_or_default())
}

fn enhanced_data_cached() -> &'static HashMap<String, EnhancedFilmData> {
    static CACHE: std::sync::OnceLock<HashMap<String, EnhancedFilmData>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| load_enhanced_film_data().unwrap_or_default())
}

fn variation_data_cached() -> &'static HashMap<String, VariationData> {
    static CACHE: std::sync::OnceLock<HashMap<String, VariationData>> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| load_variation_data().unwrap_or_default())
}

// Map a published graininess value onto a normalised 0..1 grain index.
// RMS and PGI use different scales, so normalise within each measurement family.
fn grain_index(model: Option<&GrainModel>, iso: u32) -> f32 {
    if let Some(m) = model {
        if let Some(g) = &m.graininess {
            if let (Some(metric), Some(v)) = (g.metric.as_deref(), g.value) {
                let (lo, hi) = match metric {
                    "pgi" => (25.0, 48.0),
                    _ => match g.scale.as_deref() {
                        Some("fuji_colneg_rms_48um") => (3.0, 5.0),
                        Some("reverse_rms_48um") => (6.0, 12.0),
                        _ => (6.0, 22.0), // B&W RMS and third-party estimates
                    },
                };
                return (((v - lo) / (hi - lo)) as f32).clamp(0.0, 1.0);
            }
        }
    }
    // Fallback: ISO-based expectation when no graininess value exists.
    let iso = iso.max(25) as f32;
    ((iso.ln() - 50f32.ln()) / (3200f32.ln() - 50f32.ln())).clamp(0.0, 1.0)
}

// Relative visibility of grain by emulsion class: B&W silver grain is the most
// prominent, colour-negative dye clouds are softer, reversal slides the finest.
fn class_weight(model: Option<&GrainModel>) -> f32 {
    match model.and_then(|m| m.film_type.as_deref()) {
        Some("bw_neg") => 1.0,
        Some("color_neg") => 0.8,
        Some("color_rev") => 0.65,
        _ => 1.0,
    }
}

// Documented push latitude (in stops) for a film, from its datasheet push table or
// EI range; falls back to a class default when the film publishes neither.
fn push_latitude_stops(model: Option<&GrainModel>) -> f32 {
    if let Some(m) = model {
        if let Some(p) = &m.push {
            if let Some(s) = p.stops {
                if s > 0.0 {
                    return s;
                }
            }
            if let (Some(rated), Some(ei)) = (p.rated, p.ei.as_ref().and_then(|v| v.last().copied())) {
                if rated > 0 && ei > rated {
                    return (ei as f32 / rated as f32).log2();
                }
            }
        }
        if let Some(r) = &m.ei_range {
            if let (Some(iso), Some(hi)) = (m.iso, r.max) {
                if iso > 0 && hi > iso {
                    return (hi as f32 / iso as f32).log2();
                }
            }
        }
    }
    match model.and_then(|m| m.film_type.as_deref()) {
        Some("color_rev") => 1.0,
        Some("color_neg") => 2.0,
        Some("bw_neg") => 3.0,
        _ => 2.0,
    }
}

// Grain response to the exposure/push slider, in stops: returns (size_mult, opacity_mult).
// Pushing within the film's documented latitude raises grain strongly; beyond it the
// response flattens; pulling lowers grain. Films with more push latitude (e.g. Tri-X to
// +3, Portra 800 to +2, Velvia only +1) therefore respond differently at the same setting.
fn push_response(model: Option<&GrainModel>, exposure_stops: f32) -> (f32, f32) {
    if exposure_stops > 0.0 {
        let lat = push_latitude_stops(model).max(0.5);
        let within = exposure_stops.min(lat);
        let beyond = (exposure_stops - lat).max(0.0);
        let eff = within + beyond * 0.35;
        (1.0 + 0.06 * eff, 1.0 + 0.22 * eff)
    } else if exposure_stops < 0.0 {
        let pull = (-exposure_stops).min(2.0);
        (1.0 - 0.05 * pull, (1.0 - 0.12 * pull).max(0.4))
    } else {
        (1.0, 1.0)
    }
}

// Translate the sourced morphology family into the shape keys the renderer understands.
fn morphology_shape_key(model: Option<&GrainModel>, fallback: &str) -> String {
    if let Some(m) = model {
        if let Some(mp) = &m.morphology {
            if let Some(f) = mp.family.as_deref() {
                return match f {
                    "tabular" | "core_shell" => "tabular".to_string(),
                    "sigma" => "Sigma grain".to_string(),
                    "cubic" => "irregular".to_string(),
                    other => other.to_string(),
                };
            }
        }
    }
    fallback.to_string()
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
    /// Halation grains render with a red-orange tint (light scatter into the red layer).
    halation: bool,
}


#[tauri::command]
async fn generate_grain(params: GrainParams) -> Result<GrainResult, String> {
    let start_time = std::time::Instant::now();
    
    // Data files are parsed once and cached across calls (see film_stocks_cached etc.)
    let film_stocks = film_stocks_cached();
    let stock = film_stocks.get(&params.film_stock)
        .ok_or_else(|| format!("Film stock '{}' not found", params.film_stock))?;

    // Authentic per-film variation coefficients (cached)
    let variation_stock = variation_data_cached().get(&params.film_stock);

    // Sourced grain model (manufacturer graininess / morphology), cached after first use
    let grain_model = grain_models().get(&params.film_stock);

    // Generate grains using advanced algorithms with enhancements
    let _start_time = std::time::Instant::now();
    let mut grains = generate_grains_advanced(stock, &params, variation_stock, grain_model)?;
    let _generation_time = _start_time.elapsed();

    // 🆕 ENHANCEMENT 14: Apply halation effects for CineStill films
    apply_halation_effect(&mut grains, stock, &params);
    
    // Render on the CPU (multi-threaded via Rayon)
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
            cluster_size: "2-4_grains".to_string(),
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

fn generate_grains_advanced(stock: &FilmStock, params: &GrainParams, variation_data: Option<&VariationData>, model: Option<&GrainModel>) -> Result<Vec<Grain>, String> {
    let mut rng = thread_rng();
    let mut grains = Vec::new();

    // Published graininess mapped to a 0..1 index. Drives grain amplitude so that
    // measured coarse-grained films render stronger than measured fine-grained ones.
    let iso = model.and_then(|m| m.iso).unwrap_or(stock.basic_info.iso);
    let gi = grain_index(model, iso);
    let shape_key = morphology_shape_key(model, &stock.grain_structure.shape);

    let type_weight = class_weight(model);

    // Grain size range: prefer the sourced model, fall back to the legacy stock data.
    let (size_min_um, size_max_um) = match model.and_then(|m| m.grain_size_um.as_ref()) {
        Some(sz) if sz.max > sz.min => (sz.min, sz.max),
        _ => (stock.size_metrics.min_size_um, stock.size_metrics.max_size_um),
    };

    // Use film stock's actual density as base, then apply user density multiplier
    let canvas_area_ratio = (params.width * params.height) as f32 / (1024.0 * 1024.0);
    let user_density_multiplier = params.grain_density as f32 / 1000.0; // Convert from 0.5-5.0 range

    // Count is calibrated from the sourced grain size instead of the unpublished
    // grains-per-mm2 figure: finer emulsions need more grains for a comparable
    // coverage, coarser ones fewer. The 0.9 um reference matches a mid-speed film.
    let mean_size_um = model
        .and_then(|m| m.grain_size_um.as_ref())
        .map(|s| s.mean)
        .filter(|v| *v > 0.0)
        .unwrap_or(stock.size_metrics.avg_size_um)
        .max(0.2);
    let size_scale = (0.9 / mean_size_um).powi(2).clamp(0.3, 3.0);
    let final_grain_count =
        (70000.0 * canvas_area_ratio * user_density_multiplier * size_scale) as usize;
    println!("Grain: {} | index {:.2} | mean {:.2} um | {:.1}x | {} grains",
             stock.basic_info.name, gi, mean_size_um, user_density_multiplier, final_grain_count);

    // Generate grains with spatial correlation
    // 🚀 NEW: Generate grain positions using clustering data as pattern indicator
    let pattern = if stock.grain_structure.clustering == "heavy" { "clustered" } else { "random" };
    let grain_positions = generate_pattern_based_positions(pattern, params, final_grain_count, &mut rng);
    
    for (x, y) in grain_positions.iter() {
        let x = *x;
        let y = *y;
        
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
        
        // Sourced grain size: log-normal-ish around the model mean, bounded by min/max.
        let size_range_um = match model.and_then(|m| m.grain_size_um.as_ref()) {
            Some(sz) => {
                let sigma = sz.sigma.unwrap_or(0.35);
                // Approx N(0,1) from three uniforms (mean 0, variance ~1).
                let g = (rng.gen::<f32>() + rng.gen::<f32>() + rng.gen::<f32>() - 1.5) * 2.0;
                (sz.mean * (1.0 + sigma * g)).clamp(sz.min, sz.max)
            }
            None => rng.gen_range(size_min_um..=size_max_um),
        };
        let shape_size_factor = get_shape_size_factor(&shape_key, &mut rng);
        let base_size = size_range_um * 0.5 * shape_size_factor;
        let mut size = (base_size * size_factor * params.size_multiplier).max(0.3);

        // Amplitude anchored on published graininess: fine-grained films stay subtle,
        // coarse-grained films render stronger. The opacity slider still scales it.
        let base_opacity = (0.15 + 0.40 * gi) * type_weight * rng.gen_range(0.85..1.15);
        let opacity_var = variation_data
            .map(|v| v.opacity_variation)
            .unwrap_or(stock.visual_properties.opacity_variation);

        let opacity_variation = rng.gen_range(1.0 - opacity_var * 0.5..1.0 + opacity_var * 0.5);
        let user_contrast_factor = params.contrast / 100.0;   // User opacity control

        // Apply film-specific contrast from JSON
        let film_contrast_factor = match stock.visual_properties.contrast_level.as_str() {
            "very high" => 1.3,
            "high" => 1.15,
            "medium-high" => 1.05,
            "medium" => 1.0,
            "low-medium" => 0.95,
            "low" => 0.85,
            _ => 1.0,
        };
        let contrast_factor = user_contrast_factor * film_contrast_factor;

        // Exposure slider -> datasheet push/pull response for this film
        let (push_size_mult, push_opacity_mult) = push_response(model, params.exposure_compensation);
        size *= push_size_mult;

        let mut opacity = (base_opacity * contrast_factor * opacity_variation * push_opacity_mult).min(1.0).max(0.05);

        // 🚀 ENHANCED: Apply aging effects directly from UI parameters
        if let Some(age_years) = params.film_age_years {
            if age_years > 0.0 {
                let storage_temp = params.storage_temp.unwrap_or(20.0);
                opacity = apply_realistic_aging_effects(opacity, size, age_years, storage_temp, &stock.basic_info.film_type);
            }
        }

        // 🚀 NEW: Create grain with shape-based characteristics and ISO effects
        let base_shape_factor = get_shape_factor(&shape_key, &mut rng);
        let iso_irregularity = get_iso_irregularity_factor(iso);
        let shape_factor = base_shape_factor * rng.gen_range(1.0 - iso_irregularity * 0.2..1.0 + iso_irregularity * 0.2);
        
        grains.push(Grain {
            x,
            y,
            size,
            opacity,
            shape_factor,
            halation: false,
        });
    }
    
    // Apply realistic clustering based on film stock characteristics from JSON
    let clustering_strength = match stock.grain_structure.clustering.as_str() {
        "heavy" => 0.8,
        "moderate" => 0.4,
        "light" => 0.2,
        "none" => 0.0,
        _ => 0.2,
    };
    
    if clustering_strength > 0.0 {
        // Cluster sizes come from the per-film cluster_size field (e.g. "2-4_grains")
        let cluster_size_range = parse_json_cluster_size_range(&stock.grain_structure.cluster_size);
        apply_realistic_clustering(&mut grains, &mut rng, params.width, params.height, clustering_strength, cluster_size_range);
    }
    
    Ok(grains)
}

fn apply_realistic_clustering(grains: &mut Vec<Grain>, rng: &mut ThreadRng, width: u32, height: u32, strength: f32, cluster_size_range: (usize, usize)) {
    let cluster_count = (grains.len() as f32 * strength * 0.1) as usize; // 10% of grains form clusters
    
    for _ in 0..cluster_count {
        if grains.is_empty() { break; }
        
        let seed_idx = rng.gen_range(0..grains.len());
        let seed_grain = grains[seed_idx];
        
        // Use cluster size range from JSON data
        let cluster_size = rng.gen_range(cluster_size_range.0..=cluster_size_range.1);
        
        let cluster_spread = seed_grain.size * 2.0;
        
        for _ in 0..cluster_size {
            let distance = rng.gen::<f32>() * cluster_spread;
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            
            let x = seed_grain.x + angle.cos() * distance;
            let y = seed_grain.y + angle.sin() * distance;
            
            if x >= 0.0 && y >= 0.0 && x < width as f32 && y < height as f32 {
                grains.push(Grain {
                    x,
                    y,
                    size: seed_grain.size * rng.gen_range(0.8..1.2),
                    opacity: seed_grain.opacity * rng.gen_range(0.9..1.1),
                    shape_factor: seed_grain.shape_factor,
                    halation: seed_grain.halation,
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

// Cache for expensive operations
thread_local! {
    static FILM_COLORS: std::cell::RefCell<std::collections::HashMap<String, (u8, u8, u8)>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

fn render_grain_to_pixels(grain: &Grain, stock: &FilmStock, params: &GrainParams) -> Vec<(u32, u32, Rgba<u8>)> {
    let center_x = grain.x as i32;
    let center_y = grain.y as i32;
    let radius = grain.size as i32;
    
    // Early bounds check - skip grains completely outside canvas
    if center_x + radius < 0 || center_y + radius < 0 || 
       center_x - radius >= params.width as i32 || center_y - radius >= params.height as i32 {
        return Vec::new();
    }
    
    // 🚀 NEW: Enhanced color film simulation with multi-layer rendering
    let (mut final_r, mut final_g, mut final_b) = if stock.basic_info.film_type == "color" {
        render_color_film_grain(grain, stock, params)
    } else {
        render_bw_film_grain(grain, stock, params)
    };
    
    // 🆕 ENHANCEMENT 4: Independent per-channel grain colour variation for colour films
    if stock.basic_info.film_type == "color" {
        let mut rng = thread_rng();
        final_r = ((final_r as f32 * rng.gen_range(0.90..1.10)).clamp(0.0, 255.0)) as u8;
        final_g = ((final_g as f32 * rng.gen_range(0.90..1.10)).clamp(0.0, 255.0)) as u8;
        final_b = ((final_b as f32 * rng.gen_range(0.90..1.10)).clamp(0.0, 255.0)) as u8;
    }

    // Halation renders as a red-orange glow (light scattered into the red-sensitive layer)
    if grain.halation {
        final_r = (final_r as f32 * 0.4 + 255.0 * 0.6) as u8;
        final_g = (final_g as f32 * 0.4 + 70.0 * 0.6) as u8;
        final_b = (final_b as f32 * 0.4 + 30.0 * 0.6) as u8;
    }

    // Film contrast is already applied to grain.opacity during generation; do not
    // apply the JSON contrast multiplier again here (it double-counted it previously).
    // Gamma-mapped so the opacity slider has effect without saturating immediately.
    let alpha = ((grain.opacity.powf(0.8)) * 255.0).clamp(20.0, 255.0) as u8;
    // Pre-allocate pixels vector with estimated capacity
    let estimated_pixels = ((radius * radius) as f32 * 3.14159) as usize;
    let mut pixels = Vec::with_capacity(estimated_pixels);
    
    // Optimized grain rendering with fewer calculations
    let grain_size_sq = grain.size * grain.size;
    let shape_factor_inv = 1.0 / grain.shape_factor;
    
    // 🆕 ENHANCEMENT 6: Enhanced edge rendering based on JSON edge_type
    let edge_softness = get_json_edge_softness(&stock.grain_structure.edge_type);
    
    for dy in -radius..=radius {
        let dy_sq = (dy * dy) as f32;
        let y = center_y + dy;
        
        // Skip entire row if outside bounds
        if y < 0 || y >= params.height as i32 {
            continue;
        }
        
        for dx in -radius..=radius {
            let x = center_x + dx;
            
            // Quick bounds check
            if x < 0 || x >= params.width as i32 {
                continue;
            }
            
            // Fast distance calculation with shape factor
            let adjusted_dx = dx as f32 * shape_factor_inv;
            let distance_sq = adjusted_dx * adjusted_dx + dy_sq;
            
            if distance_sq <= grain_size_sq {
                // Enhanced edge calculation using JSON edge_type data
                let distance = distance_sq.sqrt();
                let edge_alpha = if stock.grain_structure.edge_type == "soft" {
                    if distance > grain.size * 0.6 {
                        ((grain.size - distance) / (grain.size * 0.4 * edge_softness)).max(0.0)
                    } else {
                        1.0
                    }
                } else if stock.grain_structure.edge_type == "hard" {
                    if distance > grain.size * 0.98 { 0.0 } else { 1.0 }
                } else {
                    // Sharp edge (default) with configurable softness
                    if distance > grain.size * 0.85 {
                        ((grain.size - distance) / (grain.size * 0.15 * edge_softness)).max(0.0)
                    } else {
                        1.0
                    }
                };
                
                let final_alpha = (alpha as f32 * edge_alpha) as u8;
                
                if final_alpha > 10 {
                    pixels.push((x as u32, y as u32, Rgba([final_r, final_g, final_b, final_alpha])));
                }
            }
        }
    }
    
    pixels
}

// 🚀 NEW: Get JSON color cast multiplier from primary_cast field
fn get_json_color_cast_multiplier(color_cast: &str) -> (f32, f32, f32) {
    match color_cast {
        "neutral" | "neutral_gray" => (1.0, 1.0, 1.0),
        "warm" | "warm_brown" => (1.1, 1.05, 0.9),
        "cool" | "cool_blue" => (0.95, 0.98, 1.08),
        "warm_yellow" => (1.08, 1.05, 0.92),
        "cool_green" => (0.98, 1.02, 1.0),
        "sepia" => (1.2, 1.1, 0.8),
        "cyan_tint" => (0.9, 1.0, 1.1),
        "magenta_tint" => (1.1, 0.95, 1.05),
        "yellow_tint" => (1.05, 1.05, 0.9),
        "tungsten (warm cast)" => (1.15, 1.08, 0.85), // Fixed CineStill 800T
        "slight green bias" => (0.98, 1.02, 1.0),
        "saturated warm" => (1.12, 1.08, 0.88),
        // Additional casts present in fixed.json
        "warm (slight magenta bias)" => (1.08, 1.0, 1.03),
        "warm (orange-red bias)" => (1.12, 1.02, 0.9),
        "cool_gray" => (0.98, 1.0, 1.03),
        "faithful neutral" => (1.0, 1.0, 1.0),
        "neutral (slightly warm)" => (1.04, 1.01, 0.98),
        "slightly warm" => (1.05, 1.02, 0.97),
        "saturated vibrant" => (1.0, 1.0, 1.0),
        _ => (1.0, 1.0, 1.0),
    }
}

// 🚀 NEW: Get edge softness from JSON edge_type field
fn get_json_edge_softness(edge_type: &str) -> f32 {
    match edge_type {
        "soft" => 1.4,          // More gradual falloff
        "sharp" => 0.8,         // Crisp edges
        "hard" => 0.6,          // Very sharp cutoff
        "crystalline" => 0.5,   // Very sharp, faceted edges
        _ => 1.0,               // Default
    }
}

// 🚀 NEW: Parse JSON cluster_size field (e.g., "2-4_grains", "3-5_grains")
fn parse_json_cluster_size_range(cluster_size: &str) -> (usize, usize) {
    // Parse strings like "2-4_grains", "3-5_grains", "single_grains"
    if cluster_size.contains("single") {
        return (1, 1); // No clustering
    }
    
    // Extract numbers from patterns like "2-4_grains"
    if let Some(dash_pos) = cluster_size.find('-') {
        if let Some(underscore_pos) = cluster_size.find('_') {
            let min_str = &cluster_size[..dash_pos];
            let max_str = &cluster_size[dash_pos + 1..underscore_pos];
            
            if let (Ok(min), Ok(max)) = (min_str.parse::<usize>(), max_str.parse::<usize>()) {
                return (min, max);
            }
        }
    }
    
    // Fallback based on clustering level
    match cluster_size {
        s if s.contains("heavy") => (3, 5),
        s if s.contains("moderate") => (2, 4),
        s if s.contains("light") => (2, 3),
        _ => (2, 3), // Default
    }
}

// 🚀 NEW: Generate pattern-based grain positions using JSON pattern data
fn generate_pattern_based_positions(pattern: &str, params: &GrainParams, count: usize, rng: &mut ThreadRng) -> Vec<(f32, f32)> {
    match pattern {
        "random" => generate_random_positions(params, count, rng),
        "clustered" => generate_clustered_positions(params, count, rng),
        "regular" => generate_regular_positions(params, count, rng),
        "poisson" => generate_poisson_positions(params, count, rng),
        _ => generate_random_positions(params, count, rng), // Default
    }
}

fn generate_random_positions(params: &GrainParams, count: usize, rng: &mut ThreadRng) -> Vec<(f32, f32)> {
    (0..count)
        .map(|_| (
            rng.gen::<f32>() * params.width as f32,
            rng.gen::<f32>() * params.height as f32,
        ))
        .collect()
}

fn generate_clustered_positions(params: &GrainParams, count: usize, rng: &mut ThreadRng) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    let cluster_count = (count as f32 * 0.1) as usize; // 10% cluster centers
    
    // Generate cluster centers
    let cluster_centers: Vec<(f32, f32)> = (0..cluster_count)
        .map(|_| (
            rng.gen::<f32>() * params.width as f32,
            rng.gen::<f32>() * params.height as f32,
        ))
        .collect();
    
    // Distribute grains around cluster centers
    for i in 0..count {
        if i < cluster_centers.len() {
            positions.push(cluster_centers[i]);
        } else {
            let center = cluster_centers[i % cluster_centers.len()];
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let distance = rng.gen::<f32>() * 50.0; // Cluster radius
            
            let x = (center.0 + angle.cos() * distance).clamp(0.0, params.width as f32);
            let y = (center.1 + angle.sin() * distance).clamp(0.0, params.height as f32);
            positions.push((x, y));
        }
    }
    
    positions
}

fn generate_regular_positions(params: &GrainParams, count: usize, rng: &mut ThreadRng) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    let grid_size = (count as f32).sqrt() as usize;
    let x_step = params.width as f32 / grid_size as f32;
    let y_step = params.height as f32 / grid_size as f32;
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            if positions.len() >= count { break; }
            
            // Add some randomness to grid positions
            let x = (i as f32 * x_step) + rng.gen::<f32>() * x_step * 0.3;
            let y = (j as f32 * y_step) + rng.gen::<f32>() * y_step * 0.3;
            positions.push((x, y));
        }
    }
    
    // Fill remaining with random positions
    while positions.len() < count {
        positions.push((
            rng.gen::<f32>() * params.width as f32,
            rng.gen::<f32>() * params.height as f32,
        ));
    }
    
    positions
}

fn generate_poisson_positions(params: &GrainParams, count: usize, rng: &mut ThreadRng) -> Vec<(f32, f32)> {
    // Simplified Poisson disk sampling
    let mut positions = Vec::new();
    let min_distance = 3.0; // Minimum distance between grains
    let max_attempts = 30;
    
    while positions.len() < count {
        let mut attempts = 0;
        let mut valid_position = None;
        
        while attempts < max_attempts {
            let candidate = (
                rng.gen::<f32>() * params.width as f32,
                rng.gen::<f32>() * params.height as f32,
            );
            
            let mut valid = true;
            for &(px, py) in &positions {
                let dx = candidate.0 - px;
                let dy = candidate.1 - py;
                if ((dx * dx + dy * dy) as f32).sqrt() < min_distance {
                    valid = false;
                    break;
                }
            }
            
            if valid {
                valid_position = Some(candidate);
                break;
            }
            attempts += 1;
        }
        
        if let Some(pos) = valid_position {
            positions.push(pos);
        } else {
            // Fallback to random if we can't find valid Poisson position
            positions.push((
                rng.gen::<f32>() * params.width as f32,
                rng.gen::<f32>() * params.height as f32,
            ));
        }
    }
    
    positions
}

// 🚀 NEW: Get shape-specific size factor
fn get_shape_size_factor(shape: &str, rng: &mut ThreadRng) -> f32 {
    match shape {
        "Sigma grain" => rng.gen_range(0.95..1.05),      // Very uniform size
        "extremely_fine" => rng.gen_range(0.9..1.1),     // Consistent fine grain
        "fine_irregular" => rng.gen_range(0.8..1.3),     // Moderate variation
        "tabular" => rng.gen_range(0.7..1.4),            // T-grain variation
        "irregular" => rng.gen_range(0.6..1.6),          // High variation
        "cubic" => rng.gen_range(0.5..1.8),              // Very high variation
        _ => rng.gen_range(0.8..1.2),                     // Default variation
    }
}

// 🚀 NEW: Get shape-specific shape factor
fn get_shape_factor(shape: &str, rng: &mut ThreadRng) -> f32 {
    match shape {
        "Sigma grain" => rng.gen_range(0.95..1.05),      // Nearly circular
        "extremely_fine" => rng.gen_range(0.9..1.1),     // Very round
        "tabular" => rng.gen_range(0.6..0.9),            // Elongated platelets
        "fine_irregular" => rng.gen_range(0.8..1.2),     // Slightly irregular
        "irregular" => rng.gen_range(0.5..1.5),          // Quite irregular
        "cubic" => rng.gen_range(0.4..1.6),              // Very irregular
        _ => rng.gen_range(0.7..1.3),                     // Default
    }
}

// 🚀 NEW: Render color film grain with multi-layer simulation
fn render_color_film_grain(_grain: &Grain, stock: &FilmStock, params: &GrainParams) -> (u8, u8, u8) {
    FILM_COLORS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(&color) = cache.get(&stock.basic_info.name) {
            color
        } else {
            let (base_r, base_g, base_b) = get_film_grain_color(&stock.basic_info.name);
            
            // 🚀 Multi-layer color film simulation
            // Color films have 3 separate emulsion layers with different characteristics
            let mut rng = thread_rng();
            
            // Cyan layer (top) - affects red channel
            let cyan_strength = rng.gen_range(0.85..1.15);
            // Magenta layer (middle) - affects green channel  
            let magenta_strength = rng.gen_range(0.85..1.15);
            // Yellow layer (bottom) - affects blue channel
            let yellow_strength = rng.gen_range(0.85..1.15);
            
            // Apply layer variations
            let layer_r = (base_r as f32 * cyan_strength).clamp(0.0, 255.0);
            let layer_g = (base_g as f32 * magenta_strength).clamp(0.0, 255.0);
            let layer_b = (base_b as f32 * yellow_strength).clamp(0.0, 255.0);
            
            // Apply JSON color cast
            let (cast_r, cast_g, cast_b) = get_json_color_cast_multiplier(&stock.color_properties.primary_cast);
            
            // Apply color crossover effects
            let mut grain_color = [layer_r / 255.0, layer_g / 255.0, layer_b / 255.0];
            if let Some(enhanced) = enhanced_data_cached().get(&params.film_stock) {
                apply_color_crossover(&mut grain_color, &enhanced.color_crossover);
            }
            
            // Apply JSON color cast
            grain_color[0] *= cast_r;
            grain_color[1] *= cast_g;
            grain_color[2] *= cast_b;
            
            let final_color = (
                (grain_color[0] * 255.0).clamp(0.0, 255.0) as u8,
                (grain_color[1] * 255.0).clamp(0.0, 255.0) as u8,
                (grain_color[2] * 255.0).clamp(0.0, 255.0) as u8
            );
            cache.insert(stock.basic_info.name.clone(), final_color);
            final_color
        }
    })
}

// 🚀 NEW: Render B&W film grain
fn render_bw_film_grain(_grain: &Grain, stock: &FilmStock, params: &GrainParams) -> (u8, u8, u8) {
    FILM_COLORS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(&color) = cache.get(&stock.basic_info.name) {
            color
        } else {
            let (r, g, b) = get_film_grain_color(&stock.basic_info.name);
            
            // Apply JSON color cast (for toned B&W films)
            let (cast_r, cast_g, cast_b) = get_json_color_cast_multiplier(&stock.color_properties.primary_cast);
            
            // Apply color crossover effects
            let mut grain_color = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
            if let Some(enhanced) = enhanced_data_cached().get(&params.film_stock) {
                apply_color_crossover(&mut grain_color, &enhanced.color_crossover);
            }
            
            // Apply JSON color cast
            grain_color[0] *= cast_r;
            grain_color[1] *= cast_g;
            grain_color[2] *= cast_b;
            
            let final_color = (
                (grain_color[0] * 255.0).clamp(0.0, 255.0) as u8,
                (grain_color[1] * 255.0).clamp(0.0, 255.0) as u8,
                (grain_color[2] * 255.0).clamp(0.0, 255.0) as u8
            );
            cache.insert(stock.basic_info.name.clone(), final_color);
            final_color
        }
    })
}

// 🚀 NEW: Get ISO-based irregularity factor
fn get_iso_irregularity_factor(iso: u32) -> f32 {
    match iso {
        25..=100 => 0.1,      // Very uniform grain
        101..=400 => 0.2,     // Moderate variation
        401..=800 => 0.4,     // More irregular
        801..=1600 => 0.6,    // Much more irregular
        _ => 0.8,             // Very irregular, chaotic
    }
}

// 🚀 NEW: Apply realistic aging effects based on UI parameters
fn apply_realistic_aging_effects(opacity: f32, _size: f32, age_years: f32, storage_temp: f32, film_type: &str) -> f32 {
    // Storage condition multiplier
    let storage_multiplier = if storage_temp < 10.0 { 
        0.3 // Refrigerated storage (much slower aging)
    } else if storage_temp < 20.0 {
        0.6 // Cool storage
    } else {
        1.0 // Room temperature storage
    };
    
    let effective_age = age_years * storage_multiplier;
    
    // Film type aging characteristics
    let aging_factor = match film_type {
        "color" => {
            // Color films age more noticeably
            // Increased fog, color shifts, more prominent grain
            (effective_age * 0.12).min(0.6) // Max 60% aging effect
        },
        "bw" => {
            // B&W films age more gracefully
            // Slight increase in grain, minimal fog
            (effective_age * 0.08).min(0.4) // Max 40% aging effect
        },
        _ => (effective_age * 0.1).min(0.5), // Default
    };
    
    // Apply aging to opacity (more prominent grain with age)
    let aged_opacity = opacity * (1.0 + aging_factor * 0.3);
    
    // Cap opacity to prevent over-aging
    aged_opacity.min(0.85)
}

fn apply_color_crossover(grain_color: &mut [f32; 3], crossover: &ColorCrossover) {
    // Skip crossover for B&W films (all coefficients = 1.0)
    if crossover.red_to_green >= 1.0 { return; }

    // Coefficient values in more.json are the fraction of one dye layer leaking into
    // another (typically 0.01-0.05, larger for CineStill). Apply at face value; the
    // previous 0.1 damping made the effect negligible.
    const STRENGTH: f32 = 1.0;
    let original = *grain_color;
    grain_color[0] += (original[1] * crossover.green_to_red + original[2] * crossover.blue_to_red) * STRENGTH;
    grain_color[1] += (original[0] * crossover.red_to_green + original[2] * crossover.blue_to_green) * STRENGTH;
    grain_color[2] += (original[0] * crossover.red_to_blue + original[1] * crossover.green_to_blue) * STRENGTH;

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
    // 🆕 ENHANCEMENT 11: Enhanced color data loading with better fallbacks
    let color_data = include_str!("../../color.json");
    if let Ok(colors_json) = serde_json::from_str::<serde_json::Value>(color_data) {
        if let Some(film_color) = colors_json.get(film_name) {
            if let Some(base_color) = film_color.get("base_grain_color") {
                let r = base_color.get("r").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                let g = base_color.get("g").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                let b = base_color.get("b").and_then(|v| v.as_u64()).unwrap_or(180) as u8;
                
                // Apply enhanced color variation
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
    
    // 🆕 ENHANCEMENT 12: Intelligent fallback colors (only when JSON fails)
    println!("⚠️ Using fallback color for: {}", film_name);
    if film_name.contains("Tri-X") {
        (175, 175, 175) // Classic B&W grain
    } else if film_name.contains("HP5") {
        (170, 170, 170) // Slightly darker B&W
    } else if film_name.contains("T-Max") {
        (185, 185, 185) // Fine B&W grain
    } else {
        (180, 180, 180) // Neutral gray fallback
    }
}

// 🆕 ENHANCEMENT 13: Add halation effect for CineStill films
fn apply_halation_effect(grains: &mut Vec<Grain>, stock: &FilmStock, params: &GrainParams) {
    if stock.special_effects.halation == "strong" {
        let mut rng = thread_rng();
        let mut halation_grains = Vec::new();

        for grain in grains.iter() {
            // Halation bleeds from the brightest grains into the red-sensitive layer,
            // producing a soft red-orange glow around highlights.
            if grain.opacity > 0.20 && rng.gen::<f32>() < 0.30 {
                let halation_distance = stock.special_effects.halation_radius
                    * grain.size
                    * rng.gen_range(2.0..5.0);
                let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;

                let halation_grain = Grain {
                    x: grain.x + angle.cos() * halation_distance,
                    y: grain.y + angle.sin() * halation_distance,
                    size: grain.size * rng.gen_range(1.5..3.0), // wider, softer glow
                    opacity: grain.opacity * rng.gen_range(0.08..0.22), // dim
                    shape_factor: 1.0, // circular
                    halation: true,
                };

                // Only add if within bounds
                if halation_grain.x >= 0.0 && halation_grain.y >= 0.0 && 
                   halation_grain.x < params.width as f32 && halation_grain.y < params.height as f32 {
                    halation_grains.push(halation_grain);
                }
            }
        }
        
        grains.extend(halation_grains);
        println!("🌟 Applied halation effect for {}", stock.basic_info.name);
    }
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
    // Sourced grain metrics (manufacturer datasheets where available)
    iso: Option<u32>,
    graininess: Option<String>,
    morphology: Option<String>,
    grain_size_um: Option<String>,
    push_range: Option<String>,
    resolving_power: Option<String>,
}

fn fmt_graininess(g: &serde_json::Value) -> Option<String> {
    let metric = g.get("metric")?.as_str()?;
    let value = g.get("value")?.as_u64()?;
    let scale = g.get("scale").and_then(|v| v.as_str()).unwrap_or("");
    let conf = g.get("confidence").and_then(|v| v.as_str()).unwrap_or("");
    let label = match metric {
        "pgi" => format!("Print Grain Index {} (35 mm, 4x6 in print)", value),
        _ => {
            let origin = match scale {
                "kodak_bw_rms_48um" => "Kodak B&W",
                "fuji_bw_rms_48um" => "Fujifilm B&W",
                "fuji_colneg_rms_48um" => "Fujifilm colour negative",
                "reverse_rms_48um" => "reversal film",
                "third_party_estimate" => "class estimate",
                _ => "diffuse",
            };
            format!("RMS {} ({}, 48 um aperture, D=1.0)", value, origin)
        }
    };
    Some(if conf == "datasheet" {
        label
    } else {
        format!("{} [{}]", label, conf.replace('_', " "))
    })
}

fn fmt_morphology(m: &serde_json::Value) -> Option<String> {
    let s = match m.get("family")?.as_str()? {
        "tabular" => "Tabular grain (T-GRAIN)",
        "core_shell" => "Tabular grain (core-shell)",
        "sigma" => "Sigma grain",
        "cubic" => "Cubic silver halide",
        other => other,
    };
    Some(s.to_string())
}

fn fmt_size(sz: &serde_json::Value) -> Option<String> {
    let min = sz.get("min")?.as_f64()?;
    let mean = sz.get("mean")?.as_f64()?;
    let max = sz.get("max")?.as_f64()?;
    let est = sz.get("confidence").and_then(|v| v.as_str()) == Some("estimated");
    Some(if est {
        format!("{:.2}-{:.2} um (mean {:.2}, estimated)", min, max, mean)
    } else {
        format!("{:.2}-{:.2} um (mean {:.2})", min, max, mean)
    })
}

fn fmt_push(push: Option<&serde_json::Value>, ei_range: Option<&serde_json::Value>) -> Option<String> {
    if let Some(p) = push {
        let stops = p.get("stops").and_then(|v| v.as_u64());
        if let Some(ei) = p.get("ei").and_then(|v| v.as_array()) {
            if let (Some(lo), Some(hi)) = (
                ei.first().and_then(|v| v.as_u64()),
                ei.last().and_then(|v| v.as_u64()),
            ) {
                return Some(match stops {
                    Some(s) if s > 0 => format!("EI {} - {} (up to {} stop push)", lo, hi, s),
                    _ => format!("EI {} - {}", lo, hi),
                });
            }
        }
    }
    if let Some(r) = ei_range {
        let lo = r.get("min").and_then(|v| v.as_u64());
        let hi = r.get("max").and_then(|v| v.as_u64());
        if let (Some(lo), Some(hi)) = (lo, hi) {
            return Some(format!("EI {} - {}", lo, hi));
        }
    }
    None
}

fn fmt_resolving(r: &serde_json::Value) -> Option<String> {
    let a = r.get("toc_1_6_to_1")?.as_u64()?;
    let b = r.get("toc_1000_to_1")?.as_u64()?;
    Some(format!("{} / {} lines/mm (1.6:1 / 1000:1)", a, b))
}

#[tauri::command]
async fn get_film_info(film_name: String) -> Result<FilmInfo, String> {
    // Comprehensive film info (text) + sourced grain metrics.
    let json_data = include_str!("../../fixed.json");
    let stocks_json: serde_json::Value = serde_json::from_str(json_data)
        .map_err(|e| format!("Failed to parse fixed.json: {}", e))?;

    let grain_json = include_str!("../../grain_model.json");
    let grain: serde_json::Value = serde_json::from_str(grain_json).unwrap_or(serde_json::Value::Null);
    let gm = grain.get("films").and_then(|f| f.get(&film_name));

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
                iso: gm.and_then(|g| g.get("iso")).and_then(|v| v.as_u64()).map(|v| v as u32),
                graininess: gm.and_then(|g| g.get("graininess")).and_then(fmt_graininess),
                morphology: gm.and_then(|g| g.get("morphology")).and_then(fmt_morphology),
                grain_size_um: gm.and_then(|g| g.get("grain_size_um")).and_then(fmt_size),
                push_range: gm.and_then(|g| {
                    fmt_push(g.get("push"), g.get("ei_range"))
                }),
                resolving_power: gm
                    .and_then(|g| g.get("resolving_power_lines_mm"))
                    .and_then(fmt_resolving),
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
    
    // 🆕 ENHANCEMENT 7: Extract prominence data from JSON
    let prominence = visual_props.get("prominence").unwrap_or(&serde_json::Value::Null);
    let highlight_visibility = prominence.get("highlights").and_then(|v| v.as_str()).unwrap_or("medium").to_string();
    let shadow_visibility = prominence.get("shadows").and_then(|v| v.as_str()).unwrap_or("medium").to_string();
    let midtone_prominence = prominence.get("midtones").and_then(|v| v.as_str()).unwrap_or("medium").to_string();
    
    // Extract digital simulation parameters
    let _grains_per_1024 = digital_sim.get("grains_per_1024px").and_then(|v| v.as_u64()).unwrap_or(400) as u32;
    
    // 🆕 ENHANCEMENT 8: Determine ISO from film name (more accurate)
    let iso = if name.contains("3200") { 3200 }
        else if name.contains("1600") { 1600 }
        else if name.contains("800") { 800 }
        else if name.contains("400") { 400 }
        else if name.contains("200") { 200 }
        else if name.contains("160") { 160 }
        else if name.contains("125") { 125 }
        else if name.contains("100") { 100 }
        else if name.contains("50") { 50 }
        else if name.contains("25") { 25 }
        else { 400 }; // Default
    
    // 🆕 ENHANCEMENT 9: Enhanced crystal type detection
    let crystal_type = if name.contains("T-Max") || name.contains("Delta") || name.contains("Ektar") || name.contains("Portra") {
        "tabular".to_string()
    } else if name.contains("Sigma") || name.contains("Acros") {
        "sigma".to_string()
    } else {
        "cubic".to_string()
    };
    
    // 🆕 ENHANCEMENT 10: Enhanced aspect ratio based on crystal type
    let aspect_ratio = match crystal_type.as_str() {
        "tabular" => vec![3.0, 1.0],  // T-grain is flatter
        "sigma" => vec![1.2, 1.0],    // Sigma grain is slightly elongated
        _ => vec![1.0, 1.0],          // Cubic is square
    };
    
    Ok(FilmStock {
        basic_info: BasicInfo {
            name: name.to_string(),
            iso,
            film_type: if name.contains("Tri-X") || name.contains("HP5") || name.contains("T-Max") || name.contains("Delta") || name.contains("Acros") || name.contains("Pan F") || name.contains("Neopan") || name.contains("FP4") || name.contains("Plus-X") || name.contains("Technical Pan") { "bw".to_string() } else { "color".to_string() },
        },
        grain_structure: GrainStructure {
            crystal_type,
            shape: grain_chars.get("shape").and_then(|v| v.as_str()).unwrap_or("irregular").to_string(),
            aspect_ratio,
            orientation: "random".to_string(),
            clustering: density_dist.get("clustering").and_then(|v| v.as_str()).unwrap_or("moderate").to_string(),
            cluster_size: density_dist.get("cluster_size").and_then(|v| v.as_str()).unwrap_or("2-4_grains").to_string(),
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
            highlight_visibility,
            shadow_visibility,
            midtone_prominence,
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
            halation: if name.contains("CineStill") { "strong".to_string() } else { "none".to_string() },
            halation_color: "#ffffff".to_string(),
            halation_radius: if name.contains("CineStill") { 2.0 } else { 1.0 },
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
        .invoke_handler(tauri::generate_handler![generate_grain, save_grain_image, get_categorized_film_stocks, get_film_info, save_composite_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}// Performance optimizations applied - 8x faster rendering
// Test comment to trigger v1.0.5

#[cfg(test)]
mod tests {
    use super::*;

    fn model(metric: &str, value: f64, scale: &str) -> GrainModel {
        GrainModel {
            iso: Some(400),
            film_type: Some("bw_neg".to_string()),
            graininess: Some(GraininessMetric {
                metric: Some(metric.to_string()),
                value: Some(value),
                scale: Some(scale.to_string()),
            }),
            morphology: Some(GrainMorphology { family: Some("tabular".to_string()) }),
            grain_size_um: Some(GrainSizeModel { min: 0.4, mean: 0.9, max: 2.0, sigma: Some(0.35) }),
            push: None,
            ei_range: None,
        }
    }

    fn morph(family: &str) -> GrainModel {
        GrainModel {
            iso: None,
            film_type: None,
            graininess: None,
            morphology: Some(GrainMorphology { family: Some(family.to_string()) }),
            grain_size_um: None,
            push: None,
            ei_range: None,
        }
    }

    #[test]
    fn pgi_normalises_to_unit_range() {
        assert!((grain_index(Some(&model("pgi", 48.0, "kodak_pgi")), 800) - 1.0).abs() < 1e-6);
        assert!(grain_index(Some(&model("pgi", 25.0, "kodak_pgi")), 800).abs() < 1e-6);
        let mid = grain_index(Some(&model("pgi", 36.5, "kodak_pgi")), 800);
        assert!((mid - 0.5).abs() < 0.01, "got {}", mid);
    }

    #[test]
    fn bw_rms_normalises_and_tri_x_is_high() {
        assert!(grain_index(Some(&model("rms_diffuse", 6.0, "kodak_bw_rms_48um")), 100).abs() < 1e-6);
        assert!((grain_index(Some(&model("rms_diffuse", 22.0, "kodak_bw_rms_48um")), 400) - 1.0).abs() < 1e-6);
        let tri = grain_index(Some(&model("rms_diffuse", 17.0, "kodak_bw_rms_48um")), 400);
        assert!(tri > 0.6 && tri < 0.75, "Tri-X index was {}", tri);
    }

    #[test]
    fn falls_back_to_iso_when_not_published() {
        assert_eq!(grain_index(None, 50), 0.0);
        assert_eq!(grain_index(None, 3200), 1.0);
        assert!(grain_index(None, 800) > grain_index(None, 100));
    }

    #[test]
    fn morphology_maps_to_renderer_shape_keys() {
        assert_eq!(morphology_shape_key(Some(&morph("tabular")), "x"), "tabular");
        assert_eq!(morphology_shape_key(Some(&morph("core_shell")), "x"), "tabular");
        assert_eq!(morphology_shape_key(Some(&morph("sigma")), "x"), "Sigma grain");
        assert_eq!(morphology_shape_key(Some(&morph("cubic")), "x"), "irregular");
        assert_eq!(morphology_shape_key(None, "custom"), "custom");
    }

    #[test]
    fn grain_model_file_covers_all_dropdown_films() {
        let stocks = load_film_stock_data().expect("fixed.json should parse");
        let models = grain_models();
        for name in stocks.keys() {
            assert!(models.contains_key(name), "grain_model.json missing {}", name);
        }
    }

    #[test]
    fn measured_graininess_drives_grain_amplitude() {
        let stocks = load_film_stock_data().expect("fixed.json should parse");
        let models = grain_models();
        let params = GrainParams {
            film_stock: String::new(),
            exposure_compensation: 0.0,
            size_multiplier: 1.0,
            contrast: 100.0,
            grain_density: 1000,
            width: 256,
            height: 256,
            background: "transparent".to_string(),
            film_age_years: Some(0.0),
            storage_temp: Some(20.0),
        };
        let mean_opacity = |name: &str| {
            let stock = &stocks[name];
            let grains = generate_grains_advanced(stock, &params, None, models.get(name))
                .expect("generation should succeed");
            grains.iter().map(|g| g.opacity).sum::<f32>() / grains.len() as f32
        };
        // Coarse measured grain (Tri-X, RMS 17) must render stronger than a
        // fine measured grain (Pan F, RMS 7).
        let tri_x = mean_opacity("Kodak Tri-X 400");
        let pan_f = mean_opacity("Ilford Pan F Plus 50");
        assert!(tri_x > pan_f, "Tri-X {:.3} should exceed Pan F {:.3}", tri_x, pan_f);
    }

    fn params() -> GrainParams {
        GrainParams {
            film_stock: String::new(),
            exposure_compensation: 0.0,
            size_multiplier: 1.0,
            contrast: 100.0,
            grain_density: 1000,
            width: 256,
            height: 256,
            background: "transparent".to_string(),
            film_age_years: Some(0.0),
            storage_temp: Some(20.0),
        }
    }

    #[test]
    fn finer_grain_yields_more_grains() {
        let stocks = load_film_stock_data().expect("fixed.json should parse");
        let models = grain_models();
        let p = params();
        let count = |name: &str| {
            generate_grains_advanced(&stocks[name], &p, None, models.get(name))
                .unwrap()
                .len()
        };
        // Pan F (fine, ~0.41 um) should place far more grains than Delta 3200 (~1.6 um).
        assert!(
            count("Ilford Pan F Plus 50") > count("Ilford Delta 3200"),
            "fine grain should need more grains"
        );
    }

    #[test]
    fn cluster_size_is_parsed_from_json() {
        let stocks = load_film_stock_data().expect("fixed.json should parse");
        assert_eq!(stocks["Kodak Tri-X 400"].grain_structure.cluster_size, "2-4_grains");
        assert_eq!(parse_json_cluster_size_range("2-4_grains"), (2, 4));
        assert_eq!(parse_json_cluster_size_range("3-5_grains"), (3, 5));
        assert_eq!(
            parse_json_cluster_size_range(&stocks["Kodak Tri-X 400"].grain_structure.cluster_size),
            (2, 4)
        );
    }

    #[test]
    fn cinestill_halation_produces_flagged_red_grains() {
        let stocks = load_film_stock_data().expect("fixed.json should parse");
        let models = grain_models();
        let p = params();
        let mut grains =
            generate_grains_advanced(&stocks["CineStill 800T"], &p, None, models.get("CineStill 800T"))
                .unwrap();
        apply_halation_effect(&mut grains, &stocks["CineStill 800T"], &p);
        assert!(
            grains.iter().any(|g| g.halation),
            "CineStill 800T should produce halation grains"
        );
        // And a non-halation film should not.
        let mut plain =
            generate_grains_advanced(&stocks["Kodak Ektar 100"], &p, None, models.get("Kodak Ektar 100"))
                .unwrap();
        let before = plain.len();
        apply_halation_effect(&mut plain, &stocks["Kodak Ektar 100"], &p);
        assert_eq!(plain.len(), before, "Ektar must not gain halation grains");
    }

    #[test]
    fn emulsion_class_orders_grain_visibility() {
        let mk = |t: &str| GrainModel {
            iso: None,
            film_type: Some(t.to_string()),
            graininess: None,
            morphology: None,
            grain_size_um: None,
            push: None,
            ei_range: None,
        };
        assert!(class_weight(Some(&mk("bw_neg"))) > class_weight(Some(&mk("color_neg"))));
        assert!(class_weight(Some(&mk("color_neg"))) > class_weight(Some(&mk("color_rev"))));
        assert_eq!(class_weight(None), 1.0);
    }

    #[test]
    fn push_latitude_uses_datasheet_data() {
        let models = grain_models();
        // Tri-X 400 datasheet push table: up to 3 stops (EI 3200)
        let tri = push_latitude_stops(models.get("Kodak Tri-X 400"));
        assert!((tri - 3.0).abs() < 0.01, "Tri-X latitude was {}", tri);
        // Velvia 50: EI 50 -> 100 = 1 stop
        let vel = push_latitude_stops(models.get("Fuji Velvia 50"));
        assert!((vel - 1.0).abs() < 0.01, "Velvia latitude was {}", vel);
        // Portra 800: EI 800 -> 3200 = 2 stops
        let p800 = push_latitude_stops(models.get("Kodak Portra 800"));
        assert!((p800 - 2.0).abs() < 0.01, "Portra 800 latitude was {}", p800);
        // Ektar 100 publishes no push -> colour-negative class default (2)
        assert!(push_latitude_stops(models.get("Kodak Ektar 100")) >= 1.0);
    }

    #[test]
    fn push_raises_grain_more_on_high_latitude_films() {        let models = grain_models();
        let op = |name: &str, stops: f32| push_response(models.get(name), stops).1;
        // At +3 stops, Tri-X (latitude 3) gains more grain than Velvia (latitude 1).
        assert!(op("Kodak Tri-X 400", 3.0) > op("Fuji Velvia 50", 3.0));
        // At 0 stops there is no change.
        assert!((op("Kodak Tri-X 400", 0.0) - 1.0).abs() < 1e-6);
        // Pulling reduces grain.
        assert!(op("Kodak Tri-X 400", -2.0) < 1.0);
    }

    /// End-to-end render: generate + rasterise a handful of films, assert the output
    /// contains visible grain with the expected ordering, and write preview PNGs.
    #[test]
    fn renders_visible_grain_and_writes_previews() {
        let stocks = load_film_stock_data().expect("fixed.json parses");
        let models = grain_models();
        let p = params(); // 256x256, density 1.0x, opacity 100%

        let render = |name: &str| -> (usize, f32, f32, f32, Vec<u8>) {
            let stock = &stocks[name];
            let grains = generate_grains_advanced(stock, &p, None, models.get(name)).unwrap();
            let mean_op = grains.iter().map(|g| g.opacity).sum::<f32>() / grains.len() as f32;
            let mean_size = grains.iter().map(|g| g.size).sum::<f32>() / grains.len() as f32;
            let data = render_grains_parallel(&grains, &p, stock).unwrap();
            let npx = data.len() / 4;
            let mut a_sum: u64 = 0;
            for i in 0..npx {
                a_sum += data[i * 4 + 3] as u64;
            }
            (grains.len(), mean_op, mean_size, a_sum as f32 / npx as f32, data)
        };

        let (tri_n, tri_op, tri_size, tri_a, _) = render("Kodak Tri-X 400");
        let (vel_n, vel_op, vel_size, vel_a, _) = render("Fuji Velvia 50");
        assert!(tri_n > 0 && vel_n > 0, "grains should be generated");
        assert!(tri_a > 0.0 && vel_a > 0.0, "render should contain visible grain");
        // Per-grain amplitude follows the sourced graininess (Tri-X RMS 17 > Velvia RMS 9)
        assert!(
            tri_op > vel_op,
            "Tri-X grain opacity ({:.3}) should exceed Velvia ({:.3})",
            tri_op, vel_op
        );
        // Per-grain size follows the sourced grain size (Tri-X coarser than Velvia)
        assert!(
            tri_size > vel_size,
            "Tri-X grain size ({:.3}) should exceed Velvia ({:.3})",
            tri_size, vel_size
        );

        // Write black-composited previews so the output can be eyeballed.
        let out = std::path::Path::new("target/preview");
        std::fs::create_dir_all(out).ok();
        for name in [
            "Kodak Tri-X 400",
            "Kodak T-Max 100",
            "Kodak Portra 400",
            "Kodak Ektar 100",
            "Fuji Velvia 50",
            "Fuji Acros 100",
            "Ilford HP5 Plus",
            "CineStill 800T",
        ] {
            let stock = &stocks[name];
            let grains = generate_grains_advanced(stock, &p, None, models.get(name)).unwrap();
            let data = render_grains_parallel(&grains, &p, stock).unwrap();
            let mut img: RgbaImage =
                ImageBuffer::from_raw(p.width, p.height, data).expect("valid image buffer");
            for px in img.pixels_mut() {
                let a = px[3] as u16;
                px[0] = ((px[0] as u16 * a) / 255) as u8;
                px[1] = ((px[1] as u16 * a) / 255) as u8;
                px[2] = ((px[2] as u16 * a) / 255) as u8;
                px[3] = 255;
            }
            let file = out.join(format!("{}.png", name.replace(' ', "_")));
            img.save(&file).expect("preview should save");
        }
    }

    #[test]
    fn color_crossover_shifts_colour_films_only() {
        let cross = ColorCrossover {
            red_to_green: 0.05, red_to_blue: 0.03, green_to_red: 0.04,
            green_to_blue: 0.05, blue_to_red: 0.02, blue_to_green: 0.02,
        };
        let mut c = [0.7f32, 0.7, 0.7];
        apply_color_crossover(&mut c, &cross);
        assert!(
            (c[0] - 0.7).abs() > 0.005 || (c[1] - 0.7).abs() > 0.005 || (c[2] - 0.7).abs() > 0.005,
            "crossover should visibly shift the colour: {:?}", c
        );
        // B&W crossover (all 1.0) leaves the colour untouched
        let bw = ColorCrossover {
            red_to_green: 1.0, red_to_blue: 1.0, green_to_red: 1.0,
            green_to_blue: 1.0, blue_to_red: 1.0, blue_to_green: 1.0,
        };
        let mut d = [0.7f32, 0.7, 0.7];
        apply_color_crossover(&mut d, &bw);
        assert_eq!(d, [0.7, 0.7, 0.7]);
    }
}
