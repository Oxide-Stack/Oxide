use image::imageops::FilterType;
use std::collections::HashMap;

#[flutter_rust_bridge::frb(ignore)]
pub enum SettingsSideEffect {
    SetMainColor(String), // hex color value
}
// locates the image from path and extracts the most prominent color as main color and returns it as hex string
pub async fn extract_main_color(path: String) -> Option<String> {
    let img = match image::open(&path) {
        Ok(img) => img,
        Err(_) => return None,
    };

    // Downscale to speed up analysis while keeping the dominant color characteristics.
    let img = img.resize(64, 64, FilterType::Triangle).to_rgb8();

    let mut buckets: HashMap<(u8, u8, u8), u32> = HashMap::new();
    let quantize = |v: u8| -> u8 { (v / 32) * 32 };

    for pixel in img.pixels() {
        let [r, g, b] = pixel.0;
        let key = (quantize(r), quantize(g), quantize(b));
        *buckets.entry(key).or_insert(0) += 1;
    }

    let dominant = buckets
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|((r, g, b), _)| (r, g, b))
        .unwrap_or((0, 0, 0));

    let brightness = dominant.0 as u16 + dominant.1 as u16 + dominant.2 as u16;
    let best_color = if brightness > 600 {
        // If the image is very bright, use a darker contrasting color.
        "#1A1A1A".to_string()
    } else if brightness < 120 {
        // If the image is very dark, use a lighter contrasting color.
        "#F5F5F5".to_string()
    } else {
        format!("#{:02X}{:02X}{:02X}", dominant.0, dominant.1, dominant.2)
    };

    Some(best_color)
}
