// Panic handler.

use super::log::KERNEL_LOGGER;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Force unlock output in cases like panics during printing.
    unsafe { KERNEL_LOGGER.force_unlock() };

    print!("\x1b[0;31mpanic: Kernel panic - Environment is unsound!\x1b[0m\n");

    if let Some(message) = info.message().as_str() {
        print!("panic: \"{}\"\n", message);
    }

    if let Some(location) = info.location() {
        print!("panic: at {}\n", location);
    }

    // TODO: Send signal to all processors to stop execution.

    loop {}
}
