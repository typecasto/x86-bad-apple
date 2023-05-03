use std::fs::File;
use std::io::Write;

use png::Decoder;
fn main() {
    // First frame is always black, that can just be fb.clear
    let (mut prev_frame, width, bypp) = decode(1);
    // for (i, byte) in prev_frame.iter().enumerate() {
    //     if i % bypp != 0 {continue;}
    //     let x = (i / bypp) % width as usize;
    //     let y = (i / bypp) / width as usize;
    // }
    let mut diff_writer = File::create("./bad_apple.bin").unwrap();
    let mut difft: u64 = 0;
    for n in 2..=6572 {
        // codes.push(String::new());
        let (this_frame, _, _) = decode(n);
        assert!(&this_frame.len() == &prev_frame.len());
        let mut diffs: u64 = 0;
        let mut codes: Vec<u8> = vec![];
        for (i, (this, prev)) in this_frame.iter().zip(prev_frame.iter()).enumerate() {
            if i % bypp != 0 {continue;}
            let mut this = this;
            let mut prev = prev;
            if this <= &100  {this = &0;}
            if prev <= &100  {prev = &0;}
            if this >= &200 {this = &255;}
            if prev >= &200 {prev = &255;}
            if this != prev {
            // if this.abs_diff(prev.clone()) > 15 {
                let x = (i / bypp) % width as usize;
                let y = (i / bypp) / width as usize;
                // print!(
                //     "fb.putg({1},{2},{0});\n",
                //     this, x, y
                // );
                // codes.extend((x as u16).to_le_bytes());
                // codes.extend((y as u16).to_le_bytes());
                // codes.push(this.clone());
                codes.push(1);
                codes.push(2);
                codes.push(3);
                codes.push(4);
                codes.push(5);
                diffs += 1;
                difft += 1;

            }
        }
        diff_writer.write_all(codes.as_slice());
        eprintln!("{}: {}", n, &diffs);
        prev_frame = this_frame;
    }
    eprintln!("Total diffs: {}", difft);
    
}

fn decode(n: usize) -> (Vec<u8>, u32, usize) {
    let decoder = Decoder::new(File::open(format!("frames/frame_{}.png", n)).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    (bytes.to_vec(), reader.info().width, reader.info().bytes_per_pixel())
}
