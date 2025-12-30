pub const POLLIN: i16 = 0x0001;
pub const POLLPRI: i16 = 0x0002;
pub const POLLOUT: i16 = 0x0004;
pub const POLLERR: i16 = 0x0008;
pub const POLLHUP: i16 = 0x0010;
pub const POLLNVAL: i16 = 0x0020;
pub const POLLRDNORM: i16 = 0x0040;
pub const POLLRDBAND: i16 = 0x0080;
pub const POLLWRNORM: i16 = 0x0100;
pub const POLLWRBAND: i16 = 0x0200;
pub const POLLRDHUP: i16 = 0x2000;

pub type nfds_t = isize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct pollfd {
    fd: i32,
    events: i16,
    revents: i16,
}
