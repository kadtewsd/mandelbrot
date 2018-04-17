extern crate num;
extern crate mandelbrot; // 外部のクレートを読み込む (この場合は lib.rs)
pub use mandelbrot::rendering::*;
pub use num::Complex;
pub use std::str::FromStr;

/// try to determine if c is in the Mandelbrot set....
/// this is document!
fn main() {
    println!("Hello, world!");
    let c = Complex {re: 34.9899, im: 2.89  };
    println!("Complex... {:?}", c + c);
    let result = escape_time(c, 2);
    println!("{:?}", result);
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex {re: 0.0, im: 0.0};
    loop {
        z = z * z + c;
        println!("{}", z);
    }
}

