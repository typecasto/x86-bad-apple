#![no_std]
#![no_main]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Hello World!";
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0b; // cyan
        }
    }

    loop {}
}

bootloader_api::entry_point!(kernel_main);
