fn main() {
    #[cfg(target_os = "macos")]
    {
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks")
        // Blocklist the problematic enum or identifier
        .blocklist_type(".*unnamed_at_.*")  // Regex to match the problematic item
        .blocklist_type(".*anonymous_at_.*")  // Regex to match the problematic item
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
    }
}
