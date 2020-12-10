#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

mod debug;
mod uart;

use core::panic::PanicInfo;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn run() {
    // Dump the first kb in RAM
    let mut driver = uart::Uart::default();
    driver.init();

    driver.force_write("Kota OS\r\n");

    loop {
        if let Some(byte) = driver.get() {
            match byte {
                81 | 113 => {
                    driver.force_write("\r\nExiting!");
                    break;
                }
                10 | 13 => {
                    driver.force_write("\r\n");
                }
                _ => {
                    let mut buffer = [0; 2];

                    driver.force_write((byte as char).encode_utf8(&mut buffer));
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    abort();
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
