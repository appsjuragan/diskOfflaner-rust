use crate::disk_operations::enumerate_disks;
use crate::disk_operations::{set_disk_offline, set_disk_online};
use crate::structs::{DiskInfo, DiskType};
use anyhow::Result;
use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub fn run_gui() -> Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::vec2(450.0, 600.0));

    eframe::run_native(
        &format!("DiskOfflaner v{}", env!("CARGO_PKG_VERSION")),
        options,
        Box::new(|cc| {
            // Default to Dark Mode
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            // Install image loaders
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            let mut app = DiskApp::default();
            // Start initial load
            app.refresh_disks();
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow::anyhow!("GUI Error: {}", e))
}

#[derive(Default)]
struct DiskApp {
    disks: Vec<DiskInfo>,
    error: Option<String>,
    pending_offline_disk: Option<String>,
    processing_disk: Option<String>,
    op_receiver: Option<Receiver<Result<(), String>>>,
    // Async disk loading
    is_loading_disks: bool,
    disk_load_receiver: Option<Receiver<Result<Vec<DiskInfo>, String>>>,
    // New field for operation errors (e.g., disk in use)
    operation_error: Option<String>,
}

impl eframe::App for DiskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for disk load results
        if let Some(rx) = &self.disk_load_receiver {
            if let Ok(result) = rx.try_recv() {
                self.is_loading_disks = false;
                self.disk_load_receiver = None;
                match result {
                    Ok(d) => {
                        self.disks = d;
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(format!("Failed to refresh disks: {}", e));
                    }
                }
            }
        }

        // Check for background operation results
        if let Some(rx) = &self.op_receiver {
            if let Ok(result) = rx.try_recv() {
                self.processing_disk = None;
                self.op_receiver = None;
                match result {
                    Ok(_) => {
                        self.refresh_disks(); // Refresh list after operation
                    }
                    Err(e) => {
                        // Provide a user-friendly message for common errors
                        let err_lower = e.to_lowercase();
                        if err_lower.contains("disk attributes may not be changed on the current system disk") {
                            self.operation_error = Some("Operation Failed: Cannot modify the system or boot disk.\n\nWindows prevents taking the drive running the OS offline to avoid a system crash.".to_string());
                        } else if err_lower.contains("in use") {
                            self.operation_error = Some("Operation Failed: Disk is currently in use.\n\nPlease close any applications or files using this drive and try again.".to_string());
                        } else if err_lower.contains("virtual disk service error") {
                             // Clean up the verbose VDS error
                            let clean_err = e.lines()
                                .find(|l| l.to_lowercase().contains("virtual disk service error"))
                                .map(|l| l.trim().to_string())
                                .unwrap_or_else(|| "Unknown Virtual Disk Service Error".to_string());
                             self.operation_error = Some(format!("Operation Failed: {}", clean_err));
                        } else {
                            self.operation_error = Some(format!("Operation Failed: {}", e));
                        }
                    }
                }
            }
        }

        // Processing Dialog
        if self.processing_disk.is_some() {
            egui::Window::new("Processing")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Please wait, disk operation in progress...");
                    });
                });
            ctx.request_repaint(); // Ensure spinner animates
        }

        // Confirmation Dialog
        if let Some(disk_id) = self.pending_offline_disk.clone() {
            egui::Window::new("⚠️ Critical Warning")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(
                        egui::RichText::new("You are about to set a SYSTEM/BOOT disk Offline!")
                            .color(egui::Color32::RED)
                            .strong(),
                    );
                    ui.label("This can cause system instability or crashes.");
                    ui.label("Are you absolutely sure you want to continue?");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes, Set Offline").clicked() {
                            let id = disk_id.clone();
                            self.pending_offline_disk = None;
                            self.start_disk_operation(id, true); // true = currently online, so set offline
                        }
                        if ui.button("Cancel").clicked() {
                            self.pending_offline_disk = None;
                        }
                    });
                });
        }

        // Operation Error Notification
        if let Some(err_msg) = self.operation_error.clone() {
            egui::Window::new("Operation Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.colored_label(egui::Color32::RED, err_msg);
                    if ui.button("OK").clicked() {
                        self.operation_error = None;
                    }
                });
        }

        // Top Panel for Title, Refresh and Theme Toggle
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("DiskOfflaner");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let is_dark = ctx.style().visuals.dark_mode;
                    let text = if is_dark { "Light Mode" } else { "Dark Mode" };
                    if ui.button(text).clicked() {
                        if is_dark {
                            // Switch to Light Mode with 95% Grey
                            let mut visuals = egui::Visuals::light();
                            let grey_95 = egui::Color32::from_gray(211);
                            visuals.panel_fill = grey_95;
                            visuals.window_fill = grey_95;
                            visuals.widgets.noninteractive.bg_fill = grey_95;
                            ctx.set_visuals(visuals);
                        } else {
                            ctx.set_visuals(egui::Visuals::dark());
                        }
                    }
                    // Refresh button with icon - positioned to the left of theme toggle
                    if self.is_loading_disks {
                        ui.add_enabled(false, egui::Button::new("⟳ Refreshing..."));
                        ui.spinner();
                    } else if ui.button("⟳ Refresh").clicked() {
                        self.refresh_disks();
                    }
                });
            });
        });

        // Bottom Panel for Disk Counts
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            let hdd_count = self.disks.iter().filter(|d| matches!(d.disk_type, DiskType::HDD)).count();
            let ssd_count = self.disks.iter().filter(|d| matches!(d.disk_type, DiskType::SSD)).count();
            let nvme_count = self.disks.iter().filter(|d| matches!(d.disk_type, DiskType::NVMe)).count();
            let ext_hdd_count = self.disks.iter().filter(|d| matches!(d.disk_type, DiskType::ExternalHDD)).count();
            let usb_count = self.disks.iter().filter(|d| matches!(d.disk_type, DiskType::USBFlash)).count();

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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Disable interaction if processing
            ui.set_enabled(self.processing_disk.is_none());

            // Keep the disk list visible during refresh
            if self.is_loading_disks {
                ctx.request_repaint(); // Ensure spinner animates in top panel
            }

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
                if ui.button("Retry").clicked() {
                    self.error = None;
                    self.refresh_disks();
                }
            }

            if self.disks.is_empty() && self.error.is_none() && !self.is_loading_disks {
                ui.label("No disks found.");
                if ui.button("Refresh").clicked() {
                    self.refresh_disks();
                }
                return;
            }

            // Scroll area for disks - grey out during refresh
            ui.scope(|ui| {
                // Disable and grey out the disk list while refreshing
                ui.set_enabled(!self.is_loading_disks);

                let disks_view = self.disks.clone();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for disk in &disks_view {
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
                                        if is_dark {
                                            egui::Color32::GREEN
                                        } else {
                                            egui::Color32::from_rgb(0, 128, 128)
                                        } // Teal
                                    } else if is_dark {
                                        egui::Color32::RED
                                    } else {
                                        egui::Color32::from_rgb(128, 0, 0)
                                    };

                                    // 1. Icon (Fixed Width 30)
                                    ui.allocate_ui(egui::vec2(30.0, ui.available_height()), |ui| {
                                        let icon_path = match disk.disk_type {
                                            DiskType::HDD => "file://assets/hdd.svg",
                                            DiskType::SSD => "file://assets/ssd.svg",
                                            DiskType::NVMe => "file://assets/nvme.svg",
                                            DiskType::ExternalHDD => "file://assets/external_hdd.svg",
                                            DiskType::USBFlash => "file://assets/usb.svg",
                                            _ => "file://assets/hdd.svg",
                                        };
                                        
                                        ui.add(
                                            egui::Image::new(icon_path)
                                                .fit_to_exact_size(egui::vec2(24.0, 24.0))
                                                .tint(status_color)
                                        );
                                    });

                                    // 2. Status (Fixed Width 60)
                                    ui.allocate_ui(egui::vec2(60.0, ui.available_height()), |ui| {
                                        ui.colored_label(status_color, status);
                                    });

                                    // 3. Right-aligned elements (Button and Type)
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Button handling per disk type
                                        if disk.disk_type == DiskType::USBFlash {
                                            // No disk-level button; handled per-partition above
                                            ui.allocate_space(egui::vec2(100.0, 20.0));
                                        } else if disk.disk_type == DiskType::NVMe {
                                            // NVMe: only allow setting offline
                                            let button_label = if disk.is_online { "Set Offline" } else { "Set Offline" };
                                            if ui.add_sized(egui::vec2(100.0, 20.0), egui::Button::new(button_label)).clicked() {
                                                // Force offline operation
                                                if disk.is_online {
                                                    self.start_disk_operation(disk.id.clone(), true);
                                                }
                                            }
                                        } else {
                                            // HDD and ExternalHDD: toggle online/offline
                                            let button_label = if disk.is_online { "Set Offline" } else { "Set Online" };
                                            if ui.add_sized(egui::vec2(100.0, 20.0), egui::Button::new(button_label)).clicked() {
                                                if disk.is_online && disk.is_system_disk {
                                                    self.pending_offline_disk = Some(disk.id.clone());
                                                } else {
                                                    self.start_disk_operation(disk.id.clone(), disk.is_online);
                                                }
                                            }
                                        }

                                        ui.add_space(10.0);

                                        // Type (Fixed Width 80)
                                        ui.allocate_ui(egui::vec2(80.0, ui.available_height()), |ui| {
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.label(format!("{:?}", disk.disk_type));
                                            });
                                        });

                                        // 4. Info (Model + Size) - Fills remaining middle space
                                        // We switch back to left-to-right for the text to appear correctly
                                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                            if disk.is_system_disk {
                                                ui.label(
                                                    egui::RichText::new("[SYSTEM]")
                                                        .color(egui::Color32::RED)
                                                        .strong(),
                                                );
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
                                                egui::RichText::new(info_text)
                                                    .color(egui::Color32::from_rgb(255, 165, 0))
                                                    .strong()
                                            } else {
                                                egui::RichText::new(info_text)
                                            };
                                            
                                            ui.label(info_text_rich);
                                        });
                                    });
                                });
                                // Show partitions
                                if !disk.partitions.is_empty() {
                                    ui.indent("partitions", |ui| {
                                        for part in &disk.partitions {
                                            ui.label(format!(
                                                "Partition {}: {:.2} GB ({})",
                                                part.partition_number,
                                                part.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                                                part.drive_letter
                                            ));

                                            // Add Eject/Mount buttons for USB Flash partitions
                                            if disk.disk_type == DiskType::USBFlash {
                                                let is_mounted = !part.drive_letter.is_empty();
                                                let btn_label = if is_mounted { "Eject" } else { "Mount" };
                                                if ui.button(btn_label).clicked() {
                                                    self.start_partition_operation(
                                                        disk.id.clone(), 
                                                        part.partition_number, 
                                                        if is_mounted { Some(part.drive_letter.clone()) } else { None },
                                                        !is_mounted
                                                    );
                                                }
                                            }
                                        }
                                    });
                                }
                            });
                        ui.add_space(5.0);
                    }
                });
            });
        });
    }
}

