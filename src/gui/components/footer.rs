use crate::structs::{DiskInfo, DiskType};
use eframe::egui;

pub fn show_footer(ui: &mut egui::Ui, disks: &[DiskInfo]) {
    let hdd_count = disks
        .iter()
        .filter(|d| matches!(d.disk_type, DiskType::HDD))
        .count();
    let ssd_count = disks
        .iter()
        .filter(|d| matches!(d.disk_type, DiskType::SSD))
        .count();
    let nvme_count = disks
        .iter()
        .filter(|d| matches!(d.disk_type, DiskType::NVMe))
        .count();
    let ext_hdd_count = disks
        .iter()
        .filter(|d| matches!(d.disk_type, DiskType::ExtHDD))
        .count();
    let usb_count = disks
        .iter()
        .filter(|d| matches!(d.disk_type, DiskType::USBFlash))
        .count();

    ui.horizontal(|ui| {
        ui.label(format!("HDD: {}", hdd_count));
        ui.separator();
        ui.label(format!("SSD: {}", ssd_count));
        ui.separator();
        ui.label(format!("NVMe: {}", nvme_count));
        ui.separator();
        ui.label(format!("Ext. HDD: {}", ext_hdd_count));
        ui.separator();
        ui.label(format!("USB Flash: {}", usb_count));
    });
}
