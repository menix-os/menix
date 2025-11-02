use crate::generic::{
    memory::{VirtAddr, user::UserPtr},
    posix::errno::{EResult, Errno},
    util::{event::Event, mutex::spin::SpinMutex, ring::RingBuffer},
    vfs::{
        File,
        file::{FileOps, OpenFlags},
    },
};
use core::hint::unlikely;

#[derive(Debug)]
pub struct PipeBuffer {
    // Using a spin mutex here is fine because the tasks are preempted by the events.
    inner: SpinMutex<PipeInner>,
    rd_queue: Event,
    wr_queue: Event,
}

#[derive(Debug)]
struct PipeInner {
    buffer: RingBuffer,
    readers: usize,
    writers: usize,
}

impl PipeBuffer {
    pub fn new() -> Self {
        Self {
            inner: SpinMutex::new(PipeInner {
                buffer: RingBuffer::new(0x1000),
                readers: 0,
                writers: 0,
            }),
            rd_queue: Event::new(),
            wr_queue: Event::new(),
        }
    }

    /// Returns the capacity of the pipe in bytes.
    pub fn capacity(&self) -> usize {
        self.inner.lock().buffer.capacity()
    }
}

impl FileOps for PipeBuffer {
    fn acquire(&self, file: &File) -> EResult<()> {
        let mut inner = self.inner.lock();
        let flags = *file.flags.lock();

        if flags.contains(OpenFlags::Read) {
            inner.readers += 1;
        }
        if flags.contains(OpenFlags::Write) {
            inner.writers += 1;
        }

        Ok(())
    }

    fn release(&self, file: &File) -> EResult<()> {
        let mut inner = self.inner.lock();
        let flags = *file.flags.lock();

        if flags.contains(OpenFlags::Read) {
            inner.readers -= 1;
        }
        if flags.contains(OpenFlags::Write) {
            inner.writers -= 1;
        }

        Ok(())
    }

    fn read(&self, file: &File, buf: &mut [u8], _off: u64) -> EResult<isize> {
        if unlikely(buf.is_empty()) {
            return Ok(0);
        }

        let read = self.rd_queue.guard();
        loop {
            let mut inner = self.inner.lock();
            let len = inner.buffer.read(buf);

            // If there was at least one byte written to the pipe
            if len > 0 {
                self.wr_queue.wake_one();
                return Ok(len as _);
            }

            if inner.writers == 0 {
                return Ok(0);
            }

            if file.flags.lock().contains(OpenFlags::NonBlocking) {
                return Err(Errno::EAGAIN);
            } else {
                drop(inner);
                read.wait();
            }
        }
    }

    fn write(&self, file: &File, buf: &[u8], _off: u64) -> EResult<isize> {
        if unlikely(buf.is_empty()) {
            return Ok(0);
        }

        let write = self.wr_queue.guard();
        loop {
            let len = {
                let mut inner = self.inner.lock();

                if inner.readers == 0 {
                    // TODO: Kill
                    return Err(Errno::EPIPE);
                }

                inner.buffer.write(buf)
            };
            if len > 0 {
                self.rd_queue.wake_one();
                return Ok(len as _);
            }

            if file.flags.lock().contains(OpenFlags::NonBlocking) {
                return Err(Errno::EAGAIN);
            } else {
                write.wait();
            }
        }
    }

    fn poll(&self, _file: &File, _mask: u16) -> EResult<u16> {
        todo!()
    }

    fn ioctl(&self, _file: &File, request: usize, argp: usize) -> EResult<usize> {
        match request as _ {
            uapi::FIONREAD => {
                let len = self.inner.lock().buffer.get_data_len() as i32;
                let count_ptr = UserPtr::new(VirtAddr::new(argp));
                count_ptr.write(len);
            }
            _ => return Err(Errno::ENOTTY),
        }
        Ok(0)
    }
}
