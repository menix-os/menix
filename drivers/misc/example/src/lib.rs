#![no_std]

use menix::{generic::cmdline::CmdLine, module, print};

module!(
    "Example module which prints \"Hello World\" to the log",
    "John Doe",
    main
);

pub fn main(_args: CmdLine) {
    print!("Hello, world!\n");
}
