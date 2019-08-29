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
