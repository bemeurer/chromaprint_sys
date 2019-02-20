use bindgen;
use metadeps;

fn main() {
    // Make sure we have chromaprint
    let libs = metadeps::probe().unwrap();
    // Generate bindings
    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .no_copy("(?i)mutex");
    for path in &libs["libchromaprint"].include_paths {
        bindings = bindings.clang_arg(format!("-I{:?}", path))
    }
    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}
