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

use crate::structs::{DiskInfo, DiskType, PartitionInfo, SystemInfo};

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

        // PARALELLIZE: Run diskpart and powershell check in parallel
        let status_handle = std::thread::spawn(check_all_disks_online);
        let health_model_handle = std::thread::spawn(check_all_disks_health_and_model);

        let disk_status_map = status_handle.join().unwrap_or_default();
        let (disk_health_map, disk_model_map) = health_model_handle.join().unwrap_or_default();

        // Enumerate physical disks
        for disk_num in 0..MAX_DISK_COUNT {
            if let Ok(disk_info) = get_disk_info_with_status(disk_num, &disk_status_map, &disk_health_map, &disk_model_map) {
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

    // Fetch disk drives from Win32_DiskDrive (model, serial, size)
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

    // Fetch Get-Disk for BusType and IsOffline (accurate type and status detection)
    let disk_status_map = get_disk_status_powershell();

    // Fetch partitions via PowerShell (works without admin)
    let partitions_map = get_partitions_powershell();

    for drive in drives {
        let id = drive.Index.to_string();
        let model_str = drive.Model.clone().unwrap_or_else(|| format!("Disk {}", id));
        let size_bytes = drive.Size.unwrap_or(0);
        let serial = drive.SerialNumber;
        let status = drive.Status.unwrap_or_default();

        // Use Get-Disk data if available, otherwise fallback to heuristics
        let (is_online, disk_type) = if let Some((offline, bus_type)) = disk_status_map.get(&drive.Index) {
            let dtype = match bus_type.as_str() {
                "NVMe" => DiskType::NVMe,
                "USB" => DiskType::USBFlash,
                "SATA" | "ATA" => {
                    // Check model for SSD hints
                    if model_str.to_lowercase().contains("ssd") {
                        DiskType::SSD
                    } else {
                        DiskType::HDD
                    }
                }
                _ => {
                    // Fallback: check model name
                    if model_str.to_lowercase().contains("nvme") {
                        DiskType::NVMe
                    } else if model_str.to_lowercase().contains("ssd") {
                        DiskType::SSD
                    } else {
                        DiskType::HDD
                    }
                }
            };
            (!*offline, dtype)
        } else {
            // Fallback if Get-Disk failed for this disk
            let interface_type = drive.InterfaceType.unwrap_or_default();
            let media_type = drive.MediaType.unwrap_or_default();
            let dtype = if interface_type.contains("USB") || media_type.contains("External") {
                DiskType::USBFlash
            } else if model_str.to_lowercase().contains("nvme") {
                DiskType::NVMe
            } else if model_str.to_lowercase().contains("ssd") {
                DiskType::SSD
            } else {
                DiskType::HDD
            };
            (true, dtype) // Assume online if we can't determine
        };

        // Get partitions for this disk
        let partitions = partitions_map.get(&drive.Index).cloned().unwrap_or_default();

        disks.push(DiskInfo {
            id: id.clone(),
            model: model_str,
            size_bytes,
            is_online,
            is_system_disk: id == "0",
            partitions,
            disk_type,
            serial_number: serial,
            health_percentage: if status == "OK" { Some(100) } else { Some(0) },
        });
    }

    Ok(disks)
}

// Fetch disk status (IsOffline, BusType) via Get-Disk cmdlet
#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct PsDiskStatus {
    Number: u32,
    IsOffline: bool,
    BusType: Option<String>,
}

fn get_disk_status_powershell() -> std::collections::HashMap<u32, (bool, String)> {
    let mut result = std::collections::HashMap::new();

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-Disk | Select-Object Number, IsOffline, BusType | ConvertTo-Json"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let statuses: Vec<PsDiskStatus> = match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.trim().starts_with('[') {
                serde_json::from_str(&stdout).unwrap_or_default()
            } else {
                match serde_json::from_str::<PsDiskStatus>(&stdout) {
                    Ok(s) => vec![s],
                    Err(_) => vec![],
                }
            }
        }
        _ => vec![],
    };

    for s in statuses {
        result.insert(s.Number, (s.IsOffline, s.BusType.unwrap_or_default()));
    }

    result
}

// PowerShell-based partition enumeration using Get-Partition (more robust for drive letters)
#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct PsPartition {
    DiskNumber: u32,
    PartitionNumber: u32,
    DriveLetter: Option<char>, // "C", "D", or null
    Size: Option<u64>,
}

