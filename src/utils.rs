#![allow(non_snake_case)]

pub fn linearTosRGB(value: f64) -> u32 {
    let v = f64::max(0., f64::min(1., value));
    if v <= 0.0031308 {
        (v * 12.92 * 255. + 0.5).round() as u32
    } else {
        ((1.055 * f64::powf(v, 1. / 2.4) - 0.055) * 255. + 0.5).round() as u32
    }
}

pub fn sRGBToLinear(value: u32) -> f64 {
    let v = value as f64 / 255.;
    if v <= 0.04045 {
        v / 12.92
    } else {
        f64::powf((v + 0.055) / 1.055, 2.4)
    }
}

fn sign(n: f64) -> f64 {
    if n < 0. {
        -1.
    } else {
        1.
    }
}

pub fn signPow(val: f64, exp: f64) -> f64 {
    sign(val) * f64::powf(val.abs(), exp)
}
