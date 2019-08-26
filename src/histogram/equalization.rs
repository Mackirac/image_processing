use super::Histogram;
use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Equalization;

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for Equalization {
    type PO = P;

    fn transform (&self, mut image: ImageBuffer<P, Vec<u8>>) -> ImageBuffer<Self::PO, Vec<u8>> {
        let histogram = Histogram::new(&image).acumulate();
        let h_values = histogram.values();

        for pixel in image.pixels_mut() {
            let pixel = pixel.channels_mut();
            for c in 0..h_values.len() {
                pixel[c] = (
                    255.0 * h_values[c][pixel[c] as usize] as f64 / histogram.amount() as f64
                ).round() as u8;
            }
        }
        image
    }
}
