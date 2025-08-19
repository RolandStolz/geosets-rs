use bindgen;

// build.rs
fn main() {
    // Tell Cargo to link against libcdd
    println!("cargo:rustc-link-lib=cdd");

    let bindings = bindgen::Builder::default()
        .header("src/cddlib_rs/cdd_wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Add these flags to help with parsing
        .clang_arg("-I/opt/homebrew/include")
        .clang_arg("-std=c99")
        .clang_arg("-fparse-all-comments")
        // Allowlist only the functions you need
        .allowlist_function("dd_.*")
        .allowlist_type("dd_.*")
        .allowlist_var("dd_.*")
        .allowlist_function("_dd_.*")
        .allowlist_type("_dd_.*")
        .allowlist_var("_dd_.*")
        // Blocklist problematic system functions
        // Generate better function signatures
        .generate_comments(false)
        .size_t_is_usize(true)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .raw_line("#![allow(warnings)]")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/cddlib_rs/cdd_bindings.rs")
        .expect("Couldn't write bindings!");

    // Tell Cargo where to find libcdd (adjust if needed)
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");

    // Optional: re-run build if these change
    println!("cargo:rerun-if-env-changed=LIBCDD_LIB_DIR");
    println!("cargo:rerun-if-env-changed=LIBCDD_INCLUDE_DIR");
}
