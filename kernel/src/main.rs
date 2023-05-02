#![no_std]
#![no_main]
use core::panic::PanicInfo;

use bootloader_api::info::FrameBufferInfo;
use kernel::FrameBuffer;

static mut FB_ADDR: usize = 0;
static mut FB_INFO: Option<FrameBufferInfo> = None;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // todo
    loop {}
}


fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let fbinfo = boot_info.framebuffer.as_ref().unwrap().info();
    let fb = boot_info.framebuffer.as_ref().unwrap().buffer().as_ptr();
    let fb: *mut u8 = unsafe {core::mem::transmute(fb)};
    unsafe {FB_ADDR = fb as usize};
    unsafe {FB_INFO = Some(fbinfo.clone())};
    let fb = FrameBuffer{data: fb, info: fbinfo};
    fb.clear();
    fb.put(10, 20, 0, 255, 128);
    fb.rectangle(100, 200, 150, 300, 0, 255, 128);

    loop {}
}

bootloader_api::entry_point!(kernel_main);
