#![no_std]
#![no_main]

use crate::arch::sbi::shutdown;
use core::arch::global_asm;

#[cfg_attr(target_arch = "riscv64", path = "arch/riscv/mod.rs")]
pub mod arch;
mod lang;
mod logger;

pub use logger::_print;

global_asm!(include_str!("arch/riscv/boot/boot.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("这是伟大的第一步");
    clear_bss();
    unsafe {
        arch::init_no_cpu();
    }
    let mut i = 1;
    loop {
        i += 1;
        println!(i);
        if i == 1000 {
            panic!("Shutdown machine!");
        }
    }
}

fn clear_bss() {
    unsafe extern "C" {
        fn __bss();
        fn __bss_end();
    }
    (__bss as usize..__bss_end as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    })
}
