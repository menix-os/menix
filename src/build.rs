use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tsrc/arch/{arch}/{arch}.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=src/arch/{arch}/{arch}.ld");

    Ok(())
}
