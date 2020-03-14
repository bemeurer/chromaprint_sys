use bindgen;
use pkg_config;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    pkg_config::Config::new()
        .atleast_version("1.1")
        .probe("libchromaprint")
        .expect("Failed to find libchromaprint");
    bindgen::Builder::default()
        .generate_comments(true)
        .header_contents("wrapper.h", "#include<chromaprint.h>")
        .layout_tests(true)
        .no_copy("(?i)mutex")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .rustfmt_bindings(true)
        .generate()
        .expect("Failed to generate chromaprint bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
