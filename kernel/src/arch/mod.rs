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
pub use internal::firmware;
pub use internal::init;
pub use internal::irq;
pub use internal::page;
pub use internal::percpu;

/// Represents a physical address. It can't be directly read from or written to.
pub use internal::PhysAddr;
/// Represents a virtual address. It can't be directly read from or written to.
/// Note: Not the same as a pointer. A `VirtAddr` might point into another
/// process's memory that is not mapped in the kernel.
pub use internal::VirtAddr;
