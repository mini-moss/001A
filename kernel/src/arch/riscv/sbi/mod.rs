pub mod supposes;
mod types;
use core::arch::asm;
pub use supposes::{console_putchar, shutdown};

#[inline(always)]
pub fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        )
    }
    ret
}
