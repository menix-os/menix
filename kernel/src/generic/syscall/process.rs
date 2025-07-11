use crate::generic::{posix::errno::EResult, sched::Scheduler};

pub fn gettid(a0: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> EResult<usize> {
    Ok(Scheduler::get_current().get_id())
}

pub fn exit(error: usize, _: usize, _: usize, _: usize, _: usize, _: usize) -> EResult<usize> {
    let proc = Scheduler::get_current().get_process();

    if proc.get_pid() <= 1 {
        panic!("Attempted to kill init with error code {error}");
    }

    todo!()
}
