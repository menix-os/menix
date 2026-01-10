use super::{Socket, socket_poll, socket_read, socket_write};
use crate::{
    posix::errno::EResult,
    util::{mutex::Mutex, ring::RingBuffer},
    vfs::{File, file::FileOps},
};
use alloc::{sync::Arc, vec::Vec};

pub struct LocalSocket {
    buffer: Mutex<RingBuffer>,
}

impl LocalSocket {
    pub fn new() -> Self {
        Self {
            buffer: Mutex::new(RingBuffer::new(64 * 1024)),
        }
    }
}

impl Socket for LocalSocket {
    fn accept(&self, addr: &[u8]) -> EResult<()> {
        todo!()
    }

    fn bind(&self, addr: &[u8]) -> EResult<()> {
        Ok(())
    }

    fn peer_name(&self, addr: &mut [u8]) -> EResult<()> {
        addr.fill(0);
        Ok(())
    }

    fn sock_name(&self, addr: &mut [u8]) -> EResult<()> {
        addr.fill(0);
        Ok(())
    }

    fn shutdown(&self, how: u32) -> EResult<()> {
        todo!()
    }

    fn connect(&self) -> EResult<()> {
        todo!()
    }

    fn send_msg(&self) -> EResult<()> {
        todo!()
    }

    fn receive_msg(&self) -> EResult<()> {
        todo!()
    }

    fn poll(&self) -> EResult<()> {
        todo!()
    }

    fn listen(&self, backlog_size: i32) -> EResult<()> {
        // TODO
        Ok(())
    }
}

impl FileOps for LocalSocket {
    fn read(&self, file: &File, buffer: &mut [u8], offset: u64) -> EResult<isize> {
        socket_read(self, file, buffer, offset)
    }

    fn write(&self, file: &File, buffer: &[u8], offset: u64) -> EResult<isize> {
        socket_write(self, file, buffer, offset)
    }

    fn poll(&self, file: &File, mask: i16) -> EResult<i16> {
        socket_poll(self, file, mask)
    }
}
