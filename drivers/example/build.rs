fn main() {
    println!("cargo::rustc-link-arg=-Tdrivers/module.ld");
    println!("cargo::rerun-if-changed=drivers/module.ld");
}
