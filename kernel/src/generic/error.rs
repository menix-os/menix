#[derive(Debug)]
pub struct Error<'a> {
    /// Reason for this error.
    reason: &'a str,
}
