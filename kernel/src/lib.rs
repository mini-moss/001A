#![no_std]
#![no_main]

use logger::print;

mod executor;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    print!("Hello, world!");
    print!("Number: {}", 42);
    loop {
        
    }
}