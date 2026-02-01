use crate::prelude::Result;
use crate::syscall::{SYS_EXIT, SyscallReturn, syscall};

pub fn sys_exit(exit_code: i32) -> Result<SyscallReturn> {
    syscall(SYS_EXIT, [exit_code as usize, 0, 0]);
    Ok(SyscallReturn::Return(0))
}