impl DiskApp {
    fn refresh_disks(&mut self) {
        self.is_loading_disks = true;
        self.error = None;
        let (tx, rx) = channel();
        self.disk_load_receiver = Some(rx);
        thread::spawn(move || {
            let result = enumerate_disks().map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
    }

    fn start_disk_operation(&mut self, disk_id: String, is_online: bool) {
        self.processing_disk = Some(disk_id.clone());
        let (tx, rx) = channel();
        self.op_receiver = Some(rx);
        thread::spawn(move || {
            let result = if is_online {
                set_disk_offline(disk_id)
            } else {
                set_disk_online(disk_id)
            };
            let _ = tx.send(result.map_err(|e| e.to_string()));
        });
    }



    fn start_partition_operation(&mut self, disk_id: String, partition_number: u32, drive_letter: Option<String>, is_mount: bool) {
        self.processing_disk = Some(disk_id.clone());
        let (tx, rx) = channel();
        self.op_receiver = Some(rx);
        
        let disk_num_res = disk_id.parse::<u32>();
        
        thread::spawn(move || {
            let result = match disk_num_res {
                Ok(disk_num) => {
                     if is_mount {
                        crate::disk_operations::mount_partition(disk_num, partition_number)
                    } else {
                        if let Some(letter) = drive_letter {
                            crate::disk_operations::unmount_partition(letter)
                        } else {
                            Err(anyhow::anyhow!("No drive letter to unmount"))
                        }
                    }
                },
                Err(e) => Err(anyhow::anyhow!("Invalid disk ID: {}", e)),
            };
            let _ = tx.send(result.map_err(|e| e.to_string()));
        });
    }
}
