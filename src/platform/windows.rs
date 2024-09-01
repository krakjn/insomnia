use std::ptr::null_mut;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Power::{
    PowerCreateRequest, PowerSetRequest, PowerRequestDisplayRequired, PowerRequestSystemRequired,
};
use windows::Win32::System::SystemServices::{CloseHandle};
use windows::Win32::System::Threading::{
    REASON_CONTEXT, REASON_CONTEXT_Reason, REASON_CONTEXT_Detailed, POWER_REQUEST_CONTEXT_VERSION,
};

pub fn inhibit_sleep() {
    let power_request = create_power_request();
    if power_request.is_invalid() {
        eprintln!("Failed to create power request.");
        std::process::exit(1);
    }

    set_power_request(power_request);

    println!("Sleep inhibited. Press Enter to release inhibition...");
    crate::wait_for_user_input();

    release_inhibition_windows(power_request);
    println!("Inhibition released.");
}

fn create_power_request() -> HANDLE {
    let reason = REASON_CONTEXT_Reason {
        Detailed: REASON_CONTEXT_Detailed {
            LocalizedReasonModule: null_mut(),
            LocalizedReasonId: 0,
            ReasonStringCount: 0,
            ReasonStrings: null_mut(),
        },
    };
    let context = REASON_CONTEXT {
        Version: POWER_REQUEST_CONTEXT_VERSION,
        Flags: 0,
        Reason: reason,
    };

    unsafe { PowerCreateRequest(&context) }
}

fn set_power_request(power_request: HANDLE) {
    unsafe {
        PowerSetRequest(power_request, PowerRequestSystemRequired);
        PowerSetRequest(power_request, PowerRequestDisplayRequired);
    }
}

fn release_inhibition_windows(power_request: HANDLE) {
    unsafe {
        CloseHandle(power_request);
    }
}
