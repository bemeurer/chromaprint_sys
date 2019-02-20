use bindgen;
#[cfg(feature = "static")]
use cmake;
#[cfg(feature = "dynamic")]
use metadeps;

use std::path::PathBuf;

fn create_builder() -> bindgen::Builder {
    bindgen::Builder::default()
        .header("wrapper.h")
        .no_copy("(?i)mutex")
}

fn write_bindings(builder: bindgen::Builder) {
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "dynamic")]
fn include_dynamic() {
    let mut builder = create_builder();

    // Make sure we have chromaprint
    let libs = metadeps::probe().unwrap();
    let include_paths = libs["libchromaprint"].include_paths.to_owned();
    for path in include_paths {
        builder = builder.clang_arg(format!("-I{:?}", path))
    }

    write_bindings(builder);
}

#[cfg(feature = "static")]
fn include_static() {
    let builder = create_builder().clang_arg("-Ivendor/src");

    let lib_path = cmake::Config::new("chromaprint")
        .define("CMAKE_BUILD_TYPE", "RELEASE")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=chromaprint");

    write_bindings(builder);
}

fn main() {
    #[cfg(feature = "dynamic")]
    include_dynamic();
    #[cfg(feature = "static")]
    include_static();
}
