use crate::{ ImageBuffer, Pixel };

pub mod non_linear;

pub fn neighborhood
    <'a, P>
    (image: &'a ImageBuffer<P, Vec<u8>>, x: u32, y: u32, dist: u32, default: Option<&'a P>)
    -> Vec<&'a P>
where
    P: Pixel<Subpixel=u8> + 'static
{
    let mut neighborhood = vec!();
    let x_min = x.saturating_sub(dist);
    let x_max = x.saturating_add(dist);
    let y_min = y.saturating_sub(dist);
    let y_max = y.saturating_add(dist);

    for x in (x_min as u64)..(x_max as u64 + 1) {
        for y in (y_min as u64)..(y_max as u64 + 1) {
            if (x as u32) < image.width() && (y as u32) < image.height() {
                neighborhood.push(image.get_pixel(x as u32, y as u32));
            }
            else if let Some(default) = default {
                neighborhood.push(default);
            }
        }
    }
    neighborhood
}
