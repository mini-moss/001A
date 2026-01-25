#![no_std]

#[cfg_attr(target_arch = "riscv64", path = "arch/riscv/mod.rs")]
pub mod arch;
mod lang;
mod logger;

pub use logger::_print;
