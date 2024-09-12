#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

const fn fixed_str<const N: usize, const M: usize>(src: [u8; M]) -> [u8; N] {
    let mut dst = [0; N];
    let mut i = 0;
    while i < M {
        dst[i] = src[i];
        i += 1;
    }
    dst
}

#[repr(C, packed)]
pub struct Module {
    pub name: [u8; 64],
    pub author: [u8; 64],
    pub description: [u8; 128],
    pub license: [u8; 48],
    pub init: extern "C" fn() -> i32,
    pub exit: extern "C" fn(),
    pub dependencies: usize,
    pub num_dependencies: usize,
}

// Calls to kernel procedures.
extern "C" {
    fn kmesg(fmt: *const u8);
}

#[no_mangle]
#[link_section = ".mod"]
pub static THIS_MODULE: Module = Module {
    name: fixed_str(*b"rust_module"),
    author: fixed_str(*b"John Doe"),
    description: fixed_str(*b"This is a Rust example demonstrating the menix FFI"),
    license: fixed_str(*b"LGPL"),
    init: init_fn,
    exit: exit_fn,
    dependencies: 0,
    num_dependencies: 0,
};

pub extern "C" fn init_fn() -> i32 {
    unsafe {
        kmesg(b"Hello from Rust!\n".as_ptr());
    }
    return 0;
}

pub extern "C" fn exit_fn() {}
