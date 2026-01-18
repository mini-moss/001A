#![no_std]
#![no_main]

#[cfg_attr(target_arch = "riscv64", path = "arch/riscv/mod.rs")]
pub mod arch;

use core::arch::{global_asm};
use logger::{println};

mod lang;

global_asm!(include_str!("arch/riscv/boot/boot.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("这是伟大的第一步");
    unsafe { arch::init_no_cpu(); }
    loop {}
}
