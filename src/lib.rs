extern crate image;
pub use image::*;

// #[cfg(test)] pub mod tests;
pub mod intensity;
pub mod bit_manipulation;
pub mod histogram;
pub mod spatial_filtering;

pub trait Transformation <PI: Pixel<Subpixel=u8> + 'static> {
    type PO : Pixel<Subpixel=u8> + 'static;
    
    fn transform
    (&self, image: ImageBuffer<PI, Vec<u8>>)
    -> ImageBuffer<Self::PO, Vec<u8>>;
}

pub fn fourier(path: &'static str) {
    use std::f64::consts::PI as PI;
    let image = image::open(path).unwrap().to_luma();
    let mut output = image::GrayImage::new(image.width(), image.height());
    let mut buffer = Vec::<f64>::new();
    let mut higher = 0_f64;

    for i in 0..image.width() {
        for j in 0..image.height() {
            let mut t = [0_f64, 0_f64];

            for x in 0..image.width() {
                for y in 0..image.height() {
                    let pixel = image.get_pixel(x, y)[0] as f64;
                    t[0] +=
                        pixel * (2.0 * PI * (
                            (i as f64 * x as f64 / image.width() as f64) +
                            (j as f64 * y as f64 / image.height() as f64)
                        )).cos();
                    t[1] += 
                        pixel * (2.0 * PI * (
                            (i as f64 * x as f64 / image.width() as f64) +
                            (j as f64 * y as f64 / image.height() as f64)
                        )).sin();
                }
            }

            let p = f64::sqrt(
                (t[0] as f64).powi(2) +
                (t[1] as f64).powi(2)
            );
            if p > higher { higher = p }
            buffer.push(p);
        }
    }

    for x in 0..image.width() {
        for y in 0..image.height() {
            let i = (x as isize - image.width() as isize / 2).abs() as u32;
            let j = (y as isize - image.height() as isize / 2).abs() as u32;
            output.put_pixel(x, y, image::Luma([
                (buffer[(i*image.height() + j) as usize] * 255.0 / higher) as u8]
            ));
        }
    }

    // println!("{:?}", higher);
    output.save("images/fourier.bmp").unwrap();
}

#[test]
fn fourier_test() {
    fourier("icon.png");
}
