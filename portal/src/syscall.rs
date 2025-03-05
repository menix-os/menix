/// Syscall numbers to identify the type of call being made.
pub mod numbers {
    pub const NULL: usize = 0;
    pub const PRINT_LOG: usize = 1;
    pub const MEMORY_ALLOCATE: usize = 2;
    pub const MEMORY_FREE: usize = 3;
    pub const CHANNEL_CREATE: usize = 4;
    pub const CHANNEL_CLOSE: usize = 5;
    pub const CHANNEL_FIND: usize = 6;
    pub const CHANNEL_SUBMIT: usize = 7;
    pub const CHANNEL_RECEIVE: usize = 8;
    pub const OBJECT_CREATE: usize = 9;
    pub const OBJECT_DELETE: usize = 10;
    pub const OBJECT_READ: usize = 11;
    pub const OBJECT_WRITE: usize = 12;
    pub const THREAD_EXIT: usize = 13;
    pub const THREAD_CREATE: usize = 14;
    pub const THREAD_SET_TLS: usize = 15;
    pub const SUPERCALL: usize = 1 << (0usize.trailing_zeros() - 1);
}
