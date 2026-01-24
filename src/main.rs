#![cfg_attr(windows, windows_subsystem = "windows")]

use anyhow::{Context, Result};
use colored::Colorize;
use std::env;
use std::io::{self, Write};
use std::mem;
use std::ptr;

mod disk_operations;
mod gui;
mod structs;

#[cfg(test)]
mod tests;

use disk_operations::{enumerate_disks, set_disk_offline, set_disk_online};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If arguments are provided, try to attach to the parent console to show output
    if args.len() > 1 {
        #[cfg(windows)]
        unsafe {
            use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
            AttachConsole(ATTACH_PARENT_PROCESS);
        }
    }

    // Check for admin privileges
    if !is_elevated() {
        // If we attached console, this will print there. If not (GUI mode), it won't be seen,
        // but GUI mode usually requests elevation via manifest or user action.
        // For GUI app, we might want to show a message box if not elevated, but the current logic just exits.
        // Given the user runs as admin, this is fine.
        eprintln!(
            "{}",
            "ERROR: This program requires administrative privileges."
                .red()
                .bold()
        );
        eprintln!("Please run as administrator.");
        std::process::exit(1);
    }

    if args.len() > 1 {
        // Direct command mode
        let disk_id = args[1].clone();
        toggle_disk(disk_id)?;
    } else {
        // Launch GUI mode
        gui::run_gui()?;
    }

    Ok(())
}

fn toggle_disk(disk_id: String) -> Result<()> {
    println!("Checking disk {disk_id} status...");

    let disks = enumerate_disks()?;
    let disk = disks
        .iter()
        .find(|d| d.id == disk_id)
        .context(format!("Disk {disk_id} not found"))?;

    // Check if it's a critical system disk
    if disk.is_system_disk {
        println!(
            "{}",
            "WARNING: This appears to be a system or boot disk!"
                .red()
                .bold()
        );
        print!("Are you sure you want to continue? (yes/no): ");
        io::stdout().flush()?;

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;

        if !confirm.trim().eq_ignore_ascii_case("yes") {
            println!("Operation cancelled.");
            return Ok(());
        }
    }

    if disk.is_online {
        println!(
            "Disk {} is currently {}. Bringing it {}...",
            disk_id,
            "Online".green(),
            "Offline".red()
        );
        set_disk_offline(disk_id)?;
        println!("{}", "Disk is now Offline.".green());
    } else {
        println!(
            "Disk {} is currently {}. Bringing it {}...",
            disk_id,
            "Offline".red(),
            "Online".green()
        );
        set_disk_online(disk_id)?;
        println!("{}", "Disk is now Online.".green());
    }

    Ok(())
}

#[allow(clippy::borrow_as_ptr)]
#[allow(clippy::ptr_as_ptr)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::ref_as_ptr)]
fn is_elevated() -> bool {
    #[cfg(windows)]
    {
        use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
        use winapi::um::securitybaseapi::GetTokenInformation;
        use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
        unsafe {
            let mut token_handle = ptr::null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
                return false;
            }
            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut return_length = 0u32;
            let result = GetTokenInformation(
                token_handle,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            );
            winapi::um::handleapi::CloseHandle(token_handle);
            if result == 0 {
                return false;
            }
            elevation.TokenIsElevated != 0
        }
    }
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(not(any(windows, unix)))]
    {
        false
    }
}
