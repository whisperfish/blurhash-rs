use super::base83::encode as encode83;
use super::utils::{linear_to_srgb, srgb_to_linear, sign_pow};
use std::f64::consts::PI;

pub fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgb: &Vec<u8>,
) -> String {
    if components_x < 1 || components_x > 9 || components_y < 1 || components_y > 9 {
        panic!("BlurHash must have between 1 and 9 components");
    }

    let mut factors: Vec<Vec<f64>> = Vec::new();

    for y in 0..components_y {
        for x in 0..components_x {
            let factor = multiply_basis_function(x, y, width, height, rgb);
            factors.push(factor);
        }
    }

    let dc = &factors[0];
    let ac = &factors[1..];

    let mut hash = String::new();

    let size_flag = ((components_x - 1) + (components_y - 1) * 9) as u32;
    hash.push_str(&encode83(size_flag, 1));

    let maximum_value: f64;
    if ac.len() > 0 {
        let mut actualmaximum_value = 0.0;
        for i in 0..components_y * components_x - 1 {
            actualmaximum_value = f64::max(ac[i as usize][0], actualmaximum_value);
            actualmaximum_value = f64::max(ac[i as usize][1], actualmaximum_value);
            actualmaximum_value = f64::max(ac[i as usize][2], actualmaximum_value);
        }

        let quantised_maximum_value = f64::max(
            0.,
            f64::min(82., f64::floor(actualmaximum_value * 166. - 0.5)),
        ) as u32;
        maximum_value = (quantised_maximum_value + 1) as f64 / 166.;
        hash.push_str(&encode83(quantised_maximum_value, 1));
    } else {
        maximum_value = 1.;
        hash.push_str(&encode83(0, 1));
    }

    hash.push_str(&encode83(encode_dc(&dc), 4));

    for i in 0..components_y * components_x - 1 {
        hash.push_str(&encode83(encode_ac(&ac[i as usize], maximum_value), 2));
    }

    hash
}

fn multiply_basis_function(
    component_x: u32,
    component_y: u32,
    width: u32,
    height: u32,
    rgb: &Vec<u8>,
) -> Vec<f64> {
    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;
    let normalisation = match (component_x, component_y) {
        (0, 0) => 1.,
        _ => 2.,
    };

    let bytes_per_row = width * 4;

    for y in 0..width {
        for x in 0..height {
            let basis = f64::cos(PI * component_x as f64 * x as f64 / width as f64)
                * f64::cos(PI * component_y as f64 * y as f64 / height as f64);
            r += basis * srgb_to_linear(rgb[(4 * x + 0 + y * bytes_per_row) as usize] as u32);
            g += basis * srgb_to_linear(rgb[(4 * x + 1 + y * bytes_per_row) as usize] as u32);
            b += basis * srgb_to_linear(rgb[(4 * x + 2 + y * bytes_per_row) as usize] as u32);
        }
    }

    let scale = normalisation / (width * height) as f64;

    let result = vec![r * scale, g * scale, b * scale];

    result
}

fn encode_dc(value: &Vec<f64>) -> u32 {
    let rounded_r = linear_to_srgb(value[0]);
    let rounded_g = linear_to_srgb(value[1]);
    let rounded_b = linear_to_srgb(value[2]);
    (rounded_r << 16) + (rounded_g << 8) + rounded_b
}

fn encode_ac(value: &Vec<f64>, maximum_value: f64) -> u32 {
    let quant_r = i32::max(
        0,
        i32::min(
            18,
            f64::floor(sign_pow(value[0] / maximum_value, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_g = i32::max(
        0,
        i32::min(
            18,
            f64::floor(sign_pow(value[1] / maximum_value, 0.5) * 9. + 9.5) as i32,
        ),
    );
    let quant_b = i32::max(
        0,
        i32::min(
            18,
            f64::floor(sign_pow(value[2] / maximum_value, 0.5) * 9. + 9.5) as i32,
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
