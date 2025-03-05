// Code that's used by everything but the kernel.

use core::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::panic::PanicInfo;

pub mod channel;
pub mod logging;
pub mod memory;
pub mod thread;

/// Invokes a system call.
fn do_syscall(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, usize) {
    let ret_val: usize;
    let ret_err: usize;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "syscall",
            in("rax") num,
            in("rdi") a0,
            in("rsi") a1,
            in("rdx") a2,
            in("r9") a3,
            in("r8") a4,
            in("r10") a5,
            out("rcx") _, // clobbered by syscall
            out("r11") _, // clobbered by syscall
            lateout("rax") ret_val,
            lateout("rdx") ret_err,
        );
    }

    return (ret_val, ret_err);
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    logging::log("PANIC!\n");
    thread::exit();
}

struct Allocator;
#[global_allocator]
static GLOBAL_ALLOC: Allocator = Allocator {};
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return memory::allocate(layout.size(), layout.align());
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        memory::free(ptr, layout.size(), layout.align());
    }
}
