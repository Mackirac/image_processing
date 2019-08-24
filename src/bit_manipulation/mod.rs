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

mod show_bits;
pub use show_bits::*;
