use crate::generic::{posix::errno::EResult, sched::Scheduler};

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

pub fn exit(error: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();

    if proc.get_pid() <= 1 {
        panic!("Attempted to kill init with error code {error}");
    }

    todo!()
}

pub fn fork() -> EResult<usize> {
    todo!();
}

pub fn execve() -> EResult<usize> {
    todo!()
}
