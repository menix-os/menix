#![no_std]
#![no_main]

use portal::user::{logging, thread};

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() {
    logging::log("Hello world from posix!\n");
    thread::exit();
}
