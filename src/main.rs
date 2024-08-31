use std::io::{self, BufRead};

#[cfg(unix)]
use std::os::unix::io::RawFd;

#[cfg(windows)]
use std::os::windows::io::RawHandle;
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;

#[cfg(unix)]
use dbus::blocking::Connection;

#[cfg(unix)]
use dbus::arg::OwnedFd;

fn main() {
    #[cfg(unix)]
    inhibit_sleep_unix();

    #[cfg(windows)]
    inhibit_sleep_windows();
}

#[cfg(unix)]
fn inhibit_sleep_unix() {
    let conn = connect_to_system_bus();
    let fd = call_inhibit_method(&conn);

    println!("Sleep inhibited. Press Enter to release inhibition.");
    wait_for_user_input();

    release_inhibition_unix(fd);
    println!("Inhibition released.");
}

#[cfg(unix)]
fn connect_to_system_bus() -> Connection {
    Connection::new_system().expect("Failed to connect to system bus")
}

#[cfg(unix)]
fn call_inhibit_method(conn: &Connection) -> RawFd {
    let proxy = conn.with_proxy(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        std::time::Duration::from_millis(5000),
    );

    let what = "sleep";
    let who = "insomnia";
    let why = "Preventing system sleep for operational reasons";
    let mode = "block"; // Mode can be 'block' or 'delay'

    let (fd,): (OwnedFd,) = proxy
        .method_call(
            "org.freedesktop.login1.Manager",
            "Inhibit",
            (what, who, why, mode),
        )
        .expect("Failed to call Inhibit");

    fd.into_fd()
}

#[cfg(unix)]
fn release_inhibition_unix(fd: RawFd) {
    unsafe {
        libc::close(fd);
    }
}

#[cfg(windows)]
fn inhibit_sleep_windows() {
    let power_request = create_power_request();
    if power_request.is_null() {
        eprintln!("Failed to create power request.");
        exit(1);
    }

    set_power_request(power_request);

    println!("Sleep inhibited. Press Enter to release inhibition.");
    wait_for_user_input();

    release_inhibition_windows(power_request);
    println!("Inhibition released.");
}

#[cfg(windows)]
fn create_power_request() -> RawHandle {
    use std::ptr::null_mut;
    use winapi::um::winbase::PowerCreateRequest;
    use winapi::um::winnt::{POWER_REQUEST_CONTEXT_VERSION, REASON_CONTEXT};

    let mut context = REASON_CONTEXT {
        Version: POWER_REQUEST_CONTEXT_VERSION,
        Flags: 0,
        Reason: null_mut(),
    };

    unsafe { PowerCreateRequest(&mut context) }
}

#[cfg(windows)]
fn set_power_request(power_request: RawHandle) {
    use winapi::um::powerbase::*;

    unsafe {
        PowerSetRequest(power_request, PowerRequestSystemRequired);
        PowerSetRequest(power_request, PowerRequestDisplayRequired);
    }
}

#[cfg(windows)]
fn release_inhibition_windows(power_request: RawHandle) {
    unsafe {
        CloseHandle(power_request);
    }
}

fn wait_for_user_input() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let _ = reader.read_line(&mut String::new());
}
