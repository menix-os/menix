fn main() {
    let mut b = cc::Build::new();
    b.files(["flanterm/flanterm.c", "flanterm/backends/fb.c"])
        .includes(["flanterm"])
        .pic(true)
        .flag("-ffreestanding")
        .flag("-nostdlib");

    if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
        b.flag("-mgeneral-regs-only");
        b.flag("-mno-red-zone");
    }

    b.compile("flanterm");

    let bindings = bindgen::builder()
        .use_core()
        .wrap_unsafe_ops(true)
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .clang_args(["-I", "flanterm"])
        .header("src/wrapper.h")
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings!");
}
