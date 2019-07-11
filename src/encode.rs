#![allow(non_snake_case)]

use super::base83::encode as encode83;
use super::utils::{linearTosRGB, sRGBToLinear, signPow};
use std::f64::consts::PI;

pub fn encode(
    xComponents: u32,
    yComponents: u32,
    width: u32,
    height: u32,
    rgb: &Vec<u8>,
) -> String {
    if xComponents < 1 || xComponents > 9 || yComponents < 1 || yComponents > 9 {
        panic!("BlurHash must have between 1 and 9 components");
    }

    let mut factors: Vec<Vec<f64>> = Vec::new();

    for y in 0..yComponents {
        for x in 0..xComponents {
            let factor = multiplyBasisFunction(x, y, width, height, rgb);
            factors.push(factor);
        }
    }

    let dc = &factors[0];
    let ac = &factors[1..];

    let mut hash = String::new();

    let sizeFlag = ((xComponents - 1) + (yComponents - 1) * 9) as u32;
    hash.push_str(&encode83(sizeFlag, 1));

    let maximumValue: f64;
    if ac.len() > 0 {
        let mut actualMaximumValue = 0.0;
        for i in 0..yComponents * xComponents - 1 {
            actualMaximumValue = f64::max(ac[i as usize][0], actualMaximumValue);
            actualMaximumValue = f64::max(ac[i as usize][1], actualMaximumValue);
            actualMaximumValue = f64::max(ac[i as usize][2], actualMaximumValue);
        }

        let quantisedMaximumValue = f64::max(
            0.,
            f64::min(82., f64::floor(actualMaximumValue * 166. - 0.5)),
        ) as u32;
        maximumValue = (quantisedMaximumValue + 1) as f64 / 166.;
        hash.push_str(&encode83(quantisedMaximumValue, 1));
    } else {
        maximumValue = 1.;
        hash.push_str(&encode83(0, 1));
    }

    hash.push_str(&encode83(encodeDC(&dc), 4));

    for i in 0..yComponents * xComponents - 1 {
        hash.push_str(&encode83(encodeAC(&ac[i as usize], maximumValue), 2));
    }

    hash
}

fn multiplyBasisFunction(
    xComponent: u32,
    yComponent: u32,
    width: u32,
    height: u32,
    rgb: &Vec<u8>,
) -> Vec<f64> {
    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;
    let normalisation = match (xComponent, yComponent) {
        (0, 0) => 1.,
        _ => 2.,
    };

    let bytesPerRow = width * 4;

    for y in 0..width {
        for x in 0..height {
            let basis = f64::cos(PI * xComponent as f64 * x as f64 / width as f64)
                * f64::cos(PI * yComponent as f64 * y as f64 / height as f64);
            r += basis * sRGBToLinear(rgb[(4 * x + 0 + y * bytesPerRow) as usize] as u32);
            g += basis * sRGBToLinear(rgb[(4 * x + 1 + y * bytesPerRow) as usize] as u32);
            b += basis * sRGBToLinear(rgb[(4 * x + 2 + y * bytesPerRow) as usize] as u32);
        }
    }

    let scale = normalisation / (width * height) as f64;

    let result = vec![r * scale, g * scale, b * scale];

    result
}

fn encodeDC(value: &Vec<f64>) -> u32 {
    let rounded_r = linearTosRGB(value[0]);
    let rounded_g = linearTosRGB(value[1]);
    let rounded_b = linearTosRGB(value[2]);
    (rounded_r << 16) + (rounded_g << 8) + rounded_b
}

fn encodeAC(value: &Vec<f64>, maximumValue: f64) -> u32 {
    let quant_r = i32::max(
        0,
        i32::min(
            18,
            f64::floor(signPow(value[0] / maximumValue, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_g = i32::max(
        0,
        i32::min(
            18,
            f64::floor(signPow(value[1] / maximumValue, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_b = i32::max(
        0,
        i32::min(
            18,
            f64::floor(signPow(value[2] / maximumValue, 0.5) * 9. + 9.5) as i32,
        ),
    );

    (quant_r * 19 * 19 + quant_g * 19 + quant_b) as u32
}

#[cfg(test)]
mod test {
    use super::encode;
    use image::GenericImageView;

    #[test]
    fn encode_img() {
        let img = image::open("octocat.png").unwrap();
        let (width, height) = img.dimensions();

        assert_eq!(
            encode(4, 3, width, height, &img.to_rgba().into_vec()),
            "LBAdAqof00WCqZj[PDay0.WB}pof"
        );
    }
}