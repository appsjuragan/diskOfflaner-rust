use crate::structs::{DiskInfo, DiskType};
use eframe::egui;

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
            ui.horizontal(|ui| {
                let is_mounted = !part.drive_letter.is_empty();
                let label_text = format!(
                    "Partition {}: {:.2} GB ({})",
                    part.partition_number,
                    part.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                    part.drive_letter
                );

                if is_mounted {
                    let color = if ui.visuals().dark_mode {
                        egui::Color32::from_rgb(135, 206, 250) // Light Sky Blue
                    } else {
                        egui::Color32::from_rgb(0, 51, 102) // Dark Blue
                    };
                    ui.label(egui::RichText::new(label_text).color(color));
                } else {
                    ui.label(label_text);
                }

                // Determine if we should show the button and what the label should be
                let (show_button, btn_label) = if disk.disk_type == DiskType::USBFlash {
                    (true, if is_mounted { "Eject" } else { "Mount" })
                } else {
                    (true, if is_mounted { "Unmount" } else { "Mount" })
                };

                if show_button {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Use add_enabled_ui to handle enabling/disabling while allowing add_sized
                        if ui
                            .add_enabled_ui(disk.is_online, |ui| {
                                ui.add_sized(egui::vec2(100.0, 20.0), egui::Button::new(btn_label))
                            })
                            .inner
                            .clicked()
                        {
                            on_partition_op(
                                part.partition_number,
                                if is_mounted {
                                    Some(part.drive_letter.clone())
                                } else {
                                    None
                                },
                                !is_mounted,
                            );
                        }
                    });
                }
            });
        }
    });
}
