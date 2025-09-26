use crate::generic::{
    memory::user::UserPtr,
    posix::errno::{EResult, Errno},
};

impl super::Tty {
    pub fn tiocgwinsz(&self, arg: UserPtr<uapi::winsize>) -> EResult<()> {
        arg.write(*self.winsize.lock()).ok_or(Errno::EINVAL)
    }
}
