// src/tests/mod.rs
// Unit tests for safe components

#[cfg(test)]
mod structs_tests {
    use crate::structs::{DiskInfo, DiskType, PartitionInfo};

    #[test]
    fn test_disk_type_default() {
        let disk_type: DiskType = DiskType::default();
        assert_eq!(disk_type, DiskType::HDD);
    }

    #[test]
    fn test_disk_type_display() {
        assert_eq!(format!("{}", DiskType::HDD), "HDD");
        assert_eq!(format!("{}", DiskType::SSD), "SSD");
        assert_eq!(format!("{}", DiskType::NVMe), "NVMe");
        assert_eq!(format!("{}", DiskType::ExtHDD), "External HDD");
        assert_eq!(format!("{}", DiskType::USBFlash), "USB Flash");
        assert_eq!(format!("{}", DiskType::Unknown), "Unknown");
    }

    #[test]
    fn test_disk_type_equality() {
        assert_eq!(DiskType::SSD, DiskType::SSD);
        assert_ne!(DiskType::SSD, DiskType::HDD);
    }

    #[test]
    fn test_disk_type_clone() {
        let original = DiskType::NVMe;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_partition_info_creation() {
        let partition = PartitionInfo {
            partition_number: 1,
            size_bytes: 1024 * 1024 * 1024, // 1 GB
            drive_letter: "C".to_string(),
            partition_id: "PART1".to_string(),
        };

        assert_eq!(partition.partition_number, 1);
        assert_eq!(partition.size_bytes, 1_073_741_824);
        assert_eq!(partition.drive_letter, "C");
        assert_eq!(partition.partition_id, "PART1");
    }

    #[test]
    fn test_disk_info_creation() {
        let disk = DiskInfo {
            id: "0".to_string(),
            model: "Test Disk".to_string(),
            size_bytes: 500 * 1024 * 1024 * 1024, // 500 GB
            is_online: true,
            is_system_disk: false,
            partitions: vec![],
            disk_type: DiskType::SSD,
        };

        assert_eq!(disk.id, "0");
        assert_eq!(disk.model, "Test Disk");
        assert!(disk.is_online);
        assert!(!disk.is_system_disk);
        assert!(disk.partitions.is_empty());
        assert_eq!(disk.disk_type, DiskType::SSD);
    }

    #[test]
    fn test_disk_info_with_partitions() {
        let partition1 = PartitionInfo {
            partition_number: 1,
            size_bytes: 100 * 1024 * 1024 * 1024,
            drive_letter: "C".to_string(),
            partition_id: "P1".to_string(),
        };

        let partition2 = PartitionInfo {
            partition_number: 2,
            size_bytes: 200 * 1024 * 1024 * 1024,
            drive_letter: "D".to_string(),
            partition_id: "P2".to_string(),
        };

        let disk = DiskInfo {
            id: "1".to_string(),
            model: "Multi-partition Disk".to_string(),
            size_bytes: 300 * 1024 * 1024 * 1024,
            is_online: true,
            is_system_disk: true,
            partitions: vec![partition1, partition2],
            disk_type: DiskType::HDD,
        };

        assert_eq!(disk.partitions.len(), 2);
        assert!(disk.is_system_disk);
        assert_eq!(disk.partitions[0].drive_letter, "C");
        assert_eq!(disk.partitions[1].drive_letter, "D");
    }

    #[test]
    fn test_disk_info_clone() {
        let disk = DiskInfo {
            id: "test".to_string(),
            model: "Clone Test".to_string(),
            size_bytes: 1024,
            is_online: true,
            is_system_disk: false,
            partitions: vec![],
            disk_type: DiskType::USBFlash,
        };

        let cloned = disk.clone();
        assert_eq!(disk.id, cloned.id);
        assert_eq!(disk.model, cloned.model);
        assert_eq!(disk.size_bytes, cloned.size_bytes);
        assert_eq!(disk.disk_type, cloned.disk_type);
    }
}
