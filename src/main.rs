#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ptr;

global_asm!(include_str!("start.s"));

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn run() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;

    for &byte in HELLO.iter() {
        unsafe { ptr::write_volatile(UART0, byte) }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
