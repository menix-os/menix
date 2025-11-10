use core::marker::PhantomData;

// TODO
pub struct RwLock<T> {
    _p: PhantomData<T>,
}
