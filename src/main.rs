use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::mem;
use std::ptr;

mod disk_operations;
mod structs;
mod gui;

use disk_operations::{enumerate_disks, set_disk_online, set_disk_offline};


fn main() -> Result<()> {
    // Check for admin privileges
    if !is_elevated() {
        eprintln!("{}", "ERROR: This program requires administrative privileges.".red().bold());
        eprintln!("Please run as administrator.");
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Direct command mode
        let disk_number: u32 = args[1].parse().context("Invalid disk number")?;
        toggle_disk(disk_number)?;
    } else {
        // Launch GUI mode
        gui::run_gui()?;
    }

    Ok(())
}



fn toggle_disk(disk_number: u32) -> Result<()> {
    println!("Checking disk {} status...", disk_number);

    let disks = enumerate_disks()?;
    let disk = disks
        .iter()
        .find(|d| d.disk_number == disk_number)
        .context(format!("Disk {} not found", disk_number))?;

    // Check if it's a critical system disk
    if disk.is_system_disk {
        println!(
            "{}",
            "WARNING: This appears to be a system or boot disk!".red().bold()
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
            disk_number,
            "Online".green(),
            "Offline".red()
        );
        set_disk_offline(disk_number)?;
        println!("{}", "Disk is now Offline.".green());
    } else {
        println!(
            "Disk {} is currently {}. Bringing it {}...",
            disk_number,
            "Offline".red(),
            "Online".green()
        );
        set_disk_online(disk_number)?;
        println!("{}", "Disk is now Online.".green());
    }

    Ok(())
}

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
    #[cfg(not(windows))]
    {
        false
    }
}
