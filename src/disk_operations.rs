use anyhow::Result;
use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::io::Write;
use std::process::{Command, Stdio};
use winapi::um::fileapi::CreateFileW;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winioctl::{
    DISK_GEOMETRY_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX, 
    IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS, VOLUME_DISK_EXTENTS,
};
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};

use crate::structs::{DiskInfo, PartitionInfo};

const OPEN_EXISTING: u32 = 3;

pub fn enumerate_disks() -> Result<Vec<DiskInfo>> {
    let mut disks = Vec::new();

    // Enumerate up to 32 physical disks
    for disk_num in 0..32 {
        if let Ok(disk_info) = get_disk_info(disk_num) {
            disks.push(disk_info);
        }
    }

    Ok(disks)
}

pub fn get_disk_info(disk_number: u32) -> Result<DiskInfo> {
    let path = format!("\\\\.\\PhysicalDrive{}", disk_number);
    let wide_path: Vec<u16> = OsStr::new(&path).encode_wide().chain(once(0)).collect();

    unsafe {
        let handle = CreateFileW(
            wide_path.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            return Err(anyhow::anyhow!("Failed to open disk {}", disk_number));
        }

        // Get disk geometry to determine size
        let size_bytes = get_disk_size(handle)?;
        let is_online = check_disk_online(disk_number);
        let is_system_disk = disk_number == 0; // Simple heuristic
        let partitions = get_partitions(disk_number)?;

        CloseHandle(handle);

        Ok(DiskInfo {
            disk_number,
            model: format!("Disk {}", disk_number),
            size_bytes,
            is_online,
            health_status: 0,
            is_system_disk,
            partitions,
        })
    }
}

fn get_disk_size(handle: *mut winapi::ctypes::c_void) -> Result<u64> {
    unsafe {
        let mut geometry: DISK_GEOMETRY_EX = mem::zeroed();
        let mut bytes_returned = 0u32;

        let success = DeviceIoControl(
            handle,
            IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
            std::ptr::null_mut(),
            0,
            &mut geometry as *mut _ as *mut _,
            mem::size_of::<DISK_GEOMETRY_EX>() as u32,
            &mut bytes_returned,
            std::ptr::null_mut(),
        );

        if success != 0 {
            Ok(*geometry.DiskSize.QuadPart() as u64)
        } else {
            // Fallback to default size
            Ok(10 * 1024 * 1024 * 1024) // Default 10GB
        }
    }
}

fn check_disk_online(disk_number: u32) -> bool {
    // Use diskpart to check actual online/offline status
    let script = format!("list disk\nexit\n");

    let output = match Command::new("diskpart")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(script.as_bytes());
            }
            match child.wait_with_output() {
                Ok(out) => out,
                Err(_) => return true, // Default to online if we can't check
            }
        }
        Err(_) => return true, // Default to online if diskpart fails
    };

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Parse diskpart output
    // Format: "  Disk ###  Status         Size     Free     Dyn  Gpt"
    // Example: "  Disk 2    Offline        11176 GB      0 B        *"
    for line in output_str.lines() {
        let line = line.trim();
        if line.starts_with("Disk") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // parts[0] = "Disk", parts[1] = disk number, parts[2] = status
                if let Ok(disk_num) = parts[1].parse::<u32>() {
                    if disk_num == disk_number {
                        let status = parts[2].to_lowercase();
                        return status == "online";
                    }
                }
            }
        }
    }

    true // Default to online if not found
}

fn get_partitions(disk_number: u32) -> Result<Vec<PartitionInfo>> {
    let mut partitions = Vec::new();

    // Enumerate drive letters A-Z
    for letter in b'A'..=b'Z' {
        let drive_letter = (letter as char).to_string();
        let volume_path = format!("\\\\.\\{}:", drive_letter);
        
        if let Ok(partition) = get_partition_on_disk(&volume_path, disk_number, &drive_letter) {
            partitions.push(partition);
        }
    }

    // Sort by starting offset (partition_id stores it as hex)
    partitions.sort_by(|a, b| {
        let offset_a = u64::from_str_radix(&a.partition_id, 16).unwrap_or(0);
        let offset_b = u64::from_str_radix(&b.partition_id, 16).unwrap_or(0);
        offset_a.cmp(&offset_b)
    });

    // Assign incremental partition numbers
    for (i, part) in partitions.iter_mut().enumerate() {
        part.partition_number = (i + 1) as u32;
    }

    Ok(partitions)
}

fn get_partition_on_disk(
    volume_path: &str,
    expected_disk: u32,
    drive_letter: &str,
) -> Result<PartitionInfo> {
    let wide_path: Vec<u16> = OsStr::new(volume_path)
        .encode_wide()
        .chain(once(0))
        .collect();

    unsafe {
        let handle = CreateFileW(
            wide_path.as_ptr(),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            return Err(anyhow::anyhow!("Cannot open volume"));
        }

        let mut extents: VOLUME_DISK_EXTENTS = mem::zeroed();
        let mut bytes_returned = 0u32;

        let success = DeviceIoControl(
            handle,
            IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
            std::ptr::null_mut(),
            0,
            &mut extents as *mut _ as *mut _,
            mem::size_of::<VOLUME_DISK_EXTENTS>() as u32,
            &mut bytes_returned,
            std::ptr::null_mut(),
        );

        CloseHandle(handle);

        if success == 0 || extents.NumberOfDiskExtents == 0 {
            return Err(anyhow::anyhow!("No extents found"));
        }

        let extent = &extents.Extents[0];
        if extent.DiskNumber != expected_disk {
            return Err(anyhow::anyhow!("Wrong disk"));
        }

        Ok(PartitionInfo {
            partition_number: 0,
            size_bytes: *extent.ExtentLength.QuadPart() as u64,
            drive_letter: drive_letter.to_string(),
            partition_style: "GPT".to_string(),
            partition_id: format!("{:X}", extent.StartingOffset.QuadPart()),
        })
    }
}

pub fn set_disk_online(disk_number: u32) -> Result<()> {
    execute_disk_command(disk_number, "online")
}

pub fn set_disk_offline(disk_number: u32) -> Result<()> {
    execute_disk_command(disk_number, "offline")
}

fn execute_disk_command(disk_number: u32, command: &str) -> Result<()> {
    let script = format!(
        "select disk {}\n{} disk\nexit\n",
        disk_number, command
    );

    let mut child = Command::new("diskpart")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(script.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() || stdout.contains("Virtual Disk Service error") || stdout.contains("The disk is currently in use") {
        return Err(anyhow::anyhow!(
            "Diskpart failed: {}\n{}",
            stdout,
            stderr
        ));
    }

    Ok(())
}
