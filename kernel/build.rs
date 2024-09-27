fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tkernel/arch/{arch}/{arch}.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=kernel/arch/{arch}/{arch}.ld");
}
