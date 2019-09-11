use crate::{ ImageBuffer, Pixel };
use super::{ to_bin, to_dec };

pub struct Cipher;

impl Cipher {
    fn hide_character <P: Pixel<Subpixel=u8> + 'static>
        (character: u8, image: &mut ImageBuffer<P, Vec<u8>>, pixel_counter: &mut usize)
    {
        for bit in to_bin(character).iter() {
            if let Some(pixel) = image.get_mut(*pixel_counter) {
                if *bit && *pixel % 2 == 0 { *pixel += 1 }
                else if !*bit && *pixel % 2 == 1 { *pixel -= 1 }
                *pixel_counter += 1;
            }
            else {
                return;
            }
        }
    }
        
    pub fn hide <P: Pixel<Subpixel=u8> + 'static>
        (message: String, image: &mut ImageBuffer<P, Vec<u8>>)
        -> Result<(), String>
    {
        let message = message.into_bytes();
        if message.len() * 8 > image.len() {
            return Err("Image length insufficient for this message".to_string());
        }
        let mut pixel_counter = 0;
        for character in &message {
            Self::hide_character(*character, image, &mut pixel_counter);
        }
        Self::hide_character(3, image, &mut pixel_counter);
        Ok(())
    }

    pub fn seek <P: Pixel<Subpixel=u8> + 'static>
        (image: &ImageBuffer<P, Vec<u8>>)
        -> Result<String, String>
    {
        let mut buffer = Vec::new();
        let mut pixel_counter = 0;
        loop {
            let mut character = [false; 8];
            for bit in 0..8 {
                if let Some(pixel) = image.get(pixel_counter) {
                    character[bit] = *pixel % 2 == 1;
                    pixel_counter += 1;
                }
                else {
                    character = to_bin(3);
                }
            }
            match to_dec(character) {
                3 => break,
                c => buffer.push(c)
            }
        }
        String::from_utf8(buffer).or(Err("".to_string()))
    }
}
