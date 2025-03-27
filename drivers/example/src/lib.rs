#![no_std]

pub use menix;
use menix::{module, print};

module!(b"example", b"Example description", b"John Doe");

pub extern "C" fn _start() {
    print!("Hello from the example!\n");
}
