use super::posix::errno::{EResult, Errno};

pub struct Event {}

pub trait Resource: Sync + Send {
    fn add_event(&self);
    fn remove_event(&self, event: &Event);
}
