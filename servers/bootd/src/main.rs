#![no_std]
#![no_main]

use portal::user::{logging, thread};

#[unsafe(no_mangle)]
fn _start() {
    logging::log("Hello world from bootd!\n");
    thread::exit();
}
