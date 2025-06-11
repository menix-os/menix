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
    #[entails(ARCH_STAGE)]
    pub EARLY_STAGE: "arch.early" => || unsafe { core::setup_bsp() };

    #[depends(EARLY_STAGE, crate::generic::memory::MEMORY_STAGE)]
    #[entails(AP_INIT_STAGE)]
    pub APS_DISCOVERED_STAGE: "arch.aps-discovered" => || {};

    #[depends(APS_DISCOVERED_STAGE)]
    #[entails(ARCH_STAGE)]
    pub AP_INIT_STAGE: "arch.aps-initialized" => || {};

    #[depends(crate::generic::memory::MEMORY_STAGE)]
    pub ARCH_STAGE: "arch" => || {};
}
