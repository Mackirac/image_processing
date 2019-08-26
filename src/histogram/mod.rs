use crate::{ ImageBuffer, Pixel };

pub struct Histogram {
    amount: usize,
    values: Vec<[usize; 256]>
}

impl Histogram {
    pub fn new <P> (image: &ImageBuffer<P, Vec<u8>>) -> Histogram
    where P: Pixel<Subpixel=u8> + 'static
    {
        let mut amount = 0;
        let mut values = vec!();
        let channels = match P::CHANNEL_COUNT { 4 => 3, n => n } as usize;
        for _ in 0..channels { values.push([0; 256]) }
        for pixel in image.pixels() {
            let pixel = pixel.channels();
            for c in 0..channels {
                values[c][pixel[c] as usize] += 1;
            }
            amount += 1;
        }
        Histogram { amount, values }
    }

    pub fn acumulate (mut self) -> Histogram {
        for p in 1..256 {
            for c in 0..self.values.len() {
                self.values[c][p] += self.values[c][p-1];
            }
        }
        self
    }

    pub fn amount (&self) -> usize { self.amount }

    pub fn values (&self) -> &Vec<[usize; 256]> { &self.values }
}

mod equalization;
pub use equalization::*;
