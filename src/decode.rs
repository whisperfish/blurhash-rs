#![allow(non_snake_case)]
#![allow(dead_code)]

use super::base83::decode as decode83;
use super::utils::{linearTosRGB, sRGBToLinear, signPow};
use std::f64::consts::PI;

fn validateBlurhash(blurhash: &str) -> (usize, usize) {
    if blurhash.len() < 6 {
        panic!("The blurhash string must be at least 6 characters");
    }

    let sizeFlag = decode83(&blurhash.chars().nth(0).unwrap().to_string());
    let numY = (f64::floor(sizeFlag as f64 / 9.) + 1.) as usize;
    let numX = (sizeFlag % 9) + 1;

    if blurhash.len() != 4 + 2 * numX * numY {
        panic!(
            "blurhash length mismatch: length is {} but it should be {}",
            blurhash.len(),
            (4 + 2 * numX * numY)
        );
    }

    (numX, numY)
}

fn decode(blurhash: &str, width: u32, height: u32, punch: u32) -> Vec<u8> {
    let (numX, numY) = validateBlurhash(blurhash);

    let quantisedMaximumValue = decode83(&blurhash.chars().nth(1).unwrap().to_string());
    let maximumValue = (quantisedMaximumValue + 1) as f64 / 166.;

    let mut colors = vec![vec![0.0; 3]; numX * numY];

    for i in 0..colors.len() {
        if i == 0 {
            let value = decode83(&blurhash[2..6]);
            colors[i as usize] = decodeDC(value as u32);
        } else {
            let value = decode83(&blurhash[4 + i * 2..6 + i * 2]);
            colors[i as usize] = decodeAC(value as u32, maximumValue * punch as f64);
        }
    }

    let bytesPerRow = width * 4;
    let mut pixels = vec![0; (bytesPerRow * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut r = 0.;
            let mut g = 0.;
            let mut b = 0.;

            for j in 0..numY {
                for i in 0..numX {
                    let basis = f64::cos((PI * x as f64 * i as f64) / width as f64)
                        * f64::cos((PI * y as f64 * j as f64) / height as f64);
                    let color = &colors[i + j * numX as usize];

                    r += color[0] * basis;
                    g += color[1] * basis;
                    b += color[2] * basis;
                }
            }

            let intR = linearTosRGB(r);
            let intG = linearTosRGB(g);
            let intB = linearTosRGB(b);

            pixels[(4 * x + 0 + y * bytesPerRow) as usize] = intR as u8;
            pixels[(4 * x + 1 + y * bytesPerRow) as usize] = intG as u8;
            pixels[(4 * x + 2 + y * bytesPerRow) as usize] = intB as u8;
            pixels[(4 * x + 3 + y * bytesPerRow) as usize] = 255 as u8;
        }
    }
    pixels
}

fn decodeDC(value: u32) -> Vec<f64> {
    let intR = value >> 16;
    let intG = (value >> 8) & 255;
    let intB = value & 255;
    vec![sRGBToLinear(intR), sRGBToLinear(intG), sRGBToLinear(intB)]
}

fn decodeAC(value: u32, maximumValue: f64) -> Vec<f64> {
    let quantR = f64::floor(value as f64 / (19. * 19.));
    let quantG = f64::floor(value as f64 / 19.) % 19.;
    let quantB = value as f64 % 19.;

    let rgb = vec![
        signPow((quantR - 9.) / 9., 2.0) * maximumValue,
        signPow((quantG - 9.) / 9., 2.0) * maximumValue,
        signPow((quantB - 9.) / 9., 2.0) * maximumValue,
    ];

    rgb
}

#[cfg(test)]
mod test {
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
