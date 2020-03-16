use std::path::PathBuf;

#[cfg(not(feature = "vendor"))]
fn dyn_link() {
    pkg_config::Config::new()
        .atleast_version("1.4.3")
        .statik(true)
        .probe("libchromaprint")
        .expect("Failed to find libchromaprint");
}

#[cfg(feature = "vendor")]
fn vendor() {
    println!("cargo:rerun-if-changed=vendor/chromaprint");
    let build_type = if cfg!(debug_assertions) {
        "DEBUG"
    } else {
        "RELEASE"
    };
    let lib_path = cmake::Config::new("vendor/chromaprint")
        .define("CMAKE_BUILD_TYPE", build_type)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("FFT_LIB", "avfft")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=chromaprint");
}

fn main() {
    #[cfg(not(feature = "vendor"))]
    dyn_link();
    #[cfg(feature = "vendor")]
    vendor();

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
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
