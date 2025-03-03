/// Aligns a value to the next higher multiple of `alignment`.
pub const fn align_up(value: usize, alignment: usize) -> usize {
    return ((value + (alignment - 1)) / alignment) * alignment;
}

/// Aligns a value to the next lower multiple of `alignment`.
pub const fn align_down(value: usize, alignment: usize) -> usize {
    return (value / alignment) * alignment;
}
