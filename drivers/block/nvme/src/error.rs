use core::fmt::Display;

#[derive(Debug)]
pub enum NvmeError {
    UnsupportedPageSize,
    RegisterOutOfBounds,
    SubmitCommand { command_id: usize, queue_id: usize },
    MissingQueue,
    AllocationFailed,
}

impl Display for NvmeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NvmeError::UnsupportedPageSize => f.write_str("The host's page size is not supported"),
            NvmeError::RegisterOutOfBounds => {
                f.write_str("Attempted to write to register which is out of bounds")
            }
            NvmeError::SubmitCommand {
                command_id,
                queue_id,
            } => f.write_fmt(format_args!(
                "Failed to submit command {command_id} to queue {queue_id}"
            )),
            NvmeError::MissingQueue => f.write_str("Attempted to write to a missing queue"),
            NvmeError::AllocationFailed => f.write_str("Failed to allocate enough memory"),
        }
    }
}
