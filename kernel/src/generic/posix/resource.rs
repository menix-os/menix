pub enum Resource {
    Cpu = uapi::RLIMIT_CPU as _,
    FileSize = uapi::RLIMIT_FSIZE as _,
    Data = uapi::RLIMIT_DATA as _,
    Stack = uapi::RLIMIT_STACK as _,
    Core = uapi::RLIMIT_CORE as _,
    Rss = uapi::RLIMIT_RSS as _,
    NumProc = uapi::RLIMIT_NPROC as _,
    NumFiles = uapi::RLIMIT_NOFILE as _,
    MemLock = uapi::RLIMIT_MEMLOCK as _,
    AddressSpace = uapi::RLIMIT_AS as _,
    Locks = uapi::RLIMIT_LOCKS as _,
    SigPending = uapi::RLIMIT_SIGPENDING as _,
    MsgQueue = uapi::RLIMIT_MSGQUEUE as _,
    Nice = uapi::RLIMIT_NICE as _,
    RtPrio = uapi::RLIMIT_RTPRIO as _,
    RtTime = uapi::RLIMIT_RTTIME as _,
}

pub struct Limits {
    pub open_max: uapi::rlimit,
    pub core_size: uapi::rlimit,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            open_max: uapi::rlimit {
                rlim_cur: 1024,
                rlim_max: 1024,
            },
            core_size: uapi::rlimit {
                rlim_cur: 0,
                rlim_max: 0,
            },
        }
    }
}
