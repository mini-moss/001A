#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use logger::print;

mod executor;
mod lang;

global_asm!(include_str!("../../arch/riscv/boot.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    print!("这是伟大的第一步");
    loop {}
}
