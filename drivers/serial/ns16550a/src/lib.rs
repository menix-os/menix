#![no_std]

use menix::{
    alloc::string::String,
    log,
    posix::errno::EResult,
    system::dt::{Node, driver::Driver},
};

menix::module!("NS16550a serial driver", "Marvin Friedrich", main);

static DRIVER: Driver = Driver {
    name: "ns16550a",
    compatible: &[b"ns16550a"],
    probe,
};

fn probe(node: &Node) -> EResult<()> {
    log!("Hello from {}", String::from_utf8_lossy(node.name()));

    Ok(())
}

pub fn main() {
    match DRIVER.register() {
        Ok(_) => (),
        Err(e) => menix::error!("Unable to load driver: {:?}", e),
    }
}
