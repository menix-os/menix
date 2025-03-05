#[repr(usize)]
pub enum Error {
    /// One or more parameters are not valid.
    InvalidArgument,
    /// One or more parameters are valid, but their semantic values are not.
    InvalidContent,
    /// Unable to find an object.
    NotFound,
    /// Object already exists.
    AlreadyExists,
    /// Object is busy and cannot be used.
    Busy,
    /// Method is not allowed to be called.
    NotPermitted,
    /// No more memory available to make an allocation.
    OutOfMemory,
    /// Method is not implemented.
    NotImplemented,
    /// Method was aborted.
    Aborted,
}
