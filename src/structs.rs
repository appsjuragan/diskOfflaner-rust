#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub enum DiskType {
    HDD,
    SSD,
    NVMe,
    ExtHDD,
    USBFlash,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub id: String,
    pub model: String,
    pub size_bytes: u64,
    pub is_online: bool,
    pub is_system_disk: bool,
    pub partitions: Vec<PartitionInfo>,
    pub disk_type: DiskType,
}

#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub partition_number: u32,
    pub size_bytes: u64,
    pub drive_letter: String,
    pub partition_id: String,
}
