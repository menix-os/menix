#![no_std]
#![allow(unused)]
#![allow(clippy::needless_return)]
#![feature(negative_impls)]
#![feature(naked_functions)]
#![feature(allocator_api)]
// Needed for volatile memmove
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(str_from_raw_parts)]
#![feature(new_zeroed_alloc)]

pub extern crate alloc;
pub extern crate core;
pub use spin;

#[macro_use]
pub mod macros;

pub mod arch;
pub mod boot;
pub mod generic;

pub const MENIX_VERSION: &str = env!("CARGO_PKG_VERSION");
