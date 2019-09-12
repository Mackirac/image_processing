use crate::{ ImageBuffer, Pixel };

fn to_bin (mut dec: u8) -> [bool; 8] {
    let mut bin = [false; 8];
    for i in 0..8 {
        bin[i] = dec % 2 == 1;
        dec /= 2;
    }
    bin
}

fn to_dec (bin: [bool; 8]) -> u8 {
    let mut dec = 0;
    for i in 0..8 {
        if bin[i] { dec += 2_u8.pow(i as u32) }
    }
    dec
}

#[derive(Clone, Copy)]
pub struct Bit {
    pub px_idx: usize,
    pub px_bit: usize,
    pub value: bool
}

pub struct Bits <'a, P: Pixel<Subpixel=u8> + 'static> {
    image: &'a mut ImageBuffer<P, Vec<u8>>,
    byte: Option<[bool; 8]>,
    px_idx: usize,
    px_bit: usize
}

impl <'a, P: Pixel<Subpixel=u8> + 'static> Bits<'a, P> {
    pub fn new (image: &'a mut ImageBuffer<P, Vec<u8>>) -> Bits<'a, P> {
        let byte = image.get(0).and_then(|b| { Some(to_bin(*b)) });
        Bits { image, byte, px_idx: 0, px_bit: 1 }
    }

    pub fn set_current_byte(&mut self) {
        if let Some(byte) = self.byte {
            if let Some(pixel) = self.image.get_mut(self.px_idx) {
                *pixel = to_dec(byte);
            }
        }
    }

    pub fn set_current_bit(&mut self, value: bool) {
        if let Some(byte) = self.byte.as_mut() {
            byte[self.px_bit - 1] = value;
        }
    }
}

impl <'a, P: Pixel<Subpixel=u8> + 'static> Iterator for Bits<'a, P> {
    type Item = Bit;

    fn next (&mut self) -> Option<Self::Item> {
        if self.byte.is_none() { return None }
        if self.px_bit == 8 {
            self.set_current_byte();
            self.px_bit = 1;
            self.px_idx += 1;
            self.byte = self.image.get(self.px_idx).and_then(|b| { Some(to_bin(*b)) });
            if self.byte.is_none() { return None }
        }
        else { self.px_bit += 1 }
        Some(Bit {
            px_idx: self.px_idx,
            px_bit: self.px_bit - 1,
            value: self.byte.unwrap()[self.px_bit - 1]
        })
    }
}

mod show_bits;
pub use show_bits::*;
pub mod steganography;
