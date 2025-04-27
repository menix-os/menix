use core::mem;

const REG_SIZE: usize = size_of::<usize>();

#[unsafe(no_mangle)]
unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, mut n: usize) -> *mut u8 {
    if n == 0 {
        return dest;
    }

    let mut d = dest as usize;
    let mut s = src as usize;

    // Align to register size.
    if d % REG_SIZE != 0 || s % REG_SIZE != 0 {
        while n != 0 && (d % REG_SIZE != 0) && (s % REG_SIZE != 0) {
            unsafe { *(d as *mut u8) = *(s as *const u8) };
            d += 1;
            s += 1;
            n -= 1;
        }
    }

    let qword_dest = d as *mut usize;
    let qword_src = s as *const usize;
    let qword_count = n / REG_SIZE;

    // Perform register-sized copy.
    for i in 0..qword_count {
        unsafe { *(qword_dest.add(i)) = *(qword_src.add(i)) };
    }

    let mut remaining_bytes = n % REG_SIZE;
    d = (unsafe { qword_dest.add(qword_count) } as usize);
    s = (unsafe { qword_src.add(qword_count) } as usize);

    // Copy the remaining bytes.
    while remaining_bytes > 0 {
        unsafe { (d as *mut u8).write((s as *const u8).read()) };
        d += 1;
        s += 1;
        remaining_bytes -= 1;
    }

    return dest;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn memmove(dstptr: *mut u8, srcptr: *const u8, size: usize) -> *mut u8 {
    let mut dst = dstptr;
    let mut src = srcptr;
    if (dst as usize) < (src as usize) {
        for i in 0..size {
            unsafe {
                *(dst.add(i)) = *(src.add(i));
            }
        }
    } else {
        for i in (1..=size).rev() {
            unsafe {
                *(dst.add(i - 1)) = *(src.add(i - 1));
            }
        }
    }
    return dstptr;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn memset(dest: *mut u8, value: u8, mut n: usize) -> *mut u8 {
    if n == 0 {
        return dest;
    }

    let mut d = dest as usize;

    // Align to register size.
    if d % REG_SIZE != 0 {
        while n != 0 && (d % REG_SIZE != 0) {
            unsafe { *(d as *mut u8) = value as u8 };
            d += 1;
            n -= 1;
        }
    }

    let qword_dest = d as *mut usize;
    let mut qword_value = value as usize;
    qword_value |= (qword_value << 8);
    qword_value |= (qword_value << 16);
    qword_value |= (qword_value << 32);

    let word_count = n / REG_SIZE;

    // Perform register-sized copy.
    for i in 0..word_count {
        unsafe { *(qword_dest.add(i)) = qword_value };
    }

    let mut remaining_bytes = n % REG_SIZE;
    d = (unsafe { qword_dest.add(word_count) } as usize);

    // Copy the remaining bytes.
    while remaining_bytes != 0 {
        unsafe { *(d as *mut u8) = value as u8 };
        d += 1;
        remaining_bytes -= 1;
    }

    return dest;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, len: usize) -> i32 {
    let mut i = 0_usize;

    let chunks = len / REG_SIZE;

    while i < chunks * REG_SIZE {
        let a = unsafe { s1.add(i).cast::<usize>().read_unaligned() };
        let b = unsafe { s2.add(i).cast::<usize>().read_unaligned() };

        if a != b {
            let diff = usize::from_be(a).wrapping_sub(usize::from_be(b)) as isize;

            return diff.signum() as i32;
        }
        i += REG_SIZE;
    }

    while i < len {
        let a = unsafe { s1.add(i).read() };
        let b = unsafe { s2.add(i).read() };

        if a != b {
            return i32::from(a) - i32::from(b);
        }
        i += 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe fn strlen(mut s: *const core::ffi::c_char) -> usize {
    let mut n = 0;
    while unsafe { *s } != 0 {
        n += 1;
        s = s.wrapping_add(1);
    }
    n
}
