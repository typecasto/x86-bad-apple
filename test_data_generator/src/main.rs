use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // generates a "sector pattern" where the 0th sector is all 0x00, the 1st sector is 0x01, etc.
    // used to get to the bottom of this disk issue. please kill me
    let mut diff_writer = BufWriter::new(File::create("./bad_apple.bin").unwrap());
    for sector in 0u8..=255 {
        diff_writer.write(&[sector; 512]).unwrap();
    }
}
