pub trait Primitive: Copy + Default {
    type Bytes: Copy;

    fn swap_bytes(self) -> Self;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
    fn to_le_bytes(self) -> Self::Bytes;
    fn to_be_bytes(self) -> Self::Bytes;

    fn to_ne_bytes(self) -> Self::Bytes {
        #[cfg(target_endian = "little")]
        return self.to_le_bytes();
        #[cfg(target_endian = "big")]
        return self.to_be_bytes();
    }

    fn from_ne_bytes(bytes: Self::Bytes) -> Self {
        #[cfg(target_endian = "little")]
        return Self::from_le_bytes(bytes);
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(bytes);
    }
}

macro_rules! impl_primitive {
    ($($ty:ty),*) => {
        $(
            impl Primitive for $ty {
                type Bytes = [u8; core::mem::size_of::<$ty>()];

                fn swap_bytes(self) -> Self { <$ty>::swap_bytes(self) }
                fn from_le_bytes(bytes: Self::Bytes) -> Self { <$ty>::from_le_bytes(bytes) }
                fn from_be_bytes(bytes: Self::Bytes) -> Self { <$ty>::from_be_bytes(bytes) }
                fn to_le_bytes(self) -> Self::Bytes { <$ty>::to_le_bytes(self) }
                fn to_be_bytes(self) -> Self::Bytes { <$ty>::to_be_bytes(self) }
            }
        )*
    };
}

impl_primitive!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);
