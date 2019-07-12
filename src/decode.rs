#![allow(dead_code)]

use super::base83::decode as decode83;
use super::utils::{linear_to_srgb, sign_pow, srgb_to_linear};
use std::f64::consts::PI;

fn validate_blurhash(blurhash: &str) -> (usize, usize) {
    if blurhash.len() < 6 {
        panic!("The blurhash string must be at least 6 characters");
    }

    let size_flag = decode83(&blurhash.chars().nth(0).unwrap().to_string());
    let num_y = (f64::floor(size_flag as f64 / 9.) + 1.) as usize;
    let num_x = (size_flag % 9) + 1;

    if blurhash.len() != 4 + 2 * num_x * num_y {
        panic!(
            "blurhash length mismatch: length is {} but it should be {}",
            blurhash.len(),
            (4 + 2 * num_x * num_y)
        );
    }

    (num_x, num_y)
}

fn decode(blurhash: &str, width: u32, height: u32, punch: u32) -> Vec<u8> {
    let (num_x, num_y) = validate_blurhash(blurhash);

    let quantised_maximum_value = decode83(&blurhash.chars().nth(1).unwrap().to_string());
    let maximum_value = (quantised_maximum_value + 1) as f64 / 166.;

    let mut colors = vec![[0.0; 3]; num_x * num_y];

    for i in 0..colors.len() {
        if i == 0 {
            let value = decode83(&blurhash[2..6]);
            colors[i as usize] = decode_dc(value as u32);
        } else {
            let value = decode83(&blurhash[4 + i * 2..6 + i * 2]);
            colors[i as usize] = decode_ac(value as u32, maximum_value * punch as f64);
        }
    }

    let bytes_per_row = width * 4;
    let mut pixels = vec![0; (bytes_per_row * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut r = 0.;
            let mut g = 0.;
            let mut b = 0.;

            for j in 0..num_y {
                for i in 0..num_x {
                    let basis = f64::cos((PI * x as f64 * i as f64) / width as f64)
                        * f64::cos((PI * y as f64 * j as f64) / height as f64);
                    let color = &colors[i + j * num_x as usize];

                    r += color[0] * basis;
                    g += color[1] * basis;
                    b += color[2] * basis;
                }
            }

            let int_r = linear_to_srgb(r);
            let int_g = linear_to_srgb(g);
            let int_b = linear_to_srgb(b);

            pixels[(4 * x + 0 + y * bytes_per_row) as usize] = int_r as u8;
            pixels[(4 * x + 1 + y * bytes_per_row) as usize] = int_g as u8;
            pixels[(4 * x + 2 + y * bytes_per_row) as usize] = int_b as u8;
            pixels[(4 * x + 3 + y * bytes_per_row) as usize] = 255 as u8;
        }
    }
    pixels
}

fn decode_dc(value: u32) -> [f64; 3] {
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

fn decode_ac(value: u32, maximum_value: f64) -> [f64; 3] {
    let quant_r = f64::floor(value as f64 / (19. * 19.));
    let quant_g = f64::floor(value as f64 / 19.) % 19.;
    let quant_b = value as f64 % 19.;

    let rgb = [
        sign_pow((quant_r - 9.) / 9., 2.0) * maximum_value,
        sign_pow((quant_g - 9.) / 9., 2.0) * maximum_value,
        sign_pow((quant_b - 9.) / 9., 2.0) * maximum_value,
    ];

    rgb
}

#[cfg(test)]
mod tests {
    use super::decode;
    use image::GenericImageView;
    use image::{save_buffer, RGBA};

    #[test]
    fn decode_blurhash() {
        let img = image::open("octocat.png").unwrap();
        let (width, height) = img.dimensions();

        let img = decode("LBAdAqof00WCqZj[PDay0.WB}pof", width, height, 1);
        save_buffer("out.png", &img, width, height, RGBA(8)).unwrap();

        assert_eq!(img[0..5], [45, 1, 56, 255, 45]);
    }
}
