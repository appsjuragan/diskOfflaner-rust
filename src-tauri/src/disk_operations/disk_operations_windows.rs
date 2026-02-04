#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::borrow_as_ptr)]
#![allow(clippy::ptr_as_ptr)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::ref_as_ptr)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::assigning_clones)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::cast_ptr_alignment)]
#![allow(clippy::cast_possible_wrap)]

use anyhow::Result;
use std::ffi::OsStr;
use std::io::Write;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::process::{Command, Stdio};
use winapi::um::fileapi::CreateFileW;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
// src/disk_operations/disk_operations_windows.rs
use winapi::um::ioapiset::DeviceIoControl;
use winapi::um::winioctl::{
    DISK_GEOMETRY_EX, DRIVE_LAYOUT_INFORMATION_EX, IOCTL_DISK_GET_DRIVE_GEOMETRY_EX,
    IOCTL_DISK_GET_DRIVE_LAYOUT_EX, IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS, VOLUME_DISK_EXTENTS,
};
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};
// Removed unused PARTITION_INFORMATION_EX import

use crate::structs::{DiskInfo, DiskType, PartitionInfo};

const OPEN_EXISTING: u32 = 3;

// Definitions for Storage Queries
const IOCTL_STORAGE_QUERY_PROPERTY: u32 = 0x002D_1400;
const STORAGE_DEVICE_PROPERTY: u32 = 0;
const PROPERTY_STANDARD_QUERY: u32 = 0;
const STORAGE_DEVICE_SEEK_PENALTY_PROPERTY: u32 = 7;

#[repr(C)]
#[allow(non_snake_case)]
struct STORAGE_PROPERTY_QUERY {
    PropertyId: u32,
    QueryType: u32,
    AdditionalParameters: [u8; 1],
}

#[repr(C)]
#[allow(non_snake_case)]
struct STORAGE_DEVICE_DESCRIPTOR {
    Version: u32,
    Size: u32,
    DeviceType: u8,
    DeviceTypeModifier: u8,
    RemovableMedia: u8,
    CommandQueueing: u8,
    VendorIdOffset: u32,
    ProductIdOffset: u32,
    ProductRevisionOffset: u32,
    SerialNumberOffset: u32,
    BusType: u32,
    RawPropertiesLength: u32,
    RawDeviceProperties: [u8; 1],
}

#[repr(C)]
#[allow(non_snake_case)]
struct STORAGE_DESCRIPTOR_HEADER {
    Version: u32,
    Size: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
struct DEVICE_SEEK_PENALTY_DESCRIPTOR {
    Version: u32,
    Size: u32,
    IncursSeekPenalty: u8,
}

// Bus type constants for STORAGE_DEVICE_DESCRIPTOR.BusType
const BUS_TYPE_USB: u32 = 7;
const BUS_TYPE_NVME: u32 = 17;

// Maximum number of physical disks to enumerate
const MAX_DISK_COUNT: u32 = 32;

pub fn enumerate_disks() -> Result<Vec<DiskInfo>> {
    // If running as admin, use the robust low-level method
    if crate::utils::is_elevated() {
        let mut disks = Vec::new();

        // Get online status for ALL disks in a single diskpart call (OPTIMIZATION)
        let disk_status_map = check_all_disks_online();

        // Enumerate physical disks
        for disk_num in 0..MAX_DISK_COUNT {
            if let Ok(disk_info) = get_disk_info_with_status(disk_num, &disk_status_map) {
                disks.push(disk_info);
            }
        }
        return Ok(disks);
    }

    // Fallback for non-admin: Use PowerShell (Get-CimInstance)
    enumerate_disks_powershell()
}

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct Win32DiskDrive {
    Index: u32,
    Model: Option<String>,
    Size: Option<u64>,
    MediaType: Option<String>,
    InterfaceType: Option<String>,
    SerialNumber: Option<String>,
    Status: Option<String>,
}

fn enumerate_disks_powershell() -> Result<Vec<DiskInfo>> {
    let mut disks = Vec::new();

    // Get-CimInstance Win32_DiskDrive | Select-Object Index,Model,Size,MediaType,InterfaceType,SerialNumber,Status | ConvertTo-Json
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_DiskDrive | Select-Object Index,Model,Size,MediaType,InterfaceType,SerialNumber,Status | ConvertTo-Json"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        return Ok(vec![]);
    }

    // Handle single object or array
    let drives: Vec<Win32DiskDrive> = if stdout.trim().starts_with('[') {
        serde_json::from_str(&stdout).unwrap_or_default()
    } else {
        match serde_json::from_str::<Win32DiskDrive>(&stdout) {
            Ok(d) => vec![d],
            Err(_) => vec![],
        }
    };

    for drive in drives {
        let id = drive.Index.to_string();
        let model = drive.Model.unwrap_or_else(|| format!("Disk {}", id));
        let size_bytes = drive.Size.unwrap_or(0);
        let interface_type = drive.InterfaceType.unwrap_or_default();
        let media_type = drive.MediaType.unwrap_or_default();
        let serial = drive.SerialNumber;
        let status = drive.Status.unwrap_or_default(); // "OK"

        // Guess type
        let disk_type = if interface_type.contains("USB") || media_type.contains("External") {
            DiskType::USBFlash
        } else if interface_type.contains("NVMe") {
            DiskType::NVMe
        } else if media_type.to_lowercase().contains("ssd") {
            DiskType::SSD
        } else {
            DiskType::HDD
        };

        disks.push(DiskInfo {
            id: id.clone(),
            model,
            size_bytes,
            is_online: true,
            is_system_disk: id == "0",
            partitions: vec![],
            disk_type,
            serial_number: serial,
            health_percentage: if status == "OK" { Some(100) } else { Some(0) },
        });
    }

    Ok(disks)
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

// OPTIMIZED: Call diskpart ONCE for ALL disks and return a HashMap of statuses
fn check_all_disks_online() -> std::collections::HashMap<u32, bool> {
    let mut status_map = std::collections::HashMap::new();

    let script = "list disk\nexit\n".to_string();

    let output = match Command::new("diskpart")
        .creation_flags(CREATE_NO_WINDOW)
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
                Err(_) => return status_map, // Return empty on error
            }
        }
        Err(_) => return status_map, // Return empty on error
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
                    let status = parts[2].to_lowercase();
                    status_map.insert(disk_num, status == "online");
                }
            }
        }
    }

    status_map
}

// Modified version of get_disk_info that uses cached status
fn get_disk_info_with_status(
    disk_number: u32,
    status_map: &std::collections::HashMap<u32, bool>,
) -> Result<DiskInfo> {
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

        // Use cached online status (OPTIMIZATION: no diskpart call per disk)
        let is_online = status_map.get(&disk_number).copied().unwrap_or(true);

        let partitions = get_partitions(disk_number)?;

        // Determine system disk number accurately
        let system_disk_number = get_system_disk_number();
        let is_system_disk = if let Some(sys_num) = system_disk_number {
            disk_number == sys_num
        } else {
            // Fallback to drive letter check
            let system_drive_letter = std::env::var("SystemDrive")
                .ok()
                .and_then(|s| s.chars().next())
                .map(|c| c.to_ascii_uppercase().to_string());
            if let Some(sys) = system_drive_letter {
                partitions
                    .iter()
                    .any(|p| p.drive_letter.eq_ignore_ascii_case(&sys))
            } else {
                disk_number == 0
            }
        };

        // Determine disk type before moving partitions
        let disk_type = get_disk_type(disk_number, &partitions);
        let serial_number = get_disk_serial(handle);
        let health_percentage = get_disk_health(handle);

        CloseHandle(handle);

        Ok(DiskInfo {
            id: disk_number.to_string(),
            model: format!("Disk {}", disk_number),
            size_bytes,
            is_online,
            is_system_disk,
            partitions,
            disk_type,
            serial_number,
            health_percentage,
        })
    }
}

const IOCTL_STORAGE_PREDICT_FAILURE: u32 = 0x002D_1140;

#[repr(C)]
#[allow(non_snake_case)]
struct STORAGE_PREDICT_FAILURE {
    PredictFailure: u32,
    VendorSpecific: [u8; 512],
}

unsafe fn get_disk_health(handle: *mut winapi::ctypes::c_void) -> Option<u8> {
    let mut predict_failure: STORAGE_PREDICT_FAILURE = mem::zeroed();
    let mut bytes_returned = 0u32;

    let success = DeviceIoControl(
        handle,
        IOCTL_STORAGE_PREDICT_FAILURE,
        std::ptr::null_mut(),
        0,
        &mut predict_failure as *mut _ as *mut _,
        mem::size_of::<STORAGE_PREDICT_FAILURE>() as u32,
        &mut bytes_returned,
        std::ptr::null_mut(),
    );

    if success != 0 {
        // If PredictFailure is 0, disk is healthy (100%).
        // If non-zero, disk is predicting failure (0% or low).
        return Some(if predict_failure.PredictFailure == 0 {
            100
        } else {
            10
        });
    }

    // If the IOCTL fails, it might not support SMART or it's a USB drive.
    // Try to be conservative and return None (Unknown).
    None
}

// Extract serial number from STORAGE_DEVICE_DESCRIPTOR
unsafe fn get_disk_serial(handle: *mut winapi::ctypes::c_void) -> Option<String> {
    // We need to re-query for the full descriptor because the previous query (in get_disk_type)
    // used a fixed size struct which might trigger buffer overflow if we try to read past it,
    // or simply we need to do it here since handle is available.
    // Actually, get_disk_type opens its own handle. Here we have a handle already.

    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: STORAGE_DEVICE_PROPERTY,
        QueryType: PROPERTY_STANDARD_QUERY,
        AdditionalParameters: [0; 1],
    };

    // First call to get size
    let mut header: STORAGE_DESCRIPTOR_HEADER = mem::zeroed();
    let mut bytes_returned = 0u32;

    let success = DeviceIoControl(
        handle,
        IOCTL_STORAGE_QUERY_PROPERTY,
        &mut query as *mut _ as *mut _,
        mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
        &mut header as *mut _ as *mut _,
        mem::size_of::<STORAGE_DESCRIPTOR_HEADER>() as u32,
        &mut bytes_returned,
        std::ptr::null_mut(),
    );

    if success == 0 {
        return None;
    }

    // Allocate full buffer
    let mut buffer = vec![0u8; header.Size as usize];
    let success = DeviceIoControl(
        handle,
        IOCTL_STORAGE_QUERY_PROPERTY,
        &mut query as *mut _ as *mut _,
        mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
        buffer.as_mut_ptr() as *mut _,
        buffer.len() as u32,
        &mut bytes_returned,
        std::ptr::null_mut(),
    );

    if success == 0 {
        return None;
    }

    let descriptor = &*(buffer.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR);

    if descriptor.SerialNumberOffset != 0 {
        let serial_cstr = std::ffi::CStr::from_ptr(
            buffer
                .as_ptr()
                .offset(descriptor.SerialNumberOffset as isize) as *const i8,
        );
        if let Ok(serial) = serial_cstr.to_str() {
            return Some(serial.trim().to_string());
        }
    }

    None
}

// Helper to get all partitions using Drive Layout
fn get_partitions_layout(disk_number: u32) -> Result<Vec<PartitionInfo>> {
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
            return Err(anyhow::anyhow!("Failed to open disk for layout"));
        }

        // Allocate a large buffer for layout info (supports many partitions)
        let mut buffer = vec![0u8; 4096];
        let mut bytes_returned = 0u32;

        let success = DeviceIoControl(
            handle,
            IOCTL_DISK_GET_DRIVE_LAYOUT_EX,
            std::ptr::null_mut(),
            0,
            buffer.as_mut_ptr() as *mut _,
            buffer.len() as u32,
            &mut bytes_returned,
            std::ptr::null_mut(),
        );

        CloseHandle(handle);

        if success == 0 {
            return Err(anyhow::anyhow!("Failed to get drive layout"));
        }

        let layout = &*(buffer.as_ptr() as *const DRIVE_LAYOUT_INFORMATION_EX);
        let mut partitions = Vec::new();

        // Iterate over partitions
        // The PARTITION_INFORMATION_EX array is at the end of the struct.
        // We need to handle the variable length array manually or use the slice if winapi supports it.
        // winapi DRIVE_LAYOUT_INFORMATION_EX has PartitionEntry: [PARTITION_INFORMATION_EX; 1]

        let entry_ptr = layout.PartitionEntry.as_ptr();
        for i in 0..layout.PartitionCount {
            let entry = &*entry_ptr.offset(i as isize);

            // Filter out empty/unusable partitions
            // PartitionStyle: 0 = MBR, 1 = GPT, 2 = RAW
            // We generally want partitions with non-zero length
            let length = *entry.PartitionLength.QuadPart() as u64;
            if length > 0 {
                let offset = *entry.StartingOffset.QuadPart() as u64;
                let number = entry.PartitionNumber;

                // For MBR, check RecognizedPartition (bool) or PartitionType
                // For GPT, check PartitionType (GUID)
                // Simplified: just take it if length > 0

                partitions.push(PartitionInfo {
                    partition_number: number,
                    size_bytes: length,
                    drive_letter: String::new(),           // Filled later
                    partition_id: format!("{:X}", offset), // Use Offset as ID for matching
                });
            }
        }

        Ok(partitions)
    }
}

fn get_partitions(disk_number: u32) -> Result<Vec<PartitionInfo>> {
    // 1. Get all partitions from layout
    let mut partitions = get_partitions_layout(disk_number).unwrap_or_default();

    // 2. Get mounted volumes (Drive Letters) on this disk
    // OPTIMIZATION: Only check MOUNTED drives using GetLogicalDrives
    let mut mounted_map = std::collections::HashMap::new();

    unsafe {
        let drives_bitmask = winapi::um::fileapi::GetLogicalDrives();

        for i in 0..26 {
            // Check if this drive letter is mounted
            if (drives_bitmask & (1 << i)) != 0 {
                let drive_letter = ((b'A' + i) as char).to_string();
                let volume_path = format!("\\\\.\\{}:", drive_letter);
                if let Ok(info) = get_partition_on_disk(&volume_path, disk_number, &drive_letter) {
                    // info.partition_id holds the offset in Hex
                    if let Ok(offset) = u64::from_str_radix(&info.partition_id, 16) {
                        mounted_map.insert(offset, drive_letter);
                    }
                }
            }
        }
    }

    // 3. Merge Drive Letters
    for part in &mut partitions {
        if let Ok(offset) = u64::from_str_radix(&part.partition_id, 16) {
            if let Some(letter) = mounted_map.get(&offset) {
                part.drive_letter = letter.clone();
            }
        }
    }

    // If layout failed (empty partitions), fallback to old method (only mounted)
    if partitions.is_empty() {
        unsafe {
            let drives_bitmask = winapi::um::fileapi::GetLogicalDrives();

            for i in 0..26 {
                if (drives_bitmask & (1 << i)) != 0 {
                    let drive_letter = ((b'A' + i) as char).to_string();
                    let volume_path = format!("\\\\.\\{}:", drive_letter);
                    if let Ok(partition) =
                        get_partition_on_disk(&volume_path, disk_number, &drive_letter)
                    {
                        partitions.push(partition);
                    }
                }
            }
        }
    }

    // Sort by partition number
    partitions.sort_by_key(|p| p.partition_number);

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
            partition_id: format!("{:X}", extent.StartingOffset.QuadPart()),
        })
    }
}

pub fn unmount_partition(drive_letter: String) -> Result<()> {
    let script = format!("select volume {}\nremove\nexit\n", drive_letter);
    run_diskpart_script(&script)
}

pub fn mount_partition(disk_id: String, partition_number: u32, letter: Option<char>) -> Result<()> {
    let disk_number = disk_id.parse::<u32>()?;
    let assign_cmd = if let Some(l) = letter {
        format!("assign letter={}", l)
    } else {
        "assign".to_string()
    };

    let script = format!(
        "select disk {}\nselect partition {}\n{}\nexit\n",
        disk_number, partition_number, assign_cmd
    );
    run_diskpart_script(&script)
}

pub fn get_available_drive_letters() -> Vec<String> {
    let mut available = Vec::new();
    unsafe {
        let drives_bitmask = winapi::um::fileapi::GetLogicalDrives();
        // Skip A and B usually, but let's just list what's free from C to Z?
        // Actually A and B are valid if free.
        for i in 0..26 {
            if (drives_bitmask & (1 << i)) == 0 {
                let letter = (b'A' + i) as char;
                available.push(letter.to_string());
            }
        }
    }
    available
}

fn run_diskpart_script(script: &str) -> Result<()> {
    let mut child = Command::new("diskpart")
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(script.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Diskpart failed: {}\n{}", stdout, stderr));
    }
    Ok(())
}

pub fn set_disk_online(disk_id: String) -> Result<()> {
    let disk_number = disk_id.parse::<u32>()?;
    execute_disk_command(disk_number, "online")
}

pub fn set_disk_offline(disk_id: String) -> Result<()> {
    let disk_number = disk_id.parse::<u32>()?;
    execute_disk_command(disk_number, "offline")
}

#[allow(dead_code)]
pub fn eject_disk(disk_id: String) -> Result<()> {
    // Windows eject implementation (currently unused)
    let _disk_number = disk_id.parse::<u32>()?;
    // Placeholder: actual eject logic would go here
    Ok(())
}

