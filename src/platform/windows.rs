use std::ptr::null_mut;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::PowerCreateRequest;
use winapi::um::winnt::{REASON_CONTEXT, POWER_REQUEST_CONTEXT_VERSION};
use winapi::um::powerbase::{PowerSetRequest, PowerRequestSystemRequired, PowerRequestDisplayRequired};

pub fn inhibit_sleep() {
    let power_request = create_power_request();
    if power_request.is_null() {
        eprintln!("Failed to create power request.");
        std::process::exit(1);
    }

    set_power_request(power_request);

    println!("Sleep inhibited. Press Enter to release inhibition.");
    crate::common::wait_for_user_input();

    release_inhibition_windows(power_request);
    println!("Inhibition released.");
}

fn create_power_request() -> *mut std::ffi::c_void {
    let mut context = REASON_CONTEXT {
        Version: POWER_REQUEST_CONTEXT_VERSION,
        Flags: 0,
        Reason: null_mut(),
    };

    unsafe { PowerCreateRequest(&mut context) }
}

fn set_power_request(power_request: *mut std::ffi::c_void) {
    unsafe {
        PowerSetRequest(power_request, PowerRequestSystemRequired);
        PowerSetRequest(power_request, PowerRequestDisplayRequired);
    }
}

fn release_inhibition_windows(power_request: *mut std::ffi::c_void) {
    unsafe {
        CloseHandle(power_request);
    }
}
