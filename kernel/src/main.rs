#![no_std]
#![no_main]

mod arch;

#[no_mangle]
pub extern "C" fn kstart() -> ! {
    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
