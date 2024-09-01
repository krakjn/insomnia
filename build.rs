fn main() {
    #[cfg(target_os = "macos")]
    {
        cc::Build::new()
            .file("src/platform/macos.c")
            .compile("insomnia-macos");
        // These lines ensure the linker knows about the necessary macOS frameworks
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");

        // Optional: Specify the minimum macOS version if necessary
        println!("cargo:rustc-link-arg=-mmacosx-version-min=10.15");
    }
}
