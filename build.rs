use bindgen;
#[cfg(feature = "dynamic")]
use metadeps;
#[cfg(feature = "static")]
use cmake;

use std::path::PathBuf;

fn create_builder() -> bindgen::Builder {
    bindgen::Builder::default()
        .header("wrapper.h")
        .no_copy("(?i)mutex")
}

fn write_bindings(builder: bindgen::Builder) {
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}

#[cfg(feature = "dynamic")]
fn get_dynamic_chromaprint() -> Vec<PathBuf> {
    // Make sure we have chromaprint
    let libs = metadeps::probe().unwrap();
    libs["libchromaprint"].include_paths.to_owned()
}

#[cfg(feature = "dynamic")]
fn include_dynamic() {
        let mut builder = create_builder();
        let include_paths = get_dynamic_chromaprint();
        for path in include_paths {
            builder = builder.clang_arg(format!("-I{:?}", path))
        }
        write_bindings(builder);
}

#[cfg(feature = "static")]
fn build_chromaprint() -> PathBuf {
    cmake::Config::new("vendor").define("CMAKE_BUILD_TYPE", "RELEASE").build()
}

#[cfg(feature = "static")]
fn link_chromaprint(lib_path: PathBuf){
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=vendor");
}

#[cfg(feature = "static")]
fn include_static() {
        let lib_path = build_chromaprint();
        link_chromaprint(lib_path);
        let builder = create_builder().clang_arg("-Ivendor/src");
        write_bindings(builder);
}

fn main() {
    #[cfg(feature = "dynamic")]
    include_dynamic();
    #[cfg(feature = "static")]
    include_static();
}
