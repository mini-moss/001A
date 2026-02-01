use crate::syscall::{syscall, SyscallReturn, SYS_WRITE};
use crate::prelude::Result;

// pub fn write(fd: FileDesc, user_buf_ptr: Vaddr, user_buf_len: usize, ctx: &Context) -> Result<SyscallReturn> {
//     Ok(SyscallReturn::Return(0))
// }

pub fn sys_write(fd: usize, buffer: &[u8]) -> Result<SyscallReturn> {
    let ret = syscall(SYS_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()]);
    Ok(SyscallReturn::Return(ret))
}