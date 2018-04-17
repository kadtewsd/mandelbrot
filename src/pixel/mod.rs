// needs pub!!!!
//pub use not only imports it, but also publically exports it under this namespace as part of the non-canonical module graph.
// 絶対パスか public にしないと行けない
use ::num::Complex;

pub fn pixel_to_point(bounds: (usize, usize),
                   pixel: (usize, usize),
                   upper_left: Complex<f64>,
                   lower_left: Complex<f64>) -> Complex<f64>  {

    let (width, height) = (lower_left.re - upper_left.re, upper_left.im - lower_left.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width /bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_poiint((100, 100), (25, 75),
                               Complex {re: -1.0, im: 1.0},
                               Complex {re:1.0, im: -1.0}),
               Complex {re: -0.5, im: -0.5});
}

