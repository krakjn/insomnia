extern "C" {
    fn set_assertion();
    fn release_assertion();
}

pub fn inhibit_sleep() {
    unsafe {
        set_assertion();
    }
    println!("Sleep inhibited. Press Enter to release inhibition.");
    crate::common::wait_for_user_input();
    unsafe {
        release_assertion();
    }
    println!("Inhibition released.");
}
