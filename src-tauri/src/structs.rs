use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum DiskType {
    #[default]
    HDD,
    SSD,
    NVMe,
    ExtHDD,
    USBFlash,
    Unknown,
}

impl std::fmt::Display for DiskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskType::HDD => write!(f, "HDD"),
            DiskType::SSD => write!(f, "SSD"),
            DiskType::NVMe => write!(f, "NVMe"),
            DiskType::ExtHDD => write!(f, "External HDD"),
            DiskType::USBFlash => write!(f, "USB Flash"),
            DiskType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub id: String,
    pub model: String,
    pub size_bytes: u64,
    pub is_online: bool,
    pub is_system_disk: bool,
    pub partitions: Vec<PartitionInfo>,
    pub disk_type: DiskType,
    pub serial_number: Option<String>,
    pub health_percentage: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    pub partition_number: u32,
    pub size_bytes: u64,
    pub drive_letter: String,
    pub partition_id: String,
}
