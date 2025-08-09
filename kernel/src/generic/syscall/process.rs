use crate::{
    arch::sched::Context,
    generic::{memory::VirtAddr, percpu::CPU_DATA, posix::errno::EResult, sched::Scheduler},
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

pub fn do_yield() -> EResult<usize> {
    // TODO
    Ok(0)
}
