#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::ptr;

global_asm!(include_str!("start.s"));

struct Writer;

macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

macro_rules! print {
    ($($arg:tt)*) => (_print(format_args!($($arg)*)));
}

fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    Writer.write_fmt(args).unwrap();
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        const UART0: *mut u8 = 0x900_0000 as *mut u8;

        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => unsafe { ptr::write_volatile(UART0, byte) },
                // not part of printable ASCII range
                _ => unsafe { ptr::write_volatile(UART0, 0xfe) },
            }
        }

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn run() {
    println!("Hello World!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);

    loop {}
}
