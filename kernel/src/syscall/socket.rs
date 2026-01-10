use crate::{
    device::net::{Socket, create_socket},
    memory::{UserSlice, VirtAddr},
    posix::errno::{EResult, Errno},
    sched::Scheduler,
    uapi::socket::socklen_t,
    util::mutex::Mutex,
    vfs::{
        File,
        file::{FileDescription, OpenFlags},
        inode::{INode, NodeOps},
    },
};
use alloc::sync::Arc;
use core::sync::atomic::AtomicBool;

pub fn socket(family: i32, typ: i32, protocol: i32) -> EResult<i32> {
    let socket = create_socket(family as _, typ as _).ok_or(Errno::EAFNOSUPPORT)?;

    warn!(
        "socket({:#x}, {:#x}, {:#x}) is a stub!",
        family, typ, protocol
    );

    let node = INode::new(NodeOps::Socket(socket.clone()), None);

    let proc = Scheduler::get_current().get_process();
    let mut files = proc.open_files.lock();
    let fd = files
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
        .ok_or(Errno::EMFILE)?;

    log!("Opened socket on FD {:?}", fd);

    Ok(fd)
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
    let buffer = UserSlice::new(addr_ptr, addr_length as _);
    socket.bind(buffer.as_slice().ok_or(Errno::EFAULT)?)
}

pub fn connect() -> EResult<()> {
    warn!("connect is a stub!");
    Err(Errno::ENOSYS)
}

pub fn accept() -> EResult<()> {
    warn!("accept is a stub!");
    Err(Errno::ENOSYS)
}

pub fn listen(fd: i32, backlog: i32) -> EResult<()> {
    warn!("listen is a stub!");

    let socket = fd_to_socket(fd)?;

    socket.listen(backlog)
}

pub fn getpeername() -> EResult<()> {
    warn!("getpeername is a stub!");
    Err(Errno::ENOSYS)
}

pub fn getsockname() -> EResult<()> {
    warn!("getsockname is a stub!");
    Err(Errno::ENOSYS)
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

pub fn sendmsg() -> EResult<()> {
    warn!("sendmsg is a stub!");
    Err(Errno::ENOSYS)
}

pub fn recvmsg() -> EResult<()> {
    warn!("recvmsg is a stub!");
    Err(Errno::ENOSYS)
}
