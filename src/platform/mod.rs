#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub fn inhibit_sleep() {
    #[cfg(target_os = "linux")]
    linux::inhibit_sleep();

    #[cfg(target_os = "macos")]
    macos::inhibit_sleep();

    #[cfg(target_os = "windows")]
    windows::inhibit_sleep();
}
