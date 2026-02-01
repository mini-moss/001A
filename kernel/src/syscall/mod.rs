mod write;
mod exit;

use core::arch::asm;

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;

#[derive(Debug, Clone, Copy)]
pub enum SyscallReturn {
    Return(isize),
    NoReturn,
}

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        )
    }
    ret
}
