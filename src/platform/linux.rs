use dbus::blocking::Connection;
use std::os::unix::io::RawFd;

pub fn inhibit_sleep() {
    let conn = connect_to_system_bus();
    let fd = call_inhibit_method(&conn);

    println!("Sleep inhibited. Press Enter to release inhibition.");
    crate::common::wait_for_user_input();

    release_inhibition_unix(fd);
    println!("Inhibition released.");
}

fn connect_to_system_bus() -> Connection {
    Connection::new_system().expect("Failed to connect to system bus")
}

fn call_inhibit_method(conn: &Connection) -> RawFd {
    // use dbus::blocking::Proxy;
    use dbus::arg::OwnedFd;

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

fn release_inhibition_unix(fd: RawFd) {
    unsafe {
        libc::close(fd);
    }
}
