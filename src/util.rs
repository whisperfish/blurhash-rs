/// linear 0.0-1.0 floating point to srgb 0-255 integer conversion.
pub fn linear_to_srgb(value: f32) -> u32 {
    let v = f32::max(0., f32::min(1., value));
    if v <= 0.003_130_8 {
        (v * 12.92 * 255. + 0.5).round() as u32
    } else {
        ((1.055 * f32::powf(v, 1. / 2.4) - 0.055) * 255. + 0.5).round() as u32
    }
}

/// srgb 0-255 integer to linear 0.0-1.0 floating point conversion.
pub fn srgb_to_linear(value: u32) -> f32 {
    let v = value as f32 / 255.;
    if v <= 0.04045 {
        v / 12.92
    } else {
        f32::powf((v + 0.055) / 1.055, 2.4)
    }
}

fn sign(n: f32) -> f32 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

pub fn sign_pow(val: f32, exp: f32) -> f32 {
    sign(val) * f32::powf(val.abs(), exp)
}
