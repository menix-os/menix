// This code is only available to user mode applications.

use core::{arch::asm, panic::PanicInfo};

/// Invokes a system call.
pub fn do_syscall(
    num: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    let ret: usize;

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
            lateout("rax") ret,
        );
    }

    return ret;
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
