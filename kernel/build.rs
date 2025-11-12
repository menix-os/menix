use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rustc-link-arg=-Tkernel/kernel.ld");
    println!("cargo::rerun-if-changed=kernel/kernel.ld");

    Ok(())
}
