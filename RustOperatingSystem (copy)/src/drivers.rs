use alloc::string::String;
use core::arch::asm;
use core::ptr::{null, NonNull};

unsafe fn write(str: u8, color_code: u8){
    let vga_buffer = 0xb8000 as *mut u8;
    *vga_buffer = str;
    *vga_buffer.offset(1) = color_code;
}
unsafe fn add(str: &str, color_codes: &[u8]) {
    let buffer = 0xb8000 as *mut u8;

    for (i, (byte, &color)) in str.bytes().zip(color_codes.iter()).enumerate() {
        *buffer.offset(i as isize * 2) = byte;
        *buffer.offset(i as isize * 2 + 1) = color;
    }
}

unsafe fn remove(str: &str) {
    let buffer = 0xb8000 as *mut u8;

    for (i, _) in str.bytes().enumerate() {
        *buffer.offset(i as isize * 2) = 0;
        *buffer.offset(i as isize * 2 + 1) = 0;
    }
}

#[no_mangle]
extern "C" fn keyboard_handler() -> u8
{
    let mut b: u8 = 0;
    unsafe {
        asm!(
            "in al, 0x60",
            out("al") b,
        )
    }
    b
}

