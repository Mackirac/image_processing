use crate::{ Transformation, ImageBuffer, Pixel };
use super::{ to_bin, to_dec };

pub struct ShowBits(pub [bool; 8]);

impl <PI: Pixel<Subpixel=u8> + 'static> Transformation<PI> for ShowBits {
    type PO = PI;
    
    fn transform
        (&self, mut image: ImageBuffer<PI, Vec<u8>>)
        -> ImageBuffer<Self::PO, Vec<u8>>
    {
        for pixel in image.iter_mut() {
            let mut bin_pixel = to_bin(*pixel);
            for i in 0..8 {
                bin_pixel[i] = bin_pixel[i] && self.0[i];
            }
            *pixel = to_dec(bin_pixel);
        }
        image
    }
}
