use core::fmt;
use core::fmt::{Result, Write};
use core::result::Result::Ok;
use crate::arch::sbi::console_putchar;

fn uart_write_byte(byte: u8) {
    unsafe {
        const UART0: *mut u8 = 0x1000_0000 as *mut u8;
        core::ptr::write_volatile(UART0, byte);
    }
}

pub struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            uart_write_byte(byte);
        }
        Ok(())
    }
}

// pub fn _print(args: fmt::Arguments) {
//     let _ = UartWriter.write_fmt(args);
// }
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.bytes() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        $crate::_print(format_args!("{}\n", $($arg)*));
    }};
}
