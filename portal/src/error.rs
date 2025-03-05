#[repr(usize)]
pub enum Error {
    BadArgument,
    NotFound,
    AlreadyExists,
}
