// use std::io::BufRead;
use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED, EXECUTION_STATE};


pub fn inhibit_sleep() {
    let state: EXECUTION_STATE = unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED)
    };

    if state.0 == 0 {
        eprintln!("Failed to set thread execution state");
    } else {
        println!("System sleep is now prevented.");
    }

    println!("Sleep inhibited. Press Enter to release inhibition...");
    crate::wait_for_user_input();

    // call SetThreadExecutionState again to clear the flag
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }
    println!("Inhibition released.");
}