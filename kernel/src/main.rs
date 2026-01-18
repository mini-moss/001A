#![no_std]
#![no_main]

use core::arch::{global_asm};
use logger::{println};

mod lang;

global_asm!(include_str!("arch/riscv/boot.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("这是伟大的第一步");
    loop {}
}
