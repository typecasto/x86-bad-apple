#![no_std]
#![no_main]
use core::panic::PanicInfo;

use bootloader_api::info::{FrameBufferInfo, MemoryRegionKind};
use kernel::FrameBuffer;

static mut FB_ADDR: usize = 0;
static mut FB_INFO: Option<FrameBufferInfo> = None;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // todo
    loop {}
}


fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // Get the framebuffer location and parameters
    let fbinfo = boot_info.framebuffer.as_ref().unwrap().info();
    let fb = boot_info.framebuffer.as_ref().unwrap().buffer().as_ptr();
    let fb: *mut u8 = unsafe {core::mem::transmute(fb)};
    //unsafe {FB_ADDR = fb as usize};
    //unsafe {FB_INFO = Some(fbinfo.clone())};
    let fb = FrameBuffer{data: fb, info: fbinfo};

    // get the available memory regions
    let mems = &boot_info.memory_regions;
    // first-fit memory allocation, no reason to use best-fit
    // allocate a region for a 2nd framebuffer to do double buffering
    // to avoid screen tearing
    let mut fb2 = None;
    for region in mems.iter() {
        // If this region is big enough to fit a buffer and isn't used otherwise
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 20, 0); // for debugging
        if region.end - region.start >= fbinfo.byte_len as u64
        && matches!(region.kind, MemoryRegionKind::Usable) {
            fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 80, 0); // for debugging
            // give it to the framebuffer
            unsafe { *(region.start as usize as *mut u8) = 2 };
            fb2 = Some(FrameBuffer{data: region.start as *mut u8, info: fbinfo});
            // break;
        }
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 0, 0); // for debugging
        // this is a terrible way to do memory allocation but this OS is just a toy
    }
    fb.rectangle(0, 0, fb.info.width, fb.info.height, 128, 0, 0); // for debugging
    let fb2 = fb2.unwrap();
    
    fb.clear();
    for _ in 0..15 {
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 255, 0, 128);
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 128, 0, 128);
    }
    fb.clear();
    for _ in 0..15 {
        unsafe { fb2.data.write(20) };
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 255, 0, 0); // for debugging
        fb2.rectangle(0, 0, fb2.info.width, fb2.info.height, 255, 50, 128);
        fb2.rectangle(0, 0, fb2.info.width, fb2.info.height, 128, 50, 128);
        // unsafe { fb2.data.copy_to(fb.data, fbinfo.byte_len) };
    }
    fb.clear();
    for _ in 0..30 {
        for r in [255, 128] {
            unsafe { fb.data.write_bytes(r, fb.info.byte_len) };
        }
    }
    fb.clear();
    for _ in 0..30 {
        for r in [255, 128] {
            for y in 0..(fb.info.height * fb.info.stride) {
                for x in 0..fb.info.width {
                }
            }
        }
    }
    loop {}
}

bootloader_api::entry_point!(kernel_main);
