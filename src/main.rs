use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy(
        "org.freedesktop.login1",
        "/org/freedesktop/login1",
        Duration::from_secs(30),
    );

    let what = "sleep";
    let who = "insomnia";
    let why = "Preventing system sleep for operational reasons";
    let mode = "block"; // Mode can be 'block' or 'delay'

    // Call the Inhibit method
    let (fd,): (dbus::arg::OwnedFd,) = proxy.method_call(
        "org.freedesktop.login1.Manager",
        "Inhibit",
        (what, who, why, mode),
    )?;

    println!("Sleep inhibited. Press key to release inhibition.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?; // Wait for user input

    drop(fd); // release the inhibition when fd goes out of scope
    println!("Inhibition released.");
    Ok(())
}
