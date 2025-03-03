#![no_std]
#![no_main]

#[unsafe(no_mangle)]
fn _start() {
    portal::logging::log("Hello world from bootd!\n");
}
