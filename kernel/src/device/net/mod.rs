use crate::{
    posix::errno::{EResult, Errno},
    uapi::socket::{AF_INET, AF_LOCAL},
    vfs::{File, file::FileOps},
};
use alloc::sync::Arc;

pub mod local;

pub trait Socket: FileOps {
    fn accept(&self, addr: &mut [u8]) -> EResult<Arc<dyn Socket>>;
    fn bind(&self, addr: &[u8]) -> EResult<()>;
    fn connect(&self, addr: &[u8]) -> EResult<()>;
    fn peer_name(&self, addr: &mut [u8]) -> EResult<()>;
    fn sock_name(&self, addr: &mut [u8]) -> EResult<()>;
    fn send_msg(&self, buffer: &[u8], flags: i32) -> EResult<isize>;
    fn receive_msg(&self, buffer: &mut [u8], flags: i32) -> EResult<isize>;
    fn listen(&self, backlog_size: i32) -> EResult<()>;
    fn sock_poll(&self, mask: i16) -> EResult<i16>;
    fn shutdown(&self, how: u32) -> EResult<()>;
    fn set_opt(&self) -> EResult<()> {
        Err(Errno::ENOPROTOOPT)
    }
}

fn socket_read<T: Socket>(
    socket: &T,
    _file: &File,
    buffer: &mut [u8],
    _offset: u64,
) -> EResult<isize> {
    socket.receive_msg(buffer, 0)
}

fn socket_write<T: Socket>(
    socket: &T,
    _file: &File,
    buffer: &[u8],
    _offset: u64,
) -> EResult<isize> {
    socket.send_msg(buffer, 0)
}

fn socket_poll<T: Socket>(socket: &T, _file: &File, mask: i16) -> EResult<i16> {
    socket.sock_poll(mask)
}

pub fn create_socket(family: u32, typ: u32) -> Option<Arc<dyn Socket>> {
    match family {
        AF_INET => match typ {
            _ => todo!("Implement AF_INET"),
        },
        AF_LOCAL => Some(local::LocalSocket::new()),
        _ => None,
    }
}
