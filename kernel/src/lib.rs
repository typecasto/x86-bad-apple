#![no_std]
#![allow(unused, dead_code)]

use bootloader_api::info::{FrameBufferInfo, PixelFormat};

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
    #[inline]
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
            x1 < self.info.width &&
            x2 < self.info.width &&
            y1 < self.info.height &&
            y2 < self.info.height &&
            x1 <= x2 && y1 < y2
        );
        for y in y1..y2 {
            for x in x1..x2 {
                let (r, g, b) = drawfn(x, y);
                self.put(x, y, r, g, b);
            }
        }
    }
}

#[repr(packed)]
pub struct Pixel {
    b: u8,
    g: u8,
    r: u8
}

pub struct FrameBufferTwo {

}
