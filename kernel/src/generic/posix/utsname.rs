use core::ffi::c_char;

/// Hacky function to convert from a string to a C char slice.
const fn copy_str_to_buf(buf: &mut [c_char], s: &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < buf.len() && i < s.len() {
        buf[i] = bytes[i] as c_char;
        i += 1;
    }
}

pub const SYSNAME: &str = "Menix";
pub const RELEASE: &str = env!("CARGO_PKG_VERSION");
pub const VERSION: &str = env!("MENIX_VERSION");

#[cfg(target_arch = "x86_64")]
pub const MACHINE: &str = "x86_64";

pub fn utsname(result: &mut uapi::utsname) {
    copy_str_to_buf(&mut result.sysname, SYSNAME);
    copy_str_to_buf(&mut result.release, RELEASE);
    copy_str_to_buf(&mut result.version, VERSION);
    copy_str_to_buf(&mut result.machine, MACHINE);

    // TODO: Get these from the actual hostname.
    copy_str_to_buf(&mut result.nodename, "localhost");
    copy_str_to_buf(&mut result.domainname, "(none)");
}
