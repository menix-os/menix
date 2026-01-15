use crate::{
    device::net::{Socket, create_socket},
    memory::{UserPtr, VirtAddr},
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    uapi::{
        socket::{msghdr, socklen_t},
        uio::iovec,
    },
    util::mutex::Mutex,
    vfs::{
        File,
        file::{FileDescription, OpenFlags},
        inode::{INode, NodeOps},
    },
};
use alloc::{sync::Arc, vec::Vec};
use core::{mem::size_of, sync::atomic::AtomicBool};

fn alloc_fd(socket: Arc<dyn Socket>) -> EResult<i32> {
    let node = INode::new(NodeOps::Socket(socket.clone()), None);

    let proc = Scheduler::get_current().get_process();
    let mut files = proc.open_files.lock();
    files
        .open_file(
            FileDescription {
                file: Arc::new(File {
                    path: None,
                    ops: socket,
                    inode: Some(Arc::new(node)),
                    flags: Mutex::new(OpenFlags::empty()),
                    offset: Mutex::new(0),
                }),
                close_on_exec: AtomicBool::new(false),
            },
            0,
        )
        .ok_or(Errno::EMFILE)
}

pub fn socket(family: i32, typ: i32, protocol: i32) -> EResult<i32> {
    let socket = create_socket(family as _, typ as _).ok_or(Errno::EAFNOSUPPORT)?;

    if family != 1 {
        warn!(
            "socket({:#x}, {:#x}, {:#x}) is a stub!",
            family, typ, protocol
        );
    }

    alloc_fd(socket)
}

pub fn socketpair(family: i32, typ: i32, protocol: i32) -> EResult<u64> {
    let socket1 = create_socket(family as _, typ as _).ok_or(Errno::EAFNOSUPPORT)?;
    let socket2 = create_socket(family as _, typ as _).ok_or(Errno::EAFNOSUPPORT)?;

    warn!(
        "socketpair({:#x}, {:#x}, {:#x}, ...) is a stub!",
        family, typ, protocol
    );

    Err(Errno::ENOSYS)
}

fn fd_to_socket(fd: i32) -> EResult<Arc<dyn Socket>> {
    let proc = Scheduler::get_current().get_process();
    let fds = proc.open_files.lock();
    let desc = fds.get_fd(fd).ok_or(Errno::EBADF)?;
    match &desc.file.inode.as_ref().ok_or(Errno::ENOTSOCK)?.node_ops {
        NodeOps::Socket(x) => Ok(x.clone()),
        _ => Err(Errno::ENOTSOCK),
    }
}

pub fn shutdown(fd: i32, how: i32) -> EResult<()> {
    let socket = fd_to_socket(fd)?;
    socket.shutdown(how as u32)
}

pub fn bind(fd: i32, addr_ptr: VirtAddr, addr_length: socklen_t) -> EResult<()> {
    let socket = fd_to_socket(fd)?;
    let mut addr = vec![0u8; addr_length as _];
    let buffer = UserPtr::new(addr_ptr);
    buffer.read_slice(&mut addr).ok_or(Errno::EFAULT)?;
    socket.bind(&addr)
}

pub fn connect(fd: i32, addr_ptr: VirtAddr, addr_length: socklen_t) -> EResult<()> {
    let socket = fd_to_socket(fd)?;
    let mut addr = vec![0u8; addr_length as _];
    let buffer = UserPtr::new(addr_ptr);
    buffer.read_slice(&mut addr).ok_or(Errno::EFAULT)?;
    socket.connect(&addr)
}

pub fn accept(fd: i32, addr_ptr: VirtAddr, addr_len_ptr: VirtAddr) -> EResult<i32> {
    let socket = fd_to_socket(fd)?;

    let addr_len_ptr = UserPtr::<socklen_t>::new(addr_len_ptr);
    let mut addr_buf = if !addr_ptr.is_null() && !addr_len_ptr.is_null() {
        let len = addr_len_ptr.read().ok_or(Errno::EFAULT)?;
        vec![0u8; len as usize]
    } else {
        Vec::new()
    };

    let new_sock = socket.accept(&mut addr_buf)?;

    if !addr_ptr.is_null() && !addr_len_ptr.is_null() {
        let len = addr_buf.len() as socklen_t;
        let len_bytes = len.to_ne_bytes();

        //let mut len_slice = UserPtr::new(addr_len_ptr);
        //if let Some(user_len) = len_slice.as_mut_slice() {
        //    user_len.copy_from_slice(&len_bytes);
        //}

        //let mut addr_user_slice = UserSlice::new(addr_ptr, len as usize);
        //if let Some(user_addr) = addr_user_slice.as_mut_slice() {
        //    user_addr.copy_from_slice(&addr_buf);
        //}
    }

    alloc_fd(new_sock)
}

pub fn listen(fd: i32, backlog: i32) -> EResult<()> {
    let socket = fd_to_socket(fd)?;
    socket.listen(backlog)
}

pub fn getpeername(fd: i32, addr: VirtAddr, addr_len: VirtAddr) -> EResult<()> {
    if addr.is_null() || addr_len.is_null() {
        return Err(Errno::EFAULT);
    }
    let socket = fd_to_socket(fd)?;

    let addr_len = UserPtr::<socklen_t>::new(addr_len);
    let mut buf = vec![0u8; addr_len.read().ok_or(Errno::EFAULT)? as usize];
    socket.peer_name(&mut buf)?;

    UserPtr::new(addr).write_slice(&buf).ok_or(Errno::EFAULT)
}

pub fn getsockname(fd: i32, addr: VirtAddr, addr_len: VirtAddr) -> EResult<()> {
    if addr.is_null() || addr_len.is_null() {
        return Err(Errno::EFAULT);
    }
    let socket = fd_to_socket(fd)?;

    let addr_len = UserPtr::<socklen_t>::new(addr_len);
    let mut buf = vec![0u8; addr_len.read().ok_or(Errno::EFAULT)? as usize];
    socket.sock_name(&mut buf)?;

    UserPtr::new(addr).write_slice(&buf).ok_or(Errno::EFAULT)
}

pub fn getsockopt() -> EResult<()> {
    warn!("getsockopt is a stub!");
    Ok(())
}

pub fn setsockopt(
    fd: i32,
    layer: i32,
    number: i32,
    buffer: VirtAddr,
    size: socklen_t,
) -> EResult<()> {
    warn!("setsockopt is a stub!");

    let socket = fd_to_socket(fd)?;
    socket.set_opt()
}

pub fn sendmsg(fd: i32, msg_ptr: VirtAddr, flags: i32) -> EResult<isize> {
    warn!("sendmsg({}, {:?}, {:#x}) is a stub!", fd, msg_ptr, flags);

    Err(Errno::ENOSYS)
}

pub fn recvmsg(fd: i32, msg_ptr: VirtAddr, flags: i32) -> EResult<isize> {
    warn!("recvmsg({}, {:?}, {:#x}) is a stub!", fd, msg_ptr, flags);

    Err(Errno::ENOSYS)
}
