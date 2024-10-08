// Panic handler.

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
