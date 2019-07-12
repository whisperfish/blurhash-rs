use super::util::{linear_to_srgb, srgb_to_linear};

pub fn encode(value: [f32; 3]) -> u32 {
    let rounded_r = linear_to_srgb(value[0]);
    let rounded_g = linear_to_srgb(value[1]);
    let rounded_b = linear_to_srgb(value[2]);
    (rounded_r << 16) + (rounded_g << 8) + rounded_b
}

pub fn decode(value: u32) -> [f32; 3] {
    let int_r = value >> 16;
    let int_g = (value >> 8) & 255;
    let int_b = value & 255;

    let rgb = [
        srgb_to_linear(int_r),
        srgb_to_linear(int_g),
        srgb_to_linear(int_b),
    ];

    rgb
}
