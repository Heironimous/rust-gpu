extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};


#[derive(Debug)]
struct MacroCallback {
    macros: Arc<RwLock<HashSet<String>>>,
}

impl ParseCallbacks for MacroCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        self.macros.write().unwrap().insert(name.into());

        return match name {
            "FP_SUBNORMAL" | 
            "FP_NORMAL" | 
            "FP_ZERO" | 
            "FP_INFINITE" | 
            "FP_NAN" => MacroParsingBehavior::Ignore,

            _ => MacroParsingBehavior::Default
        }
    }
}


fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=SDL2_gpu");


    let macros = Arc::new(RwLock::new(HashSet::new()));

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
    // Finish the builder and generate the bindings.

        .parse_callbacks(Box::new(MacroCallback {macros: macros.clone()}))
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
