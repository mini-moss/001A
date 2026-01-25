#![no_std]
#![no_main]

use kernel::{arch, println};
use core::arch::global_asm;

global_asm!(include_str!("arch/riscv/boot/boot.S"));

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("这是伟大的第一步");
    clear_bss();
    unsafe {
        arch::init_no_cpu();
    }
    loop {}
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
