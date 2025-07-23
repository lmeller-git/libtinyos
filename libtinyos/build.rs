use std::env;

extern crate cbindgen;

fn main() {
    println!("cargo:rerun-if-changed=NULL");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .unwrap()
        .write_to_file("libtinyos.h");
}
