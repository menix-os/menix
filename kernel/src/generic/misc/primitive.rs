pub trait Primitive: Copy + Default {
    fn swap_bytes(self) -> Self;
    fn from_ne_bytes(buf: &[u8]) -> Self {
        #[cfg(target_endian = "little")]
        return Self::from_le_bytes(buf);
        #[cfg(target_endian = "big")]
        return Self::from_be_bytes(buf);
    }
    fn from_le_bytes(buf: &[u8]) -> Self;
    fn from_be_bytes(buf: &[u8]) -> Self;
    fn to_ne_bytes(self, buf: &mut [u8]) {
        #[cfg(target_endian = "little")]
        self.to_le_bytes(buf);
        #[cfg(target_endian = "big")]
        self.to_be_bytes(buf);
    }
    fn to_le_bytes(self, buf: &mut [u8]);
    fn to_be_bytes(self, buf: &mut [u8]);
}

macro_rules! impl_primitive {
    ($($ty:ty),*) => {
        $(impl Primitive for $ty {fn swap_bytes(self) -> Self {<$ty>::swap_bytes(self)}
        fn from_le_bytes(buf: &[u8]) -> Self {<$ty>::from_le_bytes(buf.try_into().unwrap())}
        fn from_be_bytes(buf: &[u8]) -> Self {<$ty>::from_be_bytes(buf.try_into().unwrap())}
        fn to_le_bytes(self, buf: &mut [u8]) {self.to_le_bytes(); }
        fn to_be_bytes(self, buf: &mut [u8]) {self.to_be_bytes(); }
})*
    };
}

impl_primitive!(
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
);
