fn main() {
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("src/platform/macos.c")
            .compile("insomnia-macos");
        // ensure the linker knows about the necessary macOS frameworks
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-arg=-mmacosx-version-min=10.15");
    }
}
