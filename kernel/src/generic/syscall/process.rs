use crate::{
    arch::sched::Context,
    generic::{
        memory::VirtAddr,
        percpu::CPU_DATA,
        posix::errno::{EResult, Errno},
        sched::Scheduler,
    },
};

pub fn gettid() -> usize {
    Scheduler::get_current().get_id()
}

pub fn getpid() -> usize {
    Scheduler::get_current().get_process().get_pid()
}

pub fn getppid() -> usize {
    Scheduler::get_current()
        .get_process()
        .get_parent()
        .map_or(0, |x| x.get_pid())
}

pub fn getuid() -> usize {
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    inner.identity.user_id as usize
}

pub fn geteuid() -> usize {
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    inner.identity.effective_user_id as usize
}

pub fn getgid() -> usize {
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    inner.identity.group_id as usize
}

pub fn getegid() -> usize {
    let proc = Scheduler::get_current().get_process();
    let inner = proc.inner.lock();
    inner.identity.effective_group_id as usize
}

pub fn getpgid(pid: usize) -> EResult<usize> {
    if pid != 0 {
        return Err(Errno::EINVAL);
    }

    let proc = Scheduler::get_current().get_process();
    Ok(proc.get_pid())
}

pub fn exit(error: usize) -> ! {
    let proc = Scheduler::get_current().get_process();

    if proc.get_pid() <= 1 {
        panic!("Attempted to kill init with error code {error}");
    }

    Scheduler::kill_current();
}

pub fn fork(ctx: &Context) -> EResult<usize> {
    let old = Scheduler::get_current().get_process();

    // Fork the current process. This puts both processes at this point in code.
    let (new_proc, new_task) = old.fork(ctx)?;
    CPU_DATA.get().scheduler.add_task(new_task.clone());

    Ok(new_proc.get_pid())
}

pub fn execve(path: VirtAddr, argv: VirtAddr, envp: VirtAddr) -> EResult<usize> {
    todo!()
}
