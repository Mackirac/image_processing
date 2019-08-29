use super::super::neighborhood;
use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Median(pub u32);

impl Median {
    fn median <P: Pixel<Subpixel=u8> + 'static> (pixels: Vec<&P>) -> P {
        let channels = P::CHANNEL_COUNT as usize;
        let (middle, odd) = (pixels.len() / 2, pixels.len() % 2 == 1);
        let mut values = Vec::with_capacity(channels);
        let mut medians = Vec::with_capacity(channels);
        for _ in 0..channels { values.push(vec!()) }
        for pixel in pixels {
            let pixel = pixel.channels();
            for c in 0..channels {
                values[c].push(pixel[c]);
            }
        }
        for c in 0..channels {
            values[c].sort();
            let median = if odd {
                values[c][middle+1]
            } else {
                ((values[c][middle] as u16 + values[c][middle + 1] as u16) / 2) as u8
            };
            medians.push(median);
        }

        *P::from_slice(&medians)
    }
}

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for Median {
    type PO = P;

    fn transform
        (&self, image: ImageBuffer<P, Vec<u8>>)
        -> ImageBuffer<Self::PO, Vec<u8>>
    {
        let mut output = ImageBuffer::new(image.width(), image.height());
        for x in 0..image.width() {
            for y in 0..image.height() {
                let neighborhood = neighborhood(&image, x, y, self.0, None);
                output.put_pixel(x, y, Median::median(neighborhood));
            }
        }
        output
    }
}
