use ::num::Complex;
pub use super::pixel as p;

fn render(pixels: &mut[u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0 ..bounds.1 {
        for column in 0 .. bounds.0 {
            let point = p::pixel_to_point(bounds, (column, row),
                                        upper_left, lower_right);
            pixels[row * bounds.0 + column] = match self::escape_time(point, 255){
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }

}

pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex{re: 0.0, im: 0.0};
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}
