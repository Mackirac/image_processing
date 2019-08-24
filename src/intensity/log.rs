use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Log;

impl <PI: Pixel<Subpixel=u8> + 'static> Transformation<PI> for Log {
    type PO = PI;

    fn transform
        (&self, mut image: ImageBuffer<PI, Vec<u8>>)
        -> ImageBuffer<Self::PO, Vec<u8>>
    {
        for pixel in image.iter_mut() {
            *pixel = (255.0 * (f64::log2(1.0 + *pixel as f64 / 255.0))) as u8;
        }
        image
    }
}
