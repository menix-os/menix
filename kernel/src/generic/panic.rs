// Panic handler.

use super::log::GLOBAL_LOGGERS;
use crate::generic::percpu;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Force unlock output in cases like panics during printing.
    unsafe { GLOBAL_LOGGERS.force_unlock() };

    error!("Kernel panic - Environment is unsound!\n");
    error!("panic: {}\n", info.message());

    if let Some(location) = info.location() {
        error!("panic: at {}\n", location);
    }

    // TODO: Send signal to all processors to stop execution.

    percpu::stop_all();
}
