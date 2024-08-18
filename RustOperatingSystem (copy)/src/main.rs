#![no_std]
#![no_main]
extern crate alloc;

mod drivers;
use alloc::collections::LinkedList;
use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::null;
use linked_list_allocator::{Heap, LockedHeap};

const pagefile: usize = 2048 * 2048;
static mut heap: [u8; pagefile] = [0; pagefile];
#[global_allocator]
static mut linkedHeap: LockedHeap = LockedHeap::empty();
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {

    linkedHeap.lock().init(heap.as_mut_ptr(), pagefile);
    loop {}
}
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
