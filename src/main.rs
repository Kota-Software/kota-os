#![no_std]
#![no_main]
#![feature(global_asm)]

mod debug;

use core::panic::PanicInfo;
use debug::dump_mem;

global_asm!(include_str!("start.s"));

#[no_mangle]
pub extern "C" fn run() {
    // Dump the first kb in RAM
    dump_mem(0x4000_0000 as *const usize, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
