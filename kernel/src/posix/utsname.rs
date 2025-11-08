use crate::util::mutex::spin::SpinMutex;
use core::ffi::c_char;

/// Hacky function to convert from a string to a C char slice.
const fn to_char_array<const N: usize>(x: &str) -> [c_char; N] {
    let mut result = [0; N];
    let mut i = 0;
    let bytes = x.as_bytes();
    while i < N && i < x.len() {
        result[i] = bytes[i] as c_char;
        i += 1;
    }
    result
}

pub static UTSNAME: SpinMutex<uapi::utsname> = SpinMutex::new(uapi::utsname {
    sysname: to_char_array("Menix"),
    nodename: to_char_array("localhost"),
    release: to_char_array(env!("CARGO_PKG_VERSION")),
    version: to_char_array(env!("MENIX_VERSION")),
    machine: {
        #[cfg(target_arch = "x86_64")]
        {
            to_char_array("x86_64")
        }

        #[cfg(target_arch = "riscv64")]
        {
            to_char_array("riscv64")
        }
    },
    domainname: to_char_array("(none)"),
});
