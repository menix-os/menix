use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rustc-link-arg=-Tkernel/kernel.ld");
    println!("cargo::rerun-if-changed=kernel/kernel.ld");

    let mut version = String::new();
    if std::env::var("CARGO_FEATURE_BOOT_LIMINE").is_ok() {
        version += "BOOT_LIMINE";
    }
    println!("cargo::rustc-env=MENIX_VERSION={version}");

    Ok(())
}