fn get_partitions_powershell() -> std::collections::HashMap<u32, Vec<PartitionInfo>> {
    let mut result: std::collections::HashMap<u32, Vec<PartitionInfo>> = std::collections::HashMap::new();

    // Get-Partition | Select-Object DiskNumber, PartitionNumber, DriveLetter, Size | ConvertTo-Json
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-Partition | Select-Object DiskNumber, PartitionNumber, DriveLetter, Size | ConvertTo-Json"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    let ps_partitions: Vec<PsPartition> = match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if stdout.trim().starts_with('[') {
                serde_json::from_str(&stdout).unwrap_or_default()
            } else {
                match serde_json::from_str::<PsPartition>(&stdout) {
                    Ok(p) => vec![p],
                    Err(_) => vec![],
                }
            }
        }
        _ => vec![],
    };

    for p in ps_partitions {
        let drive_letter = match p.DriveLetter {
            Some(c) if c != '\0' && c != ' ' => c.to_string(),
            _ => String::new(),
        };

        let partition_info = PartitionInfo {
            partition_number: p.PartitionNumber,
            size_bytes: p.Size.unwrap_or(0),
            drive_letter,
            partition_id: format!("{}:{}", p.DiskNumber, p.PartitionNumber), // Use unique ID for key
        };
        result.entry(p.DiskNumber).or_insert_with(Vec::new).push(partition_info);
    }

    result
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

#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct PsPhysicalDisk {
    DeviceId: Option<String>,
    HealthStatus: Option<String>,
    FriendlyName: Option<String>,
}

fn check_all_disks_health_and_model() -> (std::collections::HashMap<u32, u8>, std::collections::HashMap<u32, String>) {
    let mut health_map = std::collections::HashMap::new();
    let mut model_map = std::collections::HashMap::new();

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "Get-PhysicalDisk | Select-Object DeviceId, HealthStatus, FriendlyName | ConvertTo-Json"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if !stdout.trim().is_empty() {
                 let disks: Vec<PsPhysicalDisk> = if stdout.trim().starts_with('[') {
                     serde_json::from_str(&stdout).unwrap_or_default()
                 } else {
                     serde_json::from_str::<PsPhysicalDisk>(&stdout).map(|d| vec![d]).unwrap_or_default()
                 };

                 for d in disks {
                     if let Some(id_str) = d.DeviceId {
                         if let Ok(num) = id_str.parse::<u32>() {
                             // Health
                             if let Some(status) = d.HealthStatus {
                                 let score = match status.as_str() {
                                     "Healthy" => 100,
                                     "Warning" => 70,
                                     "Unhealthy" => 20,
                                     _ => 100,
                                 };
                                 health_map.insert(num, score);
                             }
                             // Model
                             if let Some(name) = d.FriendlyName {
                                 model_map.insert(num, name);
                             }
                         }
                     }
                 }
            }
        }
    }
    (health_map, model_map)
}

