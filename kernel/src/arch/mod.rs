//! This module and all submodules contain architecture dependent code.
//! All architectures should implement all functions referenced by the submodules of this module.

cfg_match! {
    target_arch = "x86_64" => {
        mod x86_64;

        mod internal {
            pub use super::x86_64::*;
        }
    }
    _ => { compile_error!("Unsupported architecture!") }
}

pub mod core;
pub mod irq;
pub mod memory;
pub mod platform;
pub mod sched;
