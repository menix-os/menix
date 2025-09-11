//! Commonly needed data structures.

pub mod event;
pub mod mutex;
pub mod once;
pub mod ring;
pub mod rwlock;
pub mod spin;

use num_traits::PrimInt;

/// Aligns a value to the next higher multiple of `alignment`.
#[inline]
pub fn align_up<T: PrimInt>(value: T, alignment: T) -> T {
    let mask = alignment - T::one();
    (value + mask) & !mask
}

/// Aligns a value to the next lower multiple of `alignment`.
#[inline]
pub fn align_down<T: PrimInt>(value: T, alignment: T) -> T {
    let mask = alignment - T::one();
    (value) & !mask
}

/// Divides a value after rounding up to the next higher multiple of `alignment`.
#[inline]
pub fn divide_up<T: PrimInt>(value: T, alignment: T) -> T {
    align_up(value, alignment) / alignment
}
