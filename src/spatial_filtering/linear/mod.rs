use super::neighborhood;
use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Filter {
    dx: u32,
    dy: u32,
    values: Vec<i16>,
    divisor: i16,
}

impl Filter {
    pub fn new (dx: u32, dy: u32, values: Vec<i16>, divisor: i16) -> Filter {
        if (1 + 2*dx) * (1 + 2*dy) != values.len() as u32 {
            panic!("Invalid values length")
        }
        Filter { dx, dy, values, divisor }
    }

    pub fn dx (&self) -> u32 { self.dx }
    pub fn dy (&self) -> u32 { self.dy }
    pub fn values (&self) -> &Vec<i16> { &self.values }
    pub fn divisor (&self) -> i16 { self.divisor }
}

pub struct Convolution(pub Filter);

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for Convolution {
    type PO = P;

    fn transform (&self, image: ImageBuffer<P, Vec<u8>>) -> ImageBuffer<Self::PO, Vec<u8>> {
        let mut output = ImageBuffer::new(image.width(), image.height());
        let channel_count = Self::PO::CHANNEL_COUNT as usize;
        let default = Self::PO::from_channels(0, 0, 0, 0);
        for x in 0..image.width() {
            for y in 0..image.height() {
                let mut channels = vec!();
                let neighborhood = neighborhood(&image, x, y, self.0.dx(), self.0.dy(), Some(&default));
                for c in 0..channel_count {
                    let mut filter = self.0.values().iter().rev();
                    let mut i : f64 = 0.0;
                    for p in neighborhood.iter() { i +=  *filter.next().unwrap() as f64 * p.channels()[c] as f64 }
                    channels.push((i / self.0.divisor() as f64).round() as u8);
                }
                output.put_pixel(x, y, *Self::PO::from_slice(&channels));
            }
        }
        output
    }
}

pub mod mean;
