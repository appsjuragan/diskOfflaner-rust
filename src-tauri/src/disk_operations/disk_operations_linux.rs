// src/disk_operations/disk_operations_linux.rs
use crate::structs::{DiskInfo, DiskType, PartitionInfo};
use anyhow::Result;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<BlockDevice>,
}

#[derive(Debug, Deserialize, Debug, Deserialize)]
struct BlockDevice {
    name: String,
    size: Option<u64>, // lsblk -b gives bytes
    #[serde(rename = "type")]
    device_type: Option<String>,
    mountpoint: Option<String>,
    model: Option<String>,
    #[serde(rename = "serial")]
    serial: Option<String>,
    state: Option<String>,
    rm: Option<String>,   // Removable flag
    rota: Option<String>, // Rotational (1 = HDD, 0 = SSD)
    tran: Option<String>, // Transport type (nvme, usb, sata, etc.)
    children: Option<Vec<BlockDevice>>,
}

pub fn enumerate_disks() -> Result<Vec<DiskInfo>> {
    // lsblk -J -b -o NAME,SIZE,TYPE,MOUNTPOINT,MODEL,SERIAL,STATE,RM,ROTA,TRAN
    let output = Command::new("lsblk")
        .arg("-J") // JSON output
        .arg("-b") // Bytes
        .arg("-o")
        .arg("NAME,SIZE,TYPE,MOUNTPOINT,MODEL,SERIAL,STATE,RM,ROTA,TRAN")
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("lsblk failed"));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lsblk: LsblkOutput = serde_json::from_str(&output_str)?;

    let mut disks = Vec::new();

    for device in lsblk.blockdevices {
        // Filter for actual disks
        if device.device_type.as_deref() != Some("disk") {
            continue;
        }

        let id = device.name.clone();
        let model = device.model.unwrap_or_else(|| format!("Disk {}", id));
        let size_bytes = device.size.unwrap_or(0);

        // Check state. If state is "offline", then it's offline.
        let is_online = device.state.as_deref() != Some("offline");

        // Determine disk type based on transport and properties
        let disk_type = get_disk_type_linux(&device);

        let mut partitions = Vec::new();
        let mut is_system_disk = false;

        if let Some(children) = device.children {
            for (i, child) in children.into_iter().enumerate() {
                if child.device_type.as_deref() == Some("part") {
                    let mountpoint = child.mountpoint.clone().unwrap_or_default();
                    if mountpoint == "/" || mountpoint == "/boot" || mountpoint == "/boot/efi" {
                        is_system_disk = true;
                    }

                    partitions.push(PartitionInfo {
                        partition_number: (i + 1) as u32,
                        size_bytes: child.size.unwrap_or(0),
                        drive_letter: mountpoint, // Using drive_letter field for mountpoint
                        partition_id: child.name, // Using partition_id for device name (e.g. sda1)
                    });
                }
            }
        }

        disks.push(DiskInfo {
            id,
            model,
            size_bytes,
            is_online,
            is_system_disk,
            partitions,
            disk_type,
            disk_type,
            serial_number: device.serial,
            health_percentage: None,
        });
    }

    Ok(disks)
}

fn get_disk_type_linux(device: &BlockDevice) -> DiskType {
    // Check transport type first
    if let Some(tran) = &device.tran {
        match tran.as_str() {
            "nvme" => return DiskType::NVMe,
            "usb" => {
                // Check if removable
                if device.rm.as_deref() == Some("1") {
                    return DiskType::USBFlash;
                } else {
                    return DiskType::ExtHDD;
                }
            }
            _ => {}
        }
    }

    // Check rotational flag (SSD vs HDD)
    if let Some(rota) = &device.rota {
        if rota == "0" {
            return DiskType::SSD;
        } else if rota == "1" {
            return DiskType::HDD;
        }
    }

    // Default to HDD
    DiskType::HDD
}

pub fn set_disk_online(disk_id: String) -> Result<()> {
    // Try to write "running" to /sys/block/disk_id/device/state
    // Note: disk_id is like "sda"
    let path = format!("/sys/block/{}/device/state", disk_id);
    if std::path::Path::new(&path).exists() {
        std::fs::write(path, "running")?;
    } else {
        return Err(anyhow::anyhow!("Cannot change state for {}", disk_id));
    }
    Ok(())
}

pub fn set_disk_offline(disk_id: String) -> Result<()> {
    // Try to write "offline" to /sys/block/disk_id/device/state
    let path = format!("/sys/block/{}/device/state", disk_id);
    if std::path::Path::new(&path).exists() {
        std::fs::write(path, "offline")?;
    } else {
        return Err(anyhow::anyhow!("Cannot change state for {}", disk_id));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn eject_disk(disk_id: String) -> Result<()> {
    // For removable drives, use udisksctl to power off
    let output = Command::new("udisksctl")
        .arg("power-off")
        .arg("-b")
        .arg(format!("/dev/{}", disk_id))
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to eject disk: {}", stderr));
    }
    Ok(())
}

pub fn mount_partition(disk_id: String, partition_number: u32) -> Result<()> {
    // Construct device path based on disk ID
    // standard: sda -> sda1
    // nvme: nvme0n1 -> nvme0n1p1
    let device_path = if disk_id.starts_with("nvme") {
        format!("/dev/{}p{}", disk_id, partition_number)
    } else {
        format!("/dev/{}{}", disk_id, partition_number)
    };

    let output = Command::new("udisksctl")
        .arg("mount")
        .arg("-b")
        .arg(&device_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to mount partition: {}", stderr));
    }
    Ok(())
}

pub fn unmount_partition(mount_point: String) -> Result<()> {
    // mount_point on Linux is the actual mount path (e.g., /media/user/USB)
    // We need to find the device from the mount point
    let output = Command::new("findmnt")
        .arg("-n")
        .arg("-o")
        .arg("SOURCE")
        .arg(&mount_point)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Cannot find device for mount point: {}",
            mount_point
        ));
    }

    let device = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("udisksctl")
        .arg("unmount")
        .arg("-b")
        .arg(&device)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to unmount partition: {}", stderr));
    }
    Ok(())
}
