#![no_std]
#![no_main]
#![allow(unused)]
#![feature(negative_impls)]
#![feature(naked_functions)]
extern crate alloc;
extern crate core;

#[macro_use]
pub mod macros;

pub mod arch;
pub mod boot;
pub mod firmware;
pub mod generic;
