use super::{Socket, socket_poll, socket_read, socket_write};
use crate::{
    posix::errno::{EResult, Errno},
    util::{mutex::Mutex, mutex::spin::SpinMutex, ring::RingBuffer, event::Event},
    vfs::{File, file::FileOps},
    percpu::CpuData,
    uapi::poll::{POLLIN, POLLOUT, POLLHUP},
};
use alloc::{sync::{Arc, Weak}, vec::Vec, collections::{VecDeque, BTreeMap}};

static BOUND_SOCKETS: SpinMutex<BTreeMap<Vec<u8>, Weak<LocalSocket>>> = SpinMutex::new(BTreeMap::new());

enum SocketState {
    Unbound,
    Bound(Vec<u8>),
    Listening(VecDeque<Arc<LocalSocket>>),
    Connected(Weak<LocalSocket>),
    Closed,
}

pub struct LocalSocket {
    state: Mutex<SocketState>,
    buffer: Mutex<RingBuffer>,
    read_wait: Event,
    myself: Mutex<Weak<LocalSocket>>,
}

impl LocalSocket {
    pub fn new() -> Arc<Self> {
        let socket = Arc::new(Self {
            state: Mutex::new(SocketState::Unbound),
            buffer: Mutex::new(RingBuffer::new(64 * 1024)),
            read_wait: Event::new(),
            myself: Mutex::new(Weak::new()),
        });
        *socket.myself.lock() = Arc::downgrade(&socket);
        socket
    }
}

impl Socket for LocalSocket {
    fn accept(&self, addr: &mut [u8]) -> EResult<Arc<dyn Socket>> {
        let mut state = self.state.lock();
        loop {
            match &mut *state {
                SocketState::Listening(queue) => {
                    if let Some(sock) = queue.pop_front() {
                        return Ok(sock);
                    }
                }
                _ => return Err(Errno::EINVAL),
            }
            drop(state);
            self.read_wait.guard().wait();
            state = self.state.lock();
        }
    }

    fn bind(&self, addr: &[u8]) -> EResult<()> {
        let mut state = self.state.lock();
        if let SocketState::Unbound = *state {
            let path = addr.to_vec();
            let mut registry = BOUND_SOCKETS.lock();
            if registry.contains_key(&path) {
                 if let Some(weak) = registry.get(&path) {
                     if weak.upgrade().is_some() {
                         return Err(Errno::EADDRINUSE);
                     }
                 }
            }

            let myself = self.myself.lock().clone();
            registry.insert(path.clone(), myself);
            *state = SocketState::Bound(path);
            Ok(())
        } else {
            Err(Errno::EINVAL)
        }
    }

    fn listen(&self, backlog: i32) -> EResult<()> {
        let mut state = self.state.lock();
        if let SocketState::Bound(_) = *state {
             *state = SocketState::Listening(VecDeque::new());
             Ok(())
        } else if let SocketState::Listening(_) = *state {
            Ok(())
        } else {
            Err(Errno::EINVAL)
        }
    }

    fn connect(&self, addr: &[u8]) -> EResult<()> {
        let path = addr.to_vec();
        // Lookup peer
        let peer = {
            let registry = BOUND_SOCKETS.lock();
            registry.get(&path).and_then(|weak| weak.upgrade()).ok_or(Errno::ENOENT)?
        };

        let mut peer_state = peer.state.lock();
        if let SocketState::Listening(queue) = &mut *peer_state {
             let server_side = LocalSocket::new();

             {
                 let mut s_state = server_side.state.lock();
                 *s_state = SocketState::Connected(self.myself.lock().clone());
             }

             let mut my_state = self.state.lock();
             match *my_state {
                 SocketState::Unbound | SocketState::Bound(_) => {
                     *my_state = SocketState::Connected(Arc::downgrade(&server_side));
                 },
                 _ => return Err(Errno::EISCONN)
             }

             queue.push_back(server_side);
             peer.read_wait.wake_one();
             Ok(())

        } else {
            Err(Errno::ECONNREFUSED)
        }
    }

    fn peer_name(&self, addr: &mut [u8]) -> EResult<()> {
        Ok(())
    }

    fn sock_name(&self, addr: &mut [u8]) -> EResult<()> {
        let state = self.state.lock();
        if let SocketState::Bound(path) = &*state {
             let len = core::cmp::min(path.len(), addr.len());
             addr[..len].copy_from_slice(&path[..len]);
        }
        Ok(())
    }

    fn send_msg(&self, buffer: &[u8], flags: i32) -> EResult<isize> {
        let state = self.state.lock();
        let peer = match &*state {
            SocketState::Connected(weak) => weak.upgrade(),
             _ => return Err(Errno::ENOTCONN),
        };
        drop(state);

        if let Some(peer) = peer {
             let mut peer_buf = peer.buffer.lock();
             let written = peer_buf.write(buffer);
             if written > 0 {
                 peer.read_wait.wake_one();
             } else if buffer.len() > 0 && peer_buf.is_full() {
                  return Err(Errno::EAGAIN);
             }
             Ok(written as isize)
        } else {
            Err(Errno::EPIPE)
        }
    }

    fn receive_msg(&self, buffer: &mut [u8], flags: i32) -> EResult<isize> {
        let mut buf = self.buffer.lock();
        loop {
            if buf.get_data_len() > 0 {
                let n = buf.read(buffer);
                return Ok(n as isize);
            }

            let state = self.state.lock();
            match &*state {
                 SocketState::Connected(peer) => {
                     if peer.upgrade().is_none() {
                         return Ok(0);
                     }
                 },
                 SocketState::Closed => return Ok(0),
                 _ => return Err(Errno::ENOTCONN),
            }
            drop(state);
            drop(buf);

            self.read_wait.guard().wait();
            buf = self.buffer.lock();
        }
    }

    fn sock_poll(&self, mask: i16) -> EResult<i16> {
         let mut events = 0;
         let state = self.state.lock();

         if mask & POLLIN != 0 {
             match &*state {
                 SocketState::Listening(q) => {
                     if !q.is_empty() { events |= POLLIN; }
                 },
                 SocketState::Connected(peer) => {
                     let buf = self.buffer.lock();
                     if buf.get_data_len() > 0 {
                         events |= POLLIN;
                     } else if peer.upgrade().is_none() {
                         events |= POLLIN | POLLHUP;
                     }
                 },
                 _ => {}
             }
         }

         if mask & POLLOUT != 0 {
              match &*state {
                  SocketState::Connected(peer_weak) => {
                      if let Some(peer) = peer_weak.upgrade() {
                           let buf = peer.buffer.lock();
                           if !buf.is_full() {
                               events |= POLLOUT;
                           }
                      } else {
                          events |= POLLHUP;
                      }
                  },
                  _ => { events |= POLLOUT; }
              }
         }

         Ok(events)
    }

    fn shutdown(&self, how: u32) -> EResult<()> {
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
