use anyhow::Result;
use std::process::Command;
use crate::structs::{DiskInfo, PartitionInfo};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<BlockDevice>,
}

#[derive(Debug, Deserialize)]
struct BlockDevice {
    name: String,
    size: Option<u64>, // lsblk -b gives bytes, but sometimes it might be missing or string? JSON usually handles numbers. Option just in case.
    #[serde(rename = "type")]
    device_type: Option<String>,
    mountpoint: Option<String>,
    model: Option<String>,
    state: Option<String>,
    children: Option<Vec<BlockDevice>>,
}

pub fn enumerate_disks() -> Result<Vec<DiskInfo>> {
    // lsblk -J -b -o NAME,SIZE,TYPE,MOUNTPOINT,MODEL,STATE
    let output = Command::new("lsblk")
        .arg("-J") // JSON output
        .arg("-b") // Bytes
        .arg("-o")
        .arg("NAME,SIZE,TYPE,MOUNTPOINT,MODEL,STATE")
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
        let model = device.model.unwrap_or_else(|| "Unknown".to_string());
        let size_bytes = device.size.unwrap_or(0);
        
        // Check state. If state is "offline", then it's offline.
        let is_online = device.state.as_deref() != Some("offline");

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
        });
    }

    Ok(disks)
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
