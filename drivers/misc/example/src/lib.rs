#![no_std]

use menix::{module, print};

module!(
    "Example module which prints \"Hello World\" to the log",
    "John Doe",
    main
);

pub fn main() {
    print!("Hello, world!\n");
}
