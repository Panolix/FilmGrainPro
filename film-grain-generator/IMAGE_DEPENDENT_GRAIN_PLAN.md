# Image-Dependent Grain Generation Implementation Plan

## Overview
Use imagecolors.json data to make grain generation respond realistically to the uploaded image content, just like real film would.

## Current imagecolors.json Parameters

### 1. Luminance Response
- `shadow_grain_boost`: Increase grain in dark areas (2.0-3.5x)
- `midtone_grain_normal`: Normal grain in mid-tones (0.8-1.3x) 
- `highlight_grain_reduction`: Reduce grain in bright areas (0.4-0.8x)
- `grain_size_curve`: [shadow_size, midtone_size, highlight_size]
- `clustering_by_brightness`: Grain clustering varies by brightness

### 2. Color Response  
- `red_grain_visibility`: How visible grain is in red areas (0.8-1.3)
- `green_grain_visibility`: Grain visibility in green areas (0.8-1.3)
- `blue_grain_visibility`: Grain visibility in blue areas (0.8-1.3)
- `saturation_effect`: How saturation affects grain (0.6-1.0)
- `skin_tone_optimization`: Reduce grain in skin tones (0.6-0.9)
- `sky_enhancement`: Enhance grain in sky areas (1.0-1.5)

### 3. Contrast Response
- `high_contrast_grain_boost`: More grain near edges (1.2-1.7x)
- `low_contrast_grain_reduction`: Less grain in flat areas (0.8-1.0x)
- `edge_proximity_enhancement`: Grain enhancement near edges (1.1-1.3x)
- `detail_preservation_factor`: Preserve details while adding grain (0.85-1.0)

### 4. Exposure Response
- `overexposure_grain_curve`: Grain reduction in blown highlights
- `underexposure_grain_curve`: Grain increase in underexposed areas
- `reciprocity_failure_threshold`: When grain behavior changes
- `latitude_grain_distribution`: How grain spreads across exposure range

### 5. Subject Optimization
- `portrait_grain_reduction`: Less grain for portraits (0.6-0.9)
- `landscape_grain_enhancement`: More grain for landscapes (1.0-1.3)
- `architecture_grain_sharpening`: Sharper grain for buildings (1.0-1.3)
- `night_grain_boost`: More grain for night scenes (1.4-1.9)

### 6. Lighting Response
- `tungsten_grain_warmth`: Warm lighting effects (1.0-1.1)
- `daylight_grain_neutral`: Neutral daylight (1.0)
- `fluorescent_grain_shift`: Fluorescent lighting effects (0.9-1.0)
- `mixed_lighting_variation`: Variation in mixed lighting (1.1-1.4)

## Implementation Steps

### Phase 1: Image Analysis
```rust
struct ImageAnalysis {
    luminance_map: Vec<f32>,        // Per-pixel luminance (0.0-1.0)
    color_dominance: Vec<(f32, f32, f32)>, // Per-pixel R,G,B strength
    contrast_map: Vec<f32>,         // Per-pixel local contrast
    edge_map: Vec<f32>,            // Per-pixel edge strength
    subject_map: Vec<SubjectType>, // Per-pixel subject classification
    saturation_map: Vec<f32>,      // Per-pixel saturation
}

enum SubjectType {
    SkinTone,
    Sky,
    Landscape,
    Architecture,
    Unknown,
}
```

### Phase 2: Grain Modulation
```rust
fn apply_image_dependent_grain(
    grains: &mut Vec<Grain>,
    image_analysis: &ImageAnalysis,
    image_params: &serde_json::Value,
    image_width: u32,
    image_height: u32,
) {
    for grain in grains.iter_mut() {
        let pixel_index = (grain.y as u32 * image_width + grain.x as u32) as usize;
        
        // Apply luminance response
        let luminance = image_analysis.luminance_map[pixel_index];
        let luminance_factor = get_luminance_factor(luminance, image_params);
        
        // Apply color response
        let (r, g, b) = image_analysis.color_dominance[pixel_index];
        let color_factor = get_color_factor(r, g, b, image_params);
        
        // Apply contrast response
        let contrast = image_analysis.contrast_map[pixel_index];
        let contrast_factor = get_contrast_factor(contrast, image_params);
        
        // Apply subject optimization
        let subject = &image_analysis.subject_map[pixel_index];
        let subject_factor = get_subject_factor(subject, image_params);
        
        // Combine all factors
        grain.size *= luminance_factor * color_factor * contrast_factor * subject_factor;
        grain.opacity *= luminance_factor * color_factor;
    }
}
```

### Phase 3: Frontend Integration
- Modify `generate_grain()` to accept optional image data
- Pass uploaded image pixels to Rust backend
- Apply grain modulation before rendering
- Update UI to show "Image-Dependent Mode" toggle

## Benefits
1. **Realistic grain behavior** - Matches how real film responds to different image content
2. **Professional results** - Grain appears naturally integrated with the image
3. **Film-specific characteristics** - Each film stock has unique response patterns
4. **Automatic optimization** - Less grain in faces, more in skies, etc.

## Technical Challenges
1. **Performance** - Per-pixel analysis and modulation
2. **Memory usage** - Storing analysis maps for large images
3. **Algorithm complexity** - Balancing multiple factors realistically
4. **UI complexity** - Exposing controls without overwhelming users

## Next Steps
1. Implement basic luminance analysis
2. Add simple grain modulation
3. Test with sample images
4. Expand to full feature set
5. Optimize performance