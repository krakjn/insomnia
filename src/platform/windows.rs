use windows::core::PWSTR;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Power::{
    PowerCreateRequest, PowerSetRequest, PowerClearRequest, 
    PowerRequestSystemRequired, PowerRequestDisplayRequired,
};
use windows::Win32::System::Threading::{
    REASON_CONTEXT, POWER_REQUEST_CONTEXT_SIMPLE_STRING, REASON_CONTEXT_0,
};

pub fn inhibit_sleep() {
    let mut reason_string: Vec<u16> = "Preventing system sleep\0".encode_utf16().collect();
    let reason_context = REASON_CONTEXT {
        Version: 0,
        Flags: POWER_REQUEST_CONTEXT_SIMPLE_STRING,
        Reason: REASON_CONTEXT_0 {
            SimpleReasonString: PWSTR(reason_string.as_mut_ptr()),
        },
    };
    let handle: HANDLE = unsafe { 
        PowerCreateRequest(&reason_context)
    }.expect("unable to call PowerCreateRequest");

    unsafe {
        let _ = PowerSetRequest(handle, PowerRequestSystemRequired);
        let _ = PowerSetRequest(handle, PowerRequestDisplayRequired);
    }

    println!("Sleep inhibited. Press Enter to release inhibition...");
    crate::wait_for_user_input();

    unsafe {
        let _ = PowerClearRequest(handle, PowerRequestSystemRequired);
        let _ = PowerClearRequest(handle, PowerRequestDisplayRequired);
    }
    println!("Inhibition released.");
}