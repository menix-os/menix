//! Efficient implementations for the freestanding C string.h functions.

use core::mem;

const REGISTER_SIZE: usize = mem::size_of::<usize>();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    let mut i = 0;
    let chunks = len / REGISTER_SIZE;

    while i < chunks * REGISTER_SIZE {
        unsafe {
            dest.add(i)
                .cast::<usize>()
                .write_unaligned(src.add(i).cast::<usize>().read_unaligned())
        };
        i += REGISTER_SIZE;
    }

    while i < len {
        unsafe { dest.add(i).write(src.add(i).read()) };
        i += 1;
    }

    return dest;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, len: usize) -> *mut u8 {
    let chunks = len / REGISTER_SIZE;

    if src < dest as *const u8 {
        let mut i = len;

        while i != chunks * REGISTER_SIZE {
            i -= 1;
            unsafe { dest.add(i).write(src.add(i).read()) };
        }

        while i > 0 {
            i -= REGISTER_SIZE;

            unsafe {
                dest.add(i)
                    .cast::<usize>()
                    .write_unaligned(src.add(i).cast::<usize>().read_unaligned())
            };
        }
    } else {
        let mut i = 0_usize;

        while i < chunks * REGISTER_SIZE {
            unsafe {
                dest.add(i)
                    .cast::<usize>()
                    .write_unaligned(src.add(i).cast::<usize>().read_unaligned())
            };

            i += REGISTER_SIZE;
        }

        while i < len {
            unsafe { dest.add(i).write(src.add(i).read()) };
            i += 1;
        }
    }

    return dest;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut u8, byte: i32, len: usize) -> *mut u8 {
    let byte = byte as u8;

    let mut i = 0;

    let broadcasted = usize::from_ne_bytes([byte; REGISTER_SIZE]);
    let chunks = len / REGISTER_SIZE;

    while i < chunks * REGISTER_SIZE {
        unsafe { dest.add(i).cast::<usize>().write_unaligned(broadcasted) };
        i += REGISTER_SIZE;
    }

    while i < len {
        unsafe { dest.add(i).write(byte) };
        i += 1;
    }

    return dest;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    let mut i = 0_usize;

    // First compare WORD_SIZE chunks...
    let chunks = len / REGISTER_SIZE;

    while i < chunks * REGISTER_SIZE {
        let a = unsafe { s1.add(i).cast::<usize>().read_unaligned() };
        let b = unsafe { s2.add(i).cast::<usize>().read_unaligned() };

        if a != b {
            let diff = usize::from_be(a).wrapping_sub(usize::from_be(b)) as isize;

            return diff.signum() as i32;
        }
        i += REGISTER_SIZE;
    }

    // ... and then compare bytes.
    while i < len {
        let a = unsafe { s1.add(i).read() };
        let b = unsafe { s2.add(i).read() };

        if a != b {
            return i32::from(a) - i32::from(b);
        }
        i += 1;
    }

    return 0;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(str: *const u8) -> usize {
    let mut result = 0;
    let mut cur_str = str;

    while unsafe { *cur_str } != 0 {
        result += 1;
        cur_str = unsafe { cur_str.add(1) };
    }

    return result;
}
