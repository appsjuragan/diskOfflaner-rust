#[cfg(target_os = "windows")]
mod disk_operations_windows;

#[cfg(target_os = "windows")]
pub use disk_operations_windows::*;

#[cfg(target_os = "linux")]
mod disk_operations_linux;

#[cfg(target_os = "linux")]
pub use disk_operations_linux::*;
