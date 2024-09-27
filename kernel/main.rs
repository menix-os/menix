#![no_std]
#![no_main]

extern crate alloc;
mod arch;
mod boot;

pub fn kernel_main() -> ! {
    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
