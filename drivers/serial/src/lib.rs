#![no_std]

use menix::{
    alloc::string::String,
    generic::posix::errno::EResult,
    log,
    system::dt::{Node, driver::Driver},
};

menix::module!("Serial devices", "Marvin Friedrich", main);

static DRIVER: Driver = Driver {
    name: "serial",
    compatible: &[b"ns16550a"],
    probe,
};

fn probe(node: &Node) -> EResult<()> {
    log!("Hello from {}", String::from_utf8_lossy(node.name()));

    Ok(())
}

pub fn main() {
    DRIVER.register().unwrap();
}
