use bindgen;
#[cfg(feature = "dynamic")]
use metadeps;
#[cfg(feature = "static")]
use cmake;

#[cfg(feature = "dynamic")]
fn get_dynamic_chromaprint(mut bindings: bindgen::Builder) -> bindgen::Builder {
    // Make sure we have chromaprint
    let libs = metadeps::probe().unwrap();
    for path in &libs["libchromaprint"].include_paths {
        bindings = bindings.clang_arg(format!("-I{:?}", path))
    }
    bindings
}

#[cfg(feature = "static")]
fn build_chromaprint() -> std::path::PathBuf {
    cmake::Config::new("vendor").define("CMAKE_BUILD_TYPE", "RELEASE").build()
}

#[cfg(feature = "static")]
fn link_chromaprint(lib_path: std::path::PathBuf){
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=vendor");
}

fn main() {
    // Generate bindings
    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .no_copy("(?i)mutex");

    if cfg!(feature = "dynamic") {
        bindings = get_dynamic_chromaprint(bindings);
    } else if cfg!(feature = "static") {
        #[cfg(feature = "static")]
        let lib_path = build_chromaprint();
        #[cfg(feature = "static")]
        link_chromaprint(lib_path);
        bindings = bindings.clang_arg("-Ivendor/src")
    }

    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
