use std::ffi::CString;
use crate::bindings::*;

pub fn inhibit_sleep() {
    let assertion_id = create_assertion_macos();
    if assertion_id == 0 {
        eprintln!("Failed to create power assertion.");
        std::process::exit(1);
    }

    println!("Sleep inhibited. Press Enter to release inhibition.");
    crate::common::wait_for_user_input();

    release_assertion_macos(assertion_id);
    println!("Inhibition released.");
}

fn create_assertion_macos() -> u32 {
    let assertion_name = CString::new("Preventing system sleep for operational reasons").unwrap();
    let assertion_name_ref = assertion_name.as_ptr() as CFStringRef;

    let mut assertion_id: u32 = 0;

    unsafe {
        IOPMAssertionCreateWithName(
            kIOPMAssertionTypeNoIdleSleep,
            kIOPMAssertionLevelOn,
            assertion_name_ref,
            &mut assertion_id as *mut u32,
        );
    }

    assertion_id
}

fn release_assertion_macos(assertion_id: u32) {
    unsafe {
        IOPMAssertionRelease(assertion_id);
    }
}
