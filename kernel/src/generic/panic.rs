// Panic handler.

use super::log::GLOBAL_LOGGERS;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Force unlock output in cases like panics during printing.
    unsafe { GLOBAL_LOGGERS.force_unlock(true) };

    error!("Kernel panic - Environment is unsound!");

    if let Some(location) = info.location() {
        error!("at {}", location);
    }

    error!("{}", info.message());

    loop {}
}
