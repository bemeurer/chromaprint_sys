use bindgen;
use pkg_config;
use std::path::PathBuf;

fn main() {
    pkg_config::Config::new()
        .atleast_version("1.1")
        .probe("libchromaprint")
        .expect("Failed to find libchromaprint");
    let bindings = bindgen::Builder::default()
        .no_copy("(?i)mutex")
        .header_contents("wrapper.h", "#include<chromaprint.h>")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate chromaprint bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
