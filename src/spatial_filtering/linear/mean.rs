use super::{ Filter, Convolution };
use crate::{ Transformation, ImageBuffer, Pixel };

pub struct Mean;

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for Mean {
    type PO = P;

    fn transform (&self, image: ImageBuffer<P, Vec<u8>>) -> ImageBuffer<Self::PO, Vec<u8>> {
        let filter = Filter::new(1, 1, vec!(
            1, 1, 1,
            1, 1, 1,
            1, 1, 1
        ), 9);

        Convolution(filter).transform(image)
    }
}

pub struct PonderedMean;

impl <P: Pixel<Subpixel=u8> + 'static> Transformation<P> for PonderedMean {
    type PO = P;

    fn transform (&self, image: ImageBuffer<P, Vec<u8>>) -> ImageBuffer<Self::PO, Vec<u8>> {
        let filter = Filter::new(1, 1, vec!(
            1, 2, 1,
            2, 4, 2,
            1, 2, 1
        ), 16);

        Convolution(filter).transform(image)
    }
}

#[test]
fn mean() {
    let image = image::open("images/ckt_board_saltpep_prob_pt05.tif").unwrap().to_luma();
    println!("{:?}", Mean.transform(image).save("images/mean.bmp"));
}
