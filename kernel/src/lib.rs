#![no_std]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![feature(negative_impls)]
#![feature(naked_functions)]
#![feature(allocator_api)]
// Needed for volatile memmove
#![allow(internal_features)]
#![feature(core_intrinsics)]

extern crate alloc;
extern crate core;

#[macro_use]
pub mod macros;

pub mod arch;
pub mod boot;
pub mod generic;

pub const MENIX_VERSION: &str = env!("CARGO_PKG_VERSION");
