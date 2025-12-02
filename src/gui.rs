use anyhow::Result;
use eframe::egui;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use crate::disk_operations::enumerate_disks;
use crate::disk_operations::{set_disk_online, set_disk_offline};
use crate::structs::DiskInfo;

pub fn run_gui() -> Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DiskOfflaner v1.0.0",
        options,
        Box::new(|cc| {
            // Default to Dark Mode
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            let mut app = DiskApp::default();
            // Start initial load
            app.refresh_disks();
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow::anyhow!("GUI Error: {}", e))
}

struct DiskApp {
    disks: Vec<DiskInfo>,
    error: Option<String>,
    pending_offline_disk: Option<u32>,
    processing_disk: Option<u32>,
    op_receiver: Option<Receiver<Result<(), String>>>,
    // Async disk loading
    is_loading_disks: bool,
    disk_load_receiver: Option<Receiver<Result<Vec<DiskInfo>, String>>>,
    // New field for operation errors (e.g., disk in use)
    operation_error: Option<String>,
}

impl Default for DiskApp {
    fn default() -> Self {
        Self {
            disks: Vec::new(),
            error: None,
            pending_offline_disk: None,
            processing_disk: None,
            op_receiver: None,
            is_loading_disks: false,
            disk_load_receiver: None,
            operation_error: None,
        }
    }
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
                        // Provide a user-friendly message if the disk is in use
                        if e.contains("in use") {
                            self.operation_error = Some("Failed to take disk offline: disk is currently in use by active processes.".to_string());
                        } else {
                            self.operation_error = Some(e);
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
        if let Some(disk_num) = self.pending_offline_disk {
            egui::Window::new("⚠️ Critical Warning")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("You are about to set a SYSTEM/BOOT disk Offline!").color(egui::Color32::RED).strong());
                    ui.label("This can cause system instability or crashes.");
                    ui.label("Are you absolutely sure you want to continue?");
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Yes, Set Offline").clicked() {
                            self.pending_offline_disk = None;
                            self.start_disk_operation(disk_num, true); // true = currently online, so set offline
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
                            let grey_95 = egui::Color32::from_gray(242);
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
                    } else {
                        if ui.button("⟳ Refresh").clicked() {
                            self.refresh_disks();
                        }
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();

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
                                    let status_color = if disk.is_online { egui::Color32::GREEN } else { egui::Color32::RED };
                                    
                                    // Colored HDD Icon
                                    ui.label(egui::RichText::new("🖴").size(24.0).color(status_color));
                                    ui.colored_label(status_color, status);
                                    if disk.is_system_disk {
                                        ui.add_space(5.0);
                                        ui.label(egui::RichText::new("[SYSTEM]").color(egui::Color32::RED).strong());
                                    }
                                    ui.add_space(5.0);
                                    // Avoid "Disk 0: Disk 0" redundancy
                                    let model_display = if disk.model == format!("Disk {}", disk.disk_number) {
                                        disk.model.clone()
                                    } else {
                                        format!("Disk {}: {}", disk.disk_number, disk.model)
                                    };
                                    let info_text = egui::RichText::new(format!(
                                        "{} - Size: {:.2} GB",
                                        model_display,
                                        disk.size_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
                                    ));
                                    let info_text = if disk.is_system_disk {
                                        info_text.color(egui::Color32::from_rgb(255, 165, 0)).strong()
                                    } else {
                                        info_text
                                    };
                                    ui.label(info_text);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let button_label = if disk.is_online { "Set Offline" } else { "Set Online" };
                                        if ui.button(button_label).clicked() {
                                            if disk.is_online && disk.is_system_disk {
                                                self.pending_offline_disk = Some(disk.disk_number);
                                            } else {
                                                self.start_disk_operation(disk.disk_number, disk.is_online);
                                            }
                                        }
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

    fn start_disk_operation(&mut self, disk_number: u32, is_online: bool) {
        self.processing_disk = Some(disk_number);
        let (tx, rx) = channel();
        self.op_receiver = Some(rx);
        thread::spawn(move || {
            let result = if is_online {
                set_disk_offline(disk_number)
            } else {
                set_disk_online(disk_number)
            };
            let _ = tx.send(result.map_err(|e| e.to_string()));
        });
    }
}
