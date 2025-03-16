// Panic handler.

use super::log::GLOBAL_LOGGERS;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    // Force unlock output in cases like panics during printing.
    unsafe { GLOBAL_LOGGERS.force_unlock() };

    print!(
        "\x1b[1;31m{}\x1b[0m\n", // Print in Red
        "panic: Kernel panic - Environment is unsound!"
    );

    print!("panic: {}\n", info.message());

    if let Some(location) = info.location() {
        print!("panic: at {}\n", location);
    }

    // TODO: Send signal to all processors to stop execution.

    loop {}
}
