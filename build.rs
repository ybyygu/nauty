// [[file:nauty.note::*build.rs][build.rs:1]]
use bindgen;
use cc;

use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=m");
    cc::Build::new()
        .cpp(false)
        .include("lib")
        .file("lib/nauty.c")
        .file("lib/nautil.c")
        .file("lib/nausparse.c")
        .file("lib/naugraph.c")
        .file("lib/schreier.c")
        .file("lib/naurng.c")
        .file("lib/traces.c")
        .file("lib/gtools.c")
        .file("lib/naututil.c")
        .file("lib/nautinv.c")
        .file("lib/gutil1.c")
        .file("lib/gutil2.c")
        .file("lib/gtnauty.c")
        .file("lib/naugroup.c")
        .file("lib/nautycliquer.c")
        .compile("libnauty.a");

    let bindings = bindgen::Builder::default()
        .header("lib/nauty.h")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
// build.rs:1 ends here