// Modified version of get_disk_info that uses cached status
fn get_disk_info_with_status(
    disk_number: u32,
    status_map: &std::collections::HashMap<u32, bool>,
    health_map: &std::collections::HashMap<u32, u8>,
    model_map: &std::collections::HashMap<u32, String>,
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
        
        // Try IOCTL first, then PowerShell fallback (using cached map)
        let health_percentage = get_disk_health(handle)
            .or_else(|| health_map.get(&disk_number).copied());

        CloseHandle(handle);

        let ioctl_model = get_disk_model(handle);
        let model = if ioctl_model == "Disk" {
             model_map.get(&disk_number).cloned().unwrap_or(format!("Disk {}", disk_number))
        } else {
             ioctl_model
        };

        Ok(DiskInfo {
            id: disk_number.to_string(),
            model,
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

// Primary health detection using IOCTL
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
        // Check PredictFailure flag first
        if predict_failure.PredictFailure != 0 {
            return Some(10); // Disk is predicting failure
        }
        
        // Parse SMART attributes from VendorSpecific data
        // Format: 2 bytes header + 30 attributes * 12 bytes each
        let data = &predict_failure.VendorSpecific;
        
        // Track health indicators
        let mut worst_health: Option<u8> = None;
        
        for i in 0..30 {
            let offset = 2 + (i * 12);
            if offset + 12 > data.len() {
                break;
            }
            
            let attr_id = data[offset];
            if attr_id == 0 {
                continue; // Empty attribute slot
            }
            
            let current = data[offset + 3];
            let _worst = data[offset + 4];
            
            // Key health attributes (normalized value, higher is better)
            match attr_id {
                // SSD Life indicators
                0xE7 | // SSD Life Left (Samsung, SanDisk)
                0xE9 | // Media Wearout Indicator (Intel)
                0xAD | // Erase Count (Micron)
                0xB1 | // Wear Range Delta
                0xCA | // Percentage Used (some SSDs)
                0xF1 | // Total LBAs Written
                0x05   // Reallocated Sector Count
                => {
                    if current > 0 && current <= 100 {
                        worst_health = Some(match worst_health {
                            Some(h) => h.min(current),
                            None => current,
                        });
                    }
                }
                _ => {}
            }
        }
        
        // If we found health attributes, return the worst one
        if let Some(h) = worst_health {
            return Some(h);
        }
        
        // No specific attributes found, but SMART passed
        return Some(100);
    }

    None
}



// Extract model name from STORAGE_DEVICE_DESCRIPTOR
unsafe fn get_disk_model(handle: *mut winapi::ctypes::c_void) -> String {
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: STORAGE_DEVICE_PROPERTY,
        QueryType: PROPERTY_STANDARD_QUERY,
        AdditionalParameters: [0],
    };

    let mut buffer = [0u8; 1024];
    let mut bytes_returned = 0u32;

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
        return format!("Disk"); // Fallback if IOCTL fails (will append ID later if needed, but usually won't match)
    }

    let descriptor = &*(buffer.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR);
    
    let get_string = |offset: u32| -> String {
        if offset == 0 || offset as usize >= buffer.len() {
            return String::new();
        }
        let start = offset as usize;
        let mut end = start;
        while end < buffer.len() && buffer[end] != 0 {
            end += 1;
        }
        String::from_utf8_lossy(&buffer[start..end]).trim().to_string()
    };

    let vendor = get_string(descriptor.VendorIdOffset);
    let product = get_string(descriptor.ProductIdOffset);
    let _revision = get_string(descriptor.ProductRevisionOffset);

    let full_name = if vendor.is_empty() {
        product
    } else {
        format!("{} {}", vendor, product)
    };

    if full_name.trim().is_empty() {
        format!("Disk")
    } else {
        full_name.trim().to_string()
    }
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
    let _ = run_diskpart_script_output(&script)?;
    Ok(())
}

pub fn mount_partition(disk_id: String, partition_number: u32, letter: Option<char>) -> Result<Option<char>> {
    let disk_number = disk_id.parse::<u32>()?;
    let assign_cmd = if let Some(l) = letter {
        format!("assign letter={}", l)
    } else {
        "assign".to_string()
    };

    let script = format!(
        "select disk {}\nselect partition {}\n{}\ndetail partition\nexit\n",
        disk_number, partition_number, assign_cmd
    );
    
    let output = run_diskpart_script_output(&script)?;
    
    // Parse the output to find the assigned letter
    // detail partition shows a Volume table: "  Volume 2    G    Label ..."
    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("* Volume") || (trimmed.starts_with("Volume") && line.contains("Partition")) {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            // If it starts with *, parts[0] is *, parts[2] is the letter
            // If it doesn't, parts[0] is Volume, parts[2] is the letter
            let ltr_idx = if trimmed.starts_with("*") { 2 } else { 2 };
            if parts.len() > ltr_idx {
                let ltr = parts[ltr_idx];
                if ltr.len() == 1 && ltr.chars().next().unwrap().is_alphabetic() {
                    return Ok(Some(ltr.chars().next().unwrap()));
                }
            }
        }
    }
    
    Ok(letter)
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

fn run_diskpart_script_output(script: &str) -> Result<String> {
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
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Diskpart failed: {}\n{}", stdout, stderr));
    }
    Ok(stdout)
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

pub fn get_system_info() -> Result<SystemInfo> {
    let disks = enumerate_disks()?;
    let total_disks = disks.len();
    let total_capacity_bytes = disks.iter().map(|d| d.size_bytes).sum();
    let system_disk_id = disks
        .iter()
        .find(|d| d.is_system_disk)
        .map(|d| d.id.clone());

    let mut os_name = "Windows".to_string();
    let mut os_version = "Unknown".to_string();

    #[derive(serde::Deserialize)]
    #[allow(non_snake_case)]
    struct Win32OS {
        Caption: String,
        Version: String,
    }

    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_OperatingSystem | Select-Object Caption,Version | ConvertTo-Json"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                if let Ok(os) = serde_json::from_str::<Win32OS>(&stdout) {
                    os_name = os.Caption;
                    os_version = os.Version;
                }
            }
        }
    }

    Ok(SystemInfo {
        os_name,
        os_version,
        is_admin: crate::utils::is_elevated(),
        total_disks,
        total_capacity_bytes,
        system_disk_id,
    })
}
