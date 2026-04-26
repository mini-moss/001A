#![no_std]
#![no_main]

use core::panic::PanicInfo;
use user_lib::sys_write;
use user_lib::sys_exit;

#[no_mangle]
fn main() -> i32 {
    let msg = b"Hello World\n";
    unsafe {
        sys_write(1, msg.as_ptr(), msg.len());
    }
    sys_exit(0) // 返回 0 表示测试通过
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { sys_exit(1) } // 出现 panic 测试失败
}
