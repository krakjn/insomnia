extern "C" {
    fn set_assertion() -> bool;
    fn release_assertion() -> bool;
}

pub fn inhibit_sleep() {
    let insomnia: bool;
    unsafe {
        insomnia = set_assertion();
    }
    if insomnia {
        println!("Sleep inhibited. Press Enter to release inhibition...");
        crate::wait_for_user_input();
        unsafe {
            release_assertion();
        }
        println!("Inhibition released.");
    }
}
