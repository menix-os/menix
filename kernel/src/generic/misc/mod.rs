/// Aligns a value to the next higher multiple of `alignment`.
#[inline]
pub const fn align_up(value: usize, alignment: usize) -> usize {
    let mask = alignment - 1;
    return (value + mask) & !mask;
}

/// Aligns a value to the next lower multiple of `alignment`.
#[inline]
pub const fn align_down(value: usize, alignment: usize) -> usize {
    let mask = alignment - 1;
    return (value) & !mask;
}
