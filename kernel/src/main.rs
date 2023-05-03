#![no_std]
#![no_main]
#![allow(unused, dead_code)]
use core::panic::PanicInfo;

use bootloader_api::info::{FrameBufferInfo, MemoryRegionKind};
use kernel::FrameBuffer;
use x86_64::instructions::port::{Port, PortReadOnly};

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
    let fb: *mut u8 = unsafe { core::mem::transmute(fb) };
    //unsafe {FB_ADDR = fb as usize};
    //unsafe {FB_INFO = Some(fbinfo.clone())};
    let fb = FrameBuffer {
        data: fb,
        info: fbinfo,
    };
    fb.clear();

    // initialize the 2nd hard drive to get the data from
    const IO_PORT: u16 = 0x1F0;
    const CTRL_PORT: u16 = 0x3F6;

    let mut data_io: Port<u16> = Port::new(IO_PORT + 0);
    let mut errors_features: Port<u8> = Port::new(IO_PORT + 1);
    let mut sector_count: Port<u8> = Port::new(IO_PORT + 2);
    let mut sector_number: Port<u8> = Port::new(IO_PORT + 3);
    let mut cyl_low: Port<u8> = Port::new(IO_PORT + 4);
    let mut cyl_high: Port<u8> = Port::new(IO_PORT + 5);
    let mut drive_select: Port<u8> = Port::new(IO_PORT + 6);
    // 0 	ERR 	Indicates an error occurred. Send a new command to clear it (or nuke it with a Software Reset).
    // 1 	IDX 	Index. Always set to zero.
    // 2 	CORR 	Corrected data. Always set to zero.
    // 3 	DRQ 	Set when the drive has PIO data to transfer, or is ready to accept PIO data.
    // 4 	SRV 	Overlapped Mode Service Request.
    // 5 	DF 	Drive Fault Error (does not set ERR).
    // 6 	RDY 	Bit is clear when drive is spun down, or after an error. Set otherwise.
    // 7 	BSY 	Indicates the drive is preparing to send/receive data (wait for it to clear). In case of 'hang' (it never clears), do a software reset.
    // read = status ^, write = commands
    let mut status_commands: Port<u8> = Port::new(IO_PORT + 7);

    // write = device ctl, read = alt status
    let mut altstatus_devicectl: Port<u8> = Port::new(CTRL_PORT + 0);
    let mut address_reg: PortReadOnly<u8> = PortReadOnly::new(CTRL_PORT + 1);

    // initialize the drive
    unsafe {
        // Select drive 1
        drive_select.write(1 << 4);
        for v in 0..15 {
            // yeah just do it 15 times, sure
            let _ = status_commands.read();
            fb.show_u8(v);
        }
        let current_status = status_commands.read(); // make that 16
        fb.show_u8(current_status);
        fb.rectangle(0, 0, 50, 50, 20, 0, 0);
        altstatus_devicectl.write(1 << 1); // Disable interrupts, we can't handle them at all
    }
    // which byte of a packet we're reading, 0 to 4 inclusive.
    let mut which = 0;
    // packets are: [YH, YL, XH, XL, DATA]
    let mut packet: (u8, u8, u8, u8, u8) = (0, 0, 0, 0, 0);
    // bytes left in the buffer
    let mut bytes_in_buffer = 0;
    // Buffer type, essentially an untagged union
    // One underlying 512-byte buffer, two logical variables that share the same data
    union Buffer {
        dbyte: [u16; 256],
        byte: [u8; 512],
    }
    let mut buffer: Buffer = unsafe { Buffer { byte: [0; 512] } };
    // LBA to read next
    let mut lba = 1u64;
    loop {
        unsafe {
            fb.show_u8_offset(status_commands.read(), 0);
            fb.show_u8_offset(errors_features.read(), 20);
            sector_count.write(1); // not fast enough to read multiple sectors lol, who cares
            sector_number.write(lba as u8); // low byte
            cyl_low.write((lba >> 8) as u8); // middle byte
            cyl_high.write((lba >> 16) as u8); // high byte
            status_commands.write(0x20); // send the READ_SECTORS command
            fb.show_u8_offset(status_commands.read(), 0);
            fb.show_u8_offset(errors_features.read(), 20);

            fb.clear();
            for _ in 0..5 {
                fb.rectangle(0, 0, fb.info.width, fb.info.height-40, 255, 0, 128);
                fb.rectangle(0, 0, fb.info.width, fb.info.height-40, 128, 0, 128);
                fb.show_u8_offset(status_commands.read(), 0);
                fb.show_u8_offset(errors_features.read(), 20);
            }
            
            for b in 0..8 {
                let data: u16 = data_io.read();
                fb.show_u8_offset(data as u8, 60 + (b * 50)); // low byte
                fb.show_u8_offset((data>>8) as u8, 80 + (b * 50)); // high byte
            }
            let data: u16 = data_io.read();
            // fb.show_u8(data as u8);
            loop {}
        }
    }

    loop {}
    // gt the available memory regions
    let mems = &boot_info.memory_regions;
    // first-fit memory allocation, no reason to use best-fit
    // allocate a region for a 2nd framebuffer to do double buffering
    // to avoid screen tearing
    // let mut fb2 = None;
    // TODO: not working
    for region in mems.iter() {
        // If this region is big enough to fit a buffer and isn't used otherwise
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 20, 0);
        if region.end - region.start >= fbinfo.byte_len as u64
            && matches!(region.kind, MemoryRegionKind::Usable)
        {
            fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 80, 0);
        }
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 0, 0, 0);
        // this is a terrible way to do memory allocation but this OS is just a project
    }
    fb.rectangle(0, 0, fb.info.width, fb.info.height, 128, 0, 0); // for debugging

    fb.clear();
    for _ in 0..20 {
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 255, 0, 128);
        fb.rectangle(0, 0, fb.info.width, fb.info.height, 128, 0, 128);
    }
    fb.clear();

    loop {}
}

bootloader_api::entry_point!(kernel_main);
