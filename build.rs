fn main() {
    #[cfg(target_os = "macos")]
    {
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}
