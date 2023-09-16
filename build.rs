use std::io::Write;

/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
pub fn srgb_to_linear(value: u8) -> f32 {
    let v = value as f32 / 255.;
    if v <= 0.04045 {
        v / 12.92
    } else {
        f32::powf((v + 0.055) / 1.055, 2.4)
    }
}

fn generate_srgb_lookup() -> [f32; 256] {
    let mut table = [0f32; 256];
    for (i, val) in table.iter_mut().enumerate() {
        *val = srgb_to_linear(i as u8);
    }
    table
}

fn main() {
    let table = generate_srgb_lookup();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    let mut f = std::fs::File::create(out_dir.join("srgb_lookup.rs")).unwrap();
    writeln!(f, "static SRGB_LOOKUP: [f32; 256] = {:?};", table).unwrap();
}
