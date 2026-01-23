use eframe::egui;
use crate::structs::{DiskInfo, DiskType};
use super::partition_list::show_partition_list;

/// Actions that can be triggered from a disk card
pub enum DiskAction {
    /// Set a disk offline
    SetOffline { disk_id: String },
    /// Set a disk online
    SetOnline { disk_id: String },
    /// Pending confirmation for system disk offline (shows warning dialog)
    ConfirmSystemOffline { disk_id: String },
    /// Mount a partition
    MountPartition { disk_id: String, partition_number: u32 },
    /// Unmount a partition (uses drive letter, not partition number)
    UnmountPartition { disk_id: String, drive_letter: String },
}

pub fn show_disk_card(
    ui: &mut egui::Ui,
    disk: &DiskInfo,
) -> Option<DiskAction> {
    let mut action = None;
    let border_color = if disk.is_system_disk {
        egui::Color32::from_rgb(255, 165, 0) // Orange for system disk
    } else {
        ui.visuals().widgets.noninteractive.bg_stroke.color
    };

    egui::Frame::group(ui.style())
        .stroke(egui::Stroke::new(1.0, border_color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let status = if disk.is_online { "Online" } else { "Offline" };
                let is_dark = ui.visuals().dark_mode;
                let status_color = if disk.is_online {
                    if is_dark { egui::Color32::GREEN } else { egui::Color32::from_rgb(0, 128, 128) }
                } else if is_dark {
                    egui::Color32::RED
                } else {
                    egui::Color32::from_rgb(128, 0, 0)
                };

                // 1. Icon
                ui.allocate_ui(egui::vec2(30.0, ui.available_height()), |ui| {
                    let icon = match disk.disk_type {
                        DiskType::HDD => egui::include_image!("../../../assets/hdd.svg"),
                        DiskType::SSD => egui::include_image!("../../../assets/ssd.svg"),
                        DiskType::NVMe => egui::include_image!("../../../assets/nvme.svg"),
                        DiskType::ExtHDD => egui::include_image!("../../../assets/external_hdd.svg"),
                        DiskType::USBFlash => egui::include_image!("../../../assets/usb.svg"),
                        _ => egui::include_image!("../../../assets/hdd.svg"),
                    };

                    ui.add(
                        egui::Image::new(icon)
                            .fit_to_exact_size(egui::vec2(24.0, 24.0))
                            .tint(status_color),
                    );
                });

                // 2. Status
                ui.allocate_ui(egui::vec2(60.0, ui.available_height()), |ui| {
                    ui.colored_label(status_color, status);
                });

                // 3. Right-aligned elements
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if disk.disk_type == DiskType::USBFlash {
                        ui.allocate_space(egui::vec2(100.0, 20.0));
                    } else if disk.disk_type == DiskType::NVMe {
                        if ui.add_sized(egui::vec2(100.0, 20.0), egui::Button::new("Set Offline")).clicked() {
                            if disk.is_online {
                                action = Some(DiskAction::SetOffline { disk_id: disk.id.clone() });
                            }
                        }
                    } else {
                        let button_label = if disk.is_online { "Set Offline" } else { "Set Online" };
                        if ui.add_sized(egui::vec2(100.0, 20.0), egui::Button::new(button_label)).clicked() {
                            if disk.is_online && disk.is_system_disk {
                                action = Some(DiskAction::ConfirmSystemOffline { disk_id: disk.id.clone() });
                            } else if disk.is_online {
                                action = Some(DiskAction::SetOffline { disk_id: disk.id.clone() });
                            } else {
                                action = Some(DiskAction::SetOnline { disk_id: disk.id.clone() });
                            }
                        }
                    }

                    ui.add_space(10.0);

                    // Type - using Display trait for human-readable names
                    ui.allocate_ui(egui::vec2(80.0, ui.available_height()), |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(format!("{}", disk.disk_type));
                        });
                    });

                    // 4. Info
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if disk.is_system_disk {
                            ui.label(egui::RichText::new("[SYSTEM]").color(egui::Color32::RED).strong());
                        }

                        let model_display = if disk.model == format!("Disk {}", disk.id) {
                            disk.model.clone()
                        } else {
                            format!("Disk {}: {}", disk.id, disk.model)
                        };

                        let info_text = format!(
                            "{} - {:.2} GB",
                            model_display,
                            disk.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
                        );

                        let info_text_rich = if disk.is_system_disk {
                            egui::RichText::new(info_text).color(egui::Color32::from_rgb(255, 165, 0)).strong()
                        } else {
                            egui::RichText::new(info_text)
                        };

                        ui.label(info_text_rich);
                    });
                });
            });

            show_partition_list(ui, disk, |part_num, letter, is_mount| {
                if is_mount {
                    action = Some(DiskAction::MountPartition {
                        disk_id: disk.id.clone(),
                        partition_number: part_num,
                    });
                } else if let Some(drive_letter) = letter {
                    action = Some(DiskAction::UnmountPartition {
                        disk_id: disk.id.clone(),
                        drive_letter,
                    });
                }
            });
        });

    action
}
