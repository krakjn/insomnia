mod platform;
#[cfg(target_os = "macos")]
mod bindings;

fn main() {
    platform::inhibit_sleep();
}

mod common {
    use std::io::BufRead;
    pub fn wait_for_user_input() {
        let stdin = std::io::stdin();
        let mut reader = stdin.lock();
        let _ = reader.read_line(&mut String::new());
    }
}
