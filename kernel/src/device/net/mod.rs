use crate::{
    posix::errno::{EResult, Errno},
    uapi::socket::{AF_INET, AF_LOCAL},
    vfs::{File, file::FileOps},
};
use alloc::sync::Arc;

pub mod local;

pub trait Socket: FileOps {
    fn accept(&self, addr: &[u8]) -> EResult<()>;
    fn bind(&self, addr: &[u8]) -> EResult<()>;
    fn connect(&self) -> EResult<()>;
    fn peer_name(&self, addr: &mut [u8]) -> EResult<()>;
    fn sock_name(&self, addr: &mut [u8]) -> EResult<()>;
    fn send_msg(&self) -> EResult<()>;
    fn receive_msg(&self) -> EResult<()>;
    fn listen(&self, backlog_size: i32) -> EResult<()>;
    fn poll(&self) -> EResult<()>;
    fn shutdown(&self, how: u32) -> EResult<()>;
    fn set_opt(&self) -> EResult<()> {
        Err(Errno::ENOPROTOOPT)
    }
}

fn socket_read<T: Socket>(
    socket: &T,
    file: &File,
    buffer: &mut [u8],
    offset: u64,
) -> EResult<isize> {
    let _ = (socket, offset, buffer, file);
    todo!()
}

fn socket_write<T: Socket>(socket: &T, file: &File, buffer: &[u8], offset: u64) -> EResult<isize> {
    let _ = (socket, offset, buffer, file);
    todo!()
}

fn socket_poll<T: Socket>(socket: &T, file: &File, mask: i16) -> EResult<i16> {
    _ = (socket, file, mask);
    todo!()
}

pub fn create_socket(family: u32, typ: u32) -> Option<Arc<dyn Socket>> {
    match family {
        AF_INET => match typ {
            _ => todo!("Implement AF_INET"),
        },
        AF_LOCAL => Some(Arc::new(local::LocalSocket::new())),
        _ => None,
    }
}
