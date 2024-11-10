// Panic handler.

use crate::{log, misc::log::STDOUT};
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Force unlock output in cases like panics during printing.
    unsafe { STDOUT.force_unlock() };

    log!("Kernel panicked!\n");

    if let Some(message) = info.message().as_str() {
        log!("[Message]\t{}\n", message);
    }

    if let Some(location) = info.location() {
        log!("[Location]\t{}\n", location);
    }

    // TODO: Send signal to all processors to stop execution.

    loop {}
}
