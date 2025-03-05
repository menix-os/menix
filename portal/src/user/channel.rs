use super::do_syscall;
use crate::{
    error::Error,
    syscall::numbers::{CHANNEL_CREATE, CHANNEL_FIND},
};

pub type ChannelHandle = usize;

/// Creates a new named channel.
/// Fails when an empty name is given or a channel with that name already exists.
pub fn create(name: &str) -> Result<ChannelHandle, Error> {
    let (val, err) = do_syscall(
        CHANNEL_CREATE,
        name.as_ptr() as usize,
        name.len(),
        0,
        0,
        0,
        0,
    );

    // TODO: Read err instead of returning only BadArgument.
    if err != 0 {
        return Err(Error::BadArgument);
    }

    return Ok(val as ChannelHandle);
}

/// Attempts to find an open channel by name.
pub fn find(name: &str) -> Option<ChannelHandle> {
    let (val, err) = do_syscall(CHANNEL_FIND, name.as_ptr() as usize, name.len(), 0, 0, 0, 0);

    if err != 0 {
        return None;
    }

    return Some(val as ChannelHandle);
}
