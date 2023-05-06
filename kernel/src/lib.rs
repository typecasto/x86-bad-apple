#![no_std]
#![allow(unused, dead_code)]

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use profont::PROFONT_18_POINT;

mod rle;

pub struct FrameBuffer {
    pub data: *mut u8,
    pub info: FrameBufferInfo,
}

impl FrameBuffer {
    pub fn pixel_at(&self, x: usize, y: usize) -> *mut u8 {
        debug_assert!(x < self.info.width);
        debug_assert!(y < self.info.height);
        return unsafe {
            self.data
                .add(self.info.bytes_per_pixel * ((y * self.info.stride) + x))
        };
    }
    // #[inline]
    pub fn put(&self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                unsafe { self.pixel_at(x, y).add(0).write(r) };
                unsafe { self.pixel_at(x, y).add(1).write(g) };
                unsafe { self.pixel_at(x, y).add(2).write(b) };
            }
            PixelFormat::Bgr => {
                unsafe { self.pixel_at(x, y).add(0).write(b) };
                unsafe { self.pixel_at(x, y).add(1).write(g) };
                unsafe { self.pixel_at(x, y).add(2).write(r) };
            }
            PixelFormat::U8 => {
                // Average them??? why??? idk
                unsafe { self.pixel_at(x, y).write_volatile(r / 3 + b / 3 + g / 3) };
            }
            _ => unreachable!(),
        }
    }
    #[inline]
    pub fn putg(&self, x: usize, y: usize, g: u8) {
        self.put(x, y, g, g, g);
    }
    pub fn clear(&self) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.put(x, y, 0, 0, 0);
            }
        }
    }
    /// x/y2 are exclusive
    /// x/y1 are inclusive
    pub fn rectangle(&self, x1: usize, y1: usize, x2: usize, y2: usize, r: u8, g: u8, b: u8) {
        for y in y1..y2 {
            for x in x1..x2 {
                self.put(x, y, r, g, b);
            }
        }
    }
    pub fn custom_draw(
        &self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        drawfn: impl Fn(usize, usize) -> (u8, u8, u8),
    ) {
        debug_assert!(
            x1 < self.info.width
                && x2 < self.info.width
                && y1 < self.info.height
                && y2 < self.info.height
                && x1 <= x2
                && y1 < y2
        );
        for y in y1..y2 {
            for x in x1..x2 {
                let (r, g, b) = drawfn(x, y);
                self.put(x, y, r, g, b);
            }
        }
    }
    // Shows a u8 at a set height offset, can be used to display multiple u8s at once.
    // The pinnacle of technology, I know.
    // 20 is one row.
    pub fn show_u8_offset(&self, val: u8, offset: usize) {
        for bit in 0..8 {
            self.rectangle(
                (7 - bit) as usize * self.info.width / 8,
                self.info.height - 20 - offset,
                (7 - bit + 1) as usize * self.info.width / 8,
                self.info.height - offset,
                if (val & (1 << bit) != 0) { 255 } else { 0 },
                if (val & (1 << bit) != 0) { 255 } else { 0 },
                if (val & (1 << bit) != 0) { 255 } else { 0 },
            );
            if bit != 0 {
                self.rectangle(
                    (7 - bit + 1) as usize * self.info.width / 8,
                    self.info.height - 20 - offset,
                    ( (7 - bit + 1) as usize * self.info.width / 8 ) + 2,
                    self.info.height - offset,
                    0, 255, 128
                );
            }
        }
    }
    /// Shows a u8 in binary form along the bottom of the screen
    /// Also shows an MSB to LSB indicator (white-to-black) for ease of use
    pub fn show_u8(&self, val: u8) {
        for bit in 0..8 {
            // show MSB-LSB indicator, gradient rectangle
            self.rectangle(
                (7 - bit) as usize * self.info.width / 8,
                self.info.height - 40,
                (7 - bit + 1) as usize * self.info.width / 8,
                self.info.height - 20,
                ((bit + 1) * 10),
                ((bit + 1) * 10),
                ((bit + 1) * 10),
            );
        }
        self.show_u8_offset(val, 0);
    }
    // coordinates are pixel-wise, multiply x by 12 and y by 22 for character-wise
    //
    pub fn putchar(&self, x: usize, y: usize, val: char, r: u8, g: u8, b: u8) {
        let charindex = PROFONT_18_POINT.glyph_mapping.index(val);
        let char_row = charindex / 32; // characters per row in profont src
        let char_col = charindex % 32;
        
        PROFONT_18_POINT;
    }
}

#[repr(packed)]
pub struct Pixel {
    b: u8,
    g: u8,
    r: u8,
}

pub struct FrameBufferTwo {}
