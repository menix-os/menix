//! This module and all submodules contain architecture dependent code.
//! All architectures should implement all functions referenced by the submodules of this module.

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
mod internal {
    pub use super::x86_64::*;
}

pub mod core;
pub mod irq;
pub mod sched;
pub mod virt;

init_stage! {
    pub EARLY_INIT_STAGE: "arch.early-init" => || {};

    #[depends(EARLY_INIT_STAGE)]
    pub INIT_STAGE: "arch.init" => || {};
}
