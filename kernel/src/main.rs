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
        drive_select.write((1 << 4) | (0 << 6));
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
    let mut index = 0;
    // packets are: [YH, YL, XH, XL, DATA]
    let mut packet: [u8; 5] = [0; 5];
    // bytes left in the buffer
    let mut buffer_pos = 0;
    // Buffer type, essentially an untagged union
    // One underlying 512-byte buffer, two logical variables that share the same data
    union Buffer {
        word: [u16; 256],
        byte: [u8; 512],
    }
    let mut buffer: Buffer = unsafe { Buffer { byte: [0; 512] } };
    // LBA to read next
    let mut lba = 1u64;
    fb.clear();
    let mut temp_counter = 0;
    loop {
        unsafe {
            // fb.show_u8_offset(status_commands.read(), 0);
            // fb.show_u8_offset(errors_features.read(), 20);
            sector_count.write(1); // not fast enough to read multiple sectors lol, who cares
            sector_number.write(lba as u8); // low byte
            cyl_low.write((lba >> 8) as u8); // middle byte
            cyl_high.write((lba >> 16) as u8); // high byte
            fb.show_u8_offset(lba as u8, 130);
            // fb.show_u8_offset((lba >> 8) as u8, 20);
            // fb.show_u8_offset((lba >> 16) as u8, 40);
            lba += 1;
            status_commands.write(0x20); // send the READ_SECTORS command
                                         // fb.show_u8_offset(status_commands.read(), 0);
                                         // fb.show_u8_offset(errors_features.read(), 20);

            // fb.clear();
            // for _ in 0..20{
            let mut polled = false;
            while polled == false {
                //fb.rectangle(0, 0, fb.info.width, fb.info.height - 40, 255, 0, 128);
                //fb.rectangle(0, 0, fb.info.width, fb.info.height - 40, 128, 0, 128);
                for _ in 0..15 {
                    let status = status_commands.read();
                }
                let status = status_commands.read();
                // fb.show_u8_offset(status, 0);
                if status & 0x80 == 0 && status & 0x08 == 0x08 {
                    // BSY = 0, and DRQ = 1
                    polled = true;
                }
                // fb.show_u8_offset(errors_features.read(), 20);
            }

            // we're ready to read a sector into the buffer
            for i in 0..256 {
                buffer.word[i] = data_io.read();
                // if i <= 30 {
                // temp_counter+=1;
                // fb.show_u8_offset(buffer.word[i] as u8, ((temp_counter) % 31) * 20);
                // temp_counter+=1;
                // fb.show_u8_offset((buffer.word[i] >> 8) as u8, ((temp_counter) % 31) * 20);
                // }
            }
            buffer_pos = 0;
            // fb.rectangle(0, 0, fb.info.width, fb.info.height - 40, 128, 0, 128);
            for i in 0..=0x1FF {
                // fb.show_u8(i as u8);
                packet[index] = buffer.byte[i];
                index = index + 1;
                if index == 5 {
                    // index points off the end of the packet, meaning we've filled it, time to roll
                    let y = packet[1] as usize | ((packet[0] as usize) << 8);
                    let x = packet[3] as usize | ((packet[2] as usize) << 8);
                    let d = packet[4];
                    // #[cfg(debug_assertions)]
                    {
                        // fb.show_u8_offset((y>>8) as u8, 0);
                        // fb.show_u8_offset((y>>0) as u8, 20);
                        // fb.show_u8_offset((x>>8) as u8, 50);
                        // fb.show_u8_offset((x>>0) as u8, 70);
                        // fb.show_u8_offset(d, 100);
                    }
                    fb.put(x, y, d, d, d);
                    index = 0;
                }
            }
            // }
            // fb.rectangle(0, 0, fb.info.width, fb.info.height - 40, 128, 40, 128);
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
