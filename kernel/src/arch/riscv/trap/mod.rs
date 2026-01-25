#[expect(clippy::module_inception)]
mod trap;

use riscv::{
    interrupt::supervisor::{Exception, Interrupt},
    register::scause::Trap,
};

pub use trap::{TrapFrame, init_no_cpu};

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    use riscv::register::{scause, stval};
    let cause = scause::read();

    let Ok(cause) = Trap::<Interrupt, Exception>::try_from(cause.cause()) else {
        panic!(
            "Cannot handle unknown trap, scause: {:#x}, trapframe: {:#x?}.",
            cause.bits(),
            tf
        );
    };

    match cause {
        Trap::Exception(e) => {
            panic!("kernel error： {:?}, stval={:#x}", e, stval::read());
        }
        Trap::Interrupt(i) => {
            panic!("kernel Interrupt： {:?}",i);
        }
    }
}
