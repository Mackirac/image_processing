use crate::{ ImageBuffer, Pixel };
use super::{ to_bin, to_dec, Bits };

#[derive(Debug)]
pub struct Cipher {
    bits: usize,
    filter: [bool; 8]
}

impl Cipher {
    pub fn new (pattern: String) -> Result<Cipher, String> {
        if pattern.len() != 8 { return Err("Invalid cipher pattern length".to_string()) }

        let mut bits = 0;
        let mut filter = [false; 8];
        let mut pattern = pattern.chars();
        for i in 0..8 {
            let c = pattern.next_back();
            if c == Some('1') {
                bits += 1;
                filter[i] = true
            }
            else if c != Some('0') { return Err("Invalid cipher pattern".to_string()) }
        }
        if bits == 0 { return Err("Empty cipher pattern".to_string()) }
        Ok(Cipher { bits, filter })
    }

    fn hide_character <'a, P: Pixel<Subpixel=u8> + 'static>
        (&self, character: u8, bits: &mut Bits<'a, P>)
    {
        for bit in to_bin(character).iter() {
            loop {
                match bits.next() {
                    None => return, // NO MORE BITS AVAILABLE ON THE IMAGE
                    Some(b) => {
                        if self.filter[b.px_bit] {
                            bits.set_current_bit(*bit);
                            break;
                        }
                    }
                }
            }
        }
        bits.set_current_byte();
    }

    pub fn hide <P: Pixel<Subpixel=u8> + 'static>
        (&self, message: String, image: &mut ImageBuffer<P, Vec<u8>>)
        -> Result<(), String>
    {
        let message = message.into_bytes();
        if message.len() * 8 > image.len() * self.bits {
            return Err("Image length insufficient for this message".to_string());
        }
        let mut bits = Bits::new(image);
        for character in &message {
            self.hide_character(*character, &mut bits);
        }
        self.hide_character(3, &mut bits);
        Ok(())
    }

    fn seek_character <P: Pixel<Subpixel=u8> + 'static>
        (&self, bits: &mut Bits<P>)
        -> u8
    {
        let mut character = [false; 8];
        let mut c_bit = 0;
        while c_bit < 8 {
            match bits.next() {
                None => return 3,
                Some(b) => {
                    if self.filter[b.px_bit] {
                        character[c_bit] = b.value;
                        c_bit += 1;
                    }
                }
            }
        }
        to_dec(character)
    }

    pub fn seek <P: Pixel<Subpixel=u8> + 'static>
        (&self, image: &mut ImageBuffer<P, Vec<u8>>)
        -> Result<String, String>
    {
        let mut buffer = Vec::new();
        let mut bits = Bits::new(image);
        loop {
            match self.seek_character(&mut bits) {
                3 => break,
                c => buffer.push(c)
            }
        }
        String::from_utf8(buffer).or(Err("No valid message found".to_string()))
    }
}
