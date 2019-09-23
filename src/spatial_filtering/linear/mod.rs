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

impl Convolution {
    #[allow(dead_code)]
    fn apply_filter
        <P: Pixel<Subpixel=u8> + 'static>
        (&self, neighborhood: Vec<&P>)
        -> Vec<f64>
    {
        let mut channels = vec!();
        for c in 0..P::CHANNEL_COUNT as usize {
            let mut filter = self.0.values().iter().rev();
            let mut i : f64 = 0.0;
            for p in neighborhood.iter() {
                i += *filter.next().unwrap() as f64 * p.channels()[c] as f64
            }
            channels.push(i / self.0.divisor() as f64);
        }
        channels
    }
}

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for Convolution {
    type PO = P;

    fn transform (&self, image: ImageBuffer<P, Vec<u8>>) -> ImageBuffer<Self::PO, Vec<u8>> {
        let mut buffer = vec!();
        let default = Self::PO::from_channels(0, 0, 0, 0);
        let (mut min, mut max) = ([0_f64; 3], [255_f64; 3]);
        for y in 0..image.height() {
            for x in 0..image.width() {
                let neighborhood = neighborhood(&image, x, y, self.0.dx(), self.0.dy(), Some(&default));
                let pixel = self.apply_filter(neighborhood);
                for c in 0..(P::CHANNEL_COUNT as usize) {
                    if pixel[c] < min[c] { min[c] = pixel[c] }
                    else if pixel[c] > max[c] { max[c] = pixel[c] }
                }
                buffer.push(pixel);
            }
        }

        let mut output : Vec<u8> = vec!();
        for p in buffer {
            for c in 0..(P::CHANNEL_COUNT as usize) {
                output.push(0 + (p[c] * 255.0 / (max[c] - min[c])).floor() as u8)
            }
        }

        ImageBuffer::from_vec(image.width(), image.height(), output).unwrap()
    }
}

#[test]
fn borders() {
    let image = image::open("images/100-dollars.tif").unwrap().to_luma();
    Convolution(Filter::new(0, 0, vec!(-1), 1)).transform(image).save("images/negative.bmp").unwrap();
}

pub mod mean;
