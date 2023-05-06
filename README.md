# useless-os
Bootable tohou meme. Project for operating systems SP2023.

# Setup
It needs a bit of one-time setup before running.
1. Make sure you have qemu-system-x86_64, ffmpeg, and rust installed.
2. Run `rustup target add x86_64-unknown-none`.
3. Run `rustup component add llvm-tools`.
4. In the `bad_apple_encoder` directory:
   1. Create a `frames` directory.
   2. Run `ffmpeg -i ../bad_apple.webm frames/frame_%d.png`
   3. Run `cargo run -r`
   4. Run `cp bad_apple.bin ../`

Complete! You should now be able to run `cargo r -r` in the main directory to run
the OS.
