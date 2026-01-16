use eframe::egui;
use crate::structs::{DiskInfo, DiskType};

pub fn show_partition_list(
    ui: &mut egui::Ui,
    disk: &DiskInfo,
    mut on_partition_op: impl FnMut(u32, Option<String>, bool),
) {
    if disk.partitions.is_empty() {
        return;
    }

    ui.indent("partitions", |ui| {
        for part in &disk.partitions {
            ui.label(format!(
                "Partition {}: {:.2} GB ({})",
                part.partition_number,
                part.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                part.drive_letter
            ));

            if disk.disk_type == DiskType::USBFlash {
                let is_mounted = !part.drive_letter.is_empty();
                let btn_label = if is_mounted { "Eject" } else { "Mount" };
                if ui.button(btn_label).clicked() {
                    on_partition_op(
                        part.partition_number,
                        if is_mounted { Some(part.drive_letter.clone()) } else { None },
                        !is_mounted,
                    );
                }
            }
        }
    });
}
