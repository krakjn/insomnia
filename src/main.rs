mod platform;
use std::io::BufRead;

fn main() {
    platform::inhibit_sleep();
}

pub fn wait_for_user_input() {
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let _ = reader.read_line(&mut String::new());
}
