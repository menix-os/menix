pub mod boot;
pub mod clock;
pub mod cmdline;
pub mod fbcon;
pub mod init;
pub mod irq;
pub mod log;
pub mod memory;
pub mod module;
pub mod panic;
pub mod percpu;
pub mod posix;
pub mod process;
pub mod resource;
pub mod syscall;
pub mod util;
pub mod vfs;

init_stage! {
    pub GENERIC_STAGE: "generic" => || {};
}