// Determine disk type based on physical drive properties
fn get_disk_type(disk_number: u32, _partitions: &Vec<PartitionInfo>) -> DiskType {
    // Windows detection uses bus type and removable flag.
    // The partitions argument is unused for now.
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
            return DiskType::Unknown;
        }
        let mut descriptor: STORAGE_DEVICE_DESCRIPTOR = mem::zeroed();
        let mut query = STORAGE_PROPERTY_QUERY {
            PropertyId: STORAGE_DEVICE_PROPERTY,
            QueryType: PROPERTY_STANDARD_QUERY,
            AdditionalParameters: [0; 1],
        };
        let mut bytes_returned = 0u32;
        let success = DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            &mut query as *mut _ as *mut _,
            mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            &mut descriptor as *mut _ as *mut _,
            mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>() as u32,
            &mut bytes_returned,
            std::ptr::null_mut(),
        );
        if success != 0 {
            match descriptor.BusType {
                BUS_TYPE_NVME => {
                    CloseHandle(handle);
                    return DiskType::NVMe;
                }
                BUS_TYPE_USB => {
                    if descriptor.RemovableMedia == 1 {
                        CloseHandle(handle);
                        return DiskType::USBFlash;
                    }
                    CloseHandle(handle);
                    return DiskType::ExtHDD;
                }
                _ => {}
            }
        }

        // If we are here, it's likely HDD or SSD (SATA/SAS).
        // Try to check for seek penalty to distinguish HDD vs SSD.
        let mut seek_penalty: DEVICE_SEEK_PENALTY_DESCRIPTOR = mem::zeroed();
        let mut query = STORAGE_PROPERTY_QUERY {
            PropertyId: STORAGE_DEVICE_SEEK_PENALTY_PROPERTY,
            QueryType: PROPERTY_STANDARD_QUERY,
            AdditionalParameters: [0; 1],
        };
        let mut bytes_returned = 0u32;
        let success = DeviceIoControl(
            handle,
            IOCTL_STORAGE_QUERY_PROPERTY,
            &mut query as *mut _ as *mut _,
            mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            &mut seek_penalty as *mut _ as *mut _,
            mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
            &mut bytes_returned,
            std::ptr::null_mut(),
        );
        CloseHandle(handle);

        if success != 0 {
            if seek_penalty.IncursSeekPenalty == 0 {
                return DiskType::SSD;
            }
            return DiskType::HDD;
        }
    }

    DiskType::HDD
}

use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

fn execute_disk_command(disk_number: u32, command: &str) -> Result<()> {
    let script = format!("select disk {}\n{} disk\nexit\n", disk_number, command);

    let mut child = Command::new("diskpart")
        .creation_flags(CREATE_NO_WINDOW)
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

    if !output.status.success()
        || stdout.contains("Virtual Disk Service error")
        || stdout.contains("The disk is currently in use")
    {
        return Err(anyhow::anyhow!("Diskpart failed: {}\n{}", stdout, stderr));
    }

    Ok(())
}

fn get_system_disk_number() -> Option<u32> {
    use winapi::um::fileapi::GetVolumePathNameW;
    use winapi::um::sysinfoapi::GetSystemDirectoryW;

    unsafe {
        let mut system_dir = [0u16; 260];
        let len = GetSystemDirectoryW(system_dir.as_mut_ptr(), 260);
        if len == 0 {
            return None;
        }

        let mut volume_path = [0u16; 260];
        if GetVolumePathNameW(system_dir.as_ptr(), volume_path.as_mut_ptr(), 260) == 0 {
            return None;
        }

        // volume_path is like "C:\"
        let mut wide_volume: Vec<u16> = volume_path
            .iter()
            .take_while(|&&c| c != 0)
            .cloned()
            .collect();
        // Remove trailing backslash for CreateFile
        if wide_volume.last() == Some(&(b'\\' as u16)) {
            wide_volume.pop();
        }

        // Add \\.\ prefix
        let mut full_path = "\\\\.\\".encode_utf16().collect::<Vec<u16>>();
        full_path.extend(wide_volume);
        full_path.push(0);

        let handle = CreateFileW(
            full_path.as_ptr(),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            return None;
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

        if success != 0 && extents.NumberOfDiskExtents > 0 {
            return Some(extents.Extents[0].DiskNumber);
        }
    }
    None
}
