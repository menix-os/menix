use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Set the linker script for the current boot method.
    if std::env::var("CARGO_FEATURE_BOOT_LIMINE").is_ok() {
        println!("cargo::rustc-link-arg=-Tkernel/kernel.ld");
        println!("cargo::rerun-if-changed=kernel/kernel.ld");
    }

    Ok(())
}
