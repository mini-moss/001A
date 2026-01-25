use core::arch::{global_asm, asm};

global_asm!(include_str!("trap.S"));

pub unsafe fn init_no_cpu() {
    unsafe {
        asm!("csrw sscratch, zero");
        asm!("csrw stvec, {}", in(reg) trap_entry as *const () as usize);
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    /// General registers
    pub general: [usize; 32], // 通用寄存器 x0..x31
    /// Supervisor Status
    pub sstatus: usize, // CPU 状态寄存器
    /// Supervisor Exception Program Counter
    pub sepc: usize, // 异常发生时的 PC
}

unsafe extern "C" {
    unsafe fn trap_entry();
}