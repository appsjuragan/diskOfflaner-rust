#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub id: String,
    pub model: String,
    pub size_bytes: u64,
    pub is_online: bool,
    pub is_system_disk: bool,
    pub partitions: Vec<PartitionInfo>,
}

#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub partition_number: u32,
    pub size_bytes: u64,
    pub drive_letter: String,
    pub partition_id: String,
}
