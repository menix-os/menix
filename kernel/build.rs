use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;

    // Set the linker script for the current target.
    println!("cargo::rustc-link-arg=-Ttoolchain/kernel.ld");
    println!("cargo::rerun-if-changed=toolchain/kernel.ld");

    Ok(())
}
