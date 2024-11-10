#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

extern "C" {
    fn foo(s: &str);
}

#[no_mangle]
unsafe fn _start() {
    foo("Hello, world!");
}
