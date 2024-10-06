// Export of kernel functions for driver interface

#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod arch;
pub mod boot;
pub mod memory;
pub mod misc;
pub mod syscall;
pub mod system;
pub mod thread;
pub mod video;

extern crate alloc;
extern crate core;

pub fn kernel_main() -> ! {
    todo!();
}
