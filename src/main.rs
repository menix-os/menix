// Export of kernel functions for driver interface

#![no_std]
#![no_main]
// TODO: remove
#![allow(unused)]

pub mod arch;
pub mod boot;
pub mod memory;
pub mod syscall;
pub mod system;
pub mod thread;
pub mod util;
pub mod video;

extern crate alloc;
extern crate core;

pub fn kernel_main() -> ! {
    loop {}
}
