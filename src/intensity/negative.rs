use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Negative;

impl <PI: Pixel<Subpixel=u8> + 'static> Transformation<PI> for Negative {
    type PO = PI;

    fn transform
        (&self, mut image: ImageBuffer<PI, Vec<u8>>)
        -> ImageBuffer<Self::PO, Vec<u8>>
    {
        for pixel in image.iter_mut() {
            *pixel = (*pixel as i16 - 255).abs() as u8;
        }
        image
    }
}
