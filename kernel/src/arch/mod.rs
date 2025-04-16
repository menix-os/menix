cfg_match! {
    target_arch = "x86_64" => {
        pub mod x86_64;

        mod internal {
            pub use super::x86_64::*;
        }
    }
    target_arch = "aarch64" => {
        pub mod aarch64;

        mod internal {
            pub use super::aarch64::*;
        }
    }
    target_arch = "riscv64" => {
        pub mod riscv64;

        mod internal {
            pub use super::riscv64::*;
        }
    }
    target_arch = "loongarch64" => {
        pub mod loongarch64;

        mod internal {
            pub use super::loongarch64::*;
        }
    }
}

// Re-export only parts of the architecture implementation that get called by generic code.
pub use internal::init;
pub use internal::irq;
pub use internal::page;
pub use internal::percpu;
pub use internal::platform;
