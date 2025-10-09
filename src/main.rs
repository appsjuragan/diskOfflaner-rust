use anyhow::{Context, Result};
use colored::*;
use std::env;
use std::io::{self, Write};
use std::mem;
use std::ptr;

mod disk_operations;
mod structs;

use disk_operations::{enumerate_disks, set_disk_online, set_disk_offline};
use structs::{DiskInfo, PartitionInfo};

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
        // Interactive mode
        interactive_mode()?;
    }

    Ok(())
}

fn interactive_mode() -> Result<()> {
    println!("{}", "DiskOfflaner - Disk Management Tool".cyan().bold());
    println!("{}", "====================================".cyan());
    println!();

    let disks = enumerate_disks()?;

    if disks.is_empty() {
        println!("{}", "No disks found.".yellow());
        return Ok(());
    }

    // Display all disks and partitions
    for disk in &disks {
        display_disk_info(disk);
    }

    println!();
    print!("Enter disk number to toggle (or 'q' to quit): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.eq_ignore_ascii_case("q") {
        return Ok(());
    }

    let disk_number: u32 = input.parse().context("Invalid disk number")?;
    toggle_disk(disk_number)?;

    Ok(())
}

fn display_disk_info(disk: &DiskInfo) {
    let status_color = if disk.is_online { "Online".green() } else { "Offline".red() };
    let size_gb = disk.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

    println!(
        "{} {}: {} - {} - Health: {} - Size: {:.2} GB",
        "Disk".bold(),
        disk.disk_number.to_string().yellow().bold(),
        disk.model.cyan(),
        status_color,
        disk.health_status,
        size_gb
    );

    // Display partitions
    for partition in &disk.partitions {
        display_partition_info(partition);
    }
}

fn display_partition_info(partition: &PartitionInfo) {
    let size_gb = partition.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let drive_letter = if !partition.drive_letter.is_empty() {
        format!("[{}:]", partition.drive_letter)
    } else {
        "[_:]".to_string()
    };

    println!(
        "   └─ Partition {}: {:.2} GB ({}: {}) {}",
        partition.partition_number,
        size_gb,
        partition.partition_style,
        partition.partition_id,
        drive_letter.bright_magenta()
    );
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
        println!("Disk {} is currently {}. Bringing it {}...", 
            disk_number, "Online".green(), "Offline".red());
        set_disk_offline(disk_number)?;
        println!("{}", "Disk is now Offline.".green());
    } else {
        println!("Disk {} is currently {}. Bringing it {}...", 
            disk_number, "Offline".red(), "Online".green());
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
