cfg_match! {
    target_arch = "x86_64" => {
        pub mod x86_64;

        mod internal {
            pub use super::x86_64::*;
        }
    }
}

// Re-export only parts of the architecture implementation that get called by generic code.
pub use internal::cpu;
pub use internal::irq;
pub use internal::page;
pub use internal::platform;
