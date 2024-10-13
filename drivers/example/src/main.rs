#![no_std]
#![no_main]

use kernel::{arch::PhysManager, log, memory::pm::CommonPhysManager};

#[no_mangle]
pub extern "C" fn _start() {
    let addr = PhysManager::alloc_zeroed(1);
    PhysManager::free(addr, 1);
    log!("Hello world!\n");
}
