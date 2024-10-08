use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;
    println!("cargo:rustc-link-arg=-Ttoolchain/{arch}.ld");
    println!("cargo:rerun-if-changed=toolchain/{arch}.ld");

    Ok(())
}
