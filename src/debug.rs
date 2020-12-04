use core::ptr;

fn dump(s: &str) {
    const UART0: *mut u8 = 0x900_0000 as *mut u8;

    for byte in s.bytes() {
        match byte {
            // printable ASCII byte or newline
            0x20..=0x7e | b'\n' => unsafe { ptr::write_volatile(UART0, byte) },
            // not part of printable ASCII range
            _ => unsafe { ptr::write_volatile(UART0, 0xfe) },
        }
    }
}

pub fn dump_mem(p: *const usize, size_in_kb: isize) {
    const PER_LINE: isize = 8;

    let lines = size_in_kb * 16;

    for i in 0..lines {
        if i % 16 == 0 {
            dump("\n");
            print_be_bytes(p as usize + 64 * (i as usize));
            dump(":\n");
        }

        for j in 0..PER_LINE {
            let v = unsafe { p.offset(i * PER_LINE + j).read_volatile() };

            print_bytes(v);
            dump(" ");
        }

        dump("\n");
    }
}

fn print_be_bytes(u: usize) {
    let bytes = u.to_be_bytes();

    dump("0x");
    for (i, byte) in bytes.iter().enumerate() {
        print_byte(*byte);
        if i % 2 == 1 && i != bytes.len() - 1 {
            dump("_");
        }
    }
}

fn print_bytes(u: usize) {
    let bytes = u.to_ne_bytes();

    dump("0x");
    for (i, byte) in bytes.iter().enumerate() {
        print_byte(*byte);
        if i % 2 == 1 && i != bytes.len() - 1 {
            dump("_");
        }
    }
}

fn print_byte(byte: u8) {
    dump(num_to_hex_str(byte / 16));
    dump(num_to_hex_str(byte % 16));

    //print!(" b");

    //for i in 0..8 {
    //match (byte >> (7 - i)) % 2 {
    //0 => print!("0"),
    //1 => print!("1"),
    //_ => unreachable!(),
    //}
    //}

    //println!("");
}

fn num_to_hex_str(b: u8) -> &'static str {
    match b {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "a",
        11 => "b",
        12 => "c",
        13 => "d",
        14 => "e",
        15 => "f",
        _ => unreachable!(),
    }
}
