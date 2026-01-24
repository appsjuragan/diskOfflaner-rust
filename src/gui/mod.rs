#![allow(clippy::uninlined_format_args)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::ignored_unit_patterns)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::ref_as_ptr)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::borrow_as_ptr)]

pub mod components;
pub mod themes;

use crate::disk_operations::enumerate_disks;
use crate::disk_operations::{set_disk_offline, set_disk_online};
use crate::structs::DiskInfo;
use anyhow::Result;
use eframe::egui;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use components::{show_disk_card, show_footer, show_header, DiskAction};

pub fn run_gui() -> Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(egui::vec2(450.0, 600.0));

    eframe::run_native(
        &format!("DiskOfflaner v{}", env!("CARGO_PKG_VERSION")),
        options,
        Box::new(|cc| {
            // Default to Dark Mode using centralized theme
            themes::apply_dark_theme(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let mut app = DiskApp::default();
            app.refresh_disks();
            app.start_device_monitoring(cc.egui_ctx.clone());
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
    is_loading_disks: bool,
    disk_load_receiver: Option<Receiver<Result<Vec<DiskInfo>, String>>>,
    operation_error: Option<String>,
    device_change_receiver: Option<Receiver<()>>,
    #[allow(dead_code)]
    last_auto_refresh: Option<Instant>,
    #[allow(dead_code)]
    device_monitor_active: Arc<AtomicBool>,
    // Mount Dialog State
    mounting_partition: Option<(String, u32)>,
    mount_letter_candidates: Vec<String>,
    selected_mount_letter: String,
}

impl eframe::App for DiskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_background_updates(ctx);

        // Mount Dialog
        if let Some((disk_id, part_num)) = self.mounting_partition.clone() {
            let mut open = true;
            egui::Window::new("Mount Partition")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.label("Select a drive letter for the new partition:");
                    ui.add_space(5.0);

                    egui::ComboBox::from_label("Drive Letter")
                        .selected_text(&self.selected_mount_letter)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.selected_mount_letter,
                                "Auto".to_string(),
                                "Auto",
                            );
                            for letter in &self.mount_letter_candidates {
                                ui.selectable_value(
                                    &mut self.selected_mount_letter,
                                    letter.clone(),
                                    letter,
                                );
                            }
                        });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Mount").clicked() {
                            let letter = if self.selected_mount_letter == "Auto" {
                                None
                            } else {
                                self.selected_mount_letter.chars().next()
                            };
                            self.mounting_partition = None;
                            self.start_partition_operation(
                                disk_id.clone(),
                                part_num,
                                None,
                                true,
                                letter,
                            );
                        }
                        if ui.button("Cancel").clicked() {
                            self.mounting_partition = None;
                        }
                    });
                });

            if !open {
                self.mounting_partition = None;
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
            ctx.request_repaint();
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
                            self.start_disk_operation(id, true);
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            show_header(ui, ctx, self.is_loading_disks, || self.refresh_disks());
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            show_footer(ui, &self.disks);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_enabled(self.processing_disk.is_none());

            if self.is_loading_disks {
                ctx.request_repaint();
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

            ui.scope(|ui| {
                ui.set_enabled(!self.is_loading_disks);
                let disks_view = self.disks.clone();
                let mut action = None;
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for disk in &disks_view {
                        if let Some(act) = show_disk_card(ui, disk) {
                            action = Some(act);
                        }
                        ui.add_space(5.0);
                    }
                });

                if let Some(act) = action {
                    match act {
                        DiskAction::SetOffline { disk_id } => {
                            self.start_disk_operation(disk_id, true)
                        }
                        DiskAction::SetOnline { disk_id } => {
                            self.start_disk_operation(disk_id, false)
                        }
                        DiskAction::ConfirmSystemOffline { disk_id } => {
                            self.pending_offline_disk = Some(disk_id)
                        }
                        DiskAction::MountPartition {
                            disk_id,
                            partition_number,
                        } => {
                            self.mounting_partition = Some((disk_id, partition_number));
                            self.selected_mount_letter = "Auto".to_string();
                            #[cfg(target_os = "windows")]
                            {
                                self.mount_letter_candidates =
                                    crate::disk_operations::get_available_drive_letters();
                            }
                            #[cfg(not(target_os = "windows"))]
                            {
                                self.mount_letter_candidates = vec![];
                            }
                        }
                        DiskAction::UnmountPartition {
                            disk_id,
                            drive_letter,
                        } => self.start_partition_operation(
                            disk_id,
                            0,
                            Some(drive_letter),
                            false,
                            None,
                        ),
                    }
                }
            });
        });
    }
}

impl DiskApp {
    fn handle_background_updates(&mut self, _ctx: &egui::Context) {
        // Check for device change notifications
        if let Some(rx) = &self.device_change_receiver {
            if rx.try_recv().is_ok() {
                let should_refresh = if let Some(last_time) = self.last_auto_refresh {
                    !self.is_loading_disks && last_time.elapsed() > Duration::from_secs(2)
                } else {
                    !self.is_loading_disks
                };

                if should_refresh {
                    self.last_auto_refresh = Some(Instant::now());
                    self.refresh_disks();
                }
            }
        }

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
                        self.refresh_disks();
                    }
                    Err(e) => {
                        let err_lower = e.to_lowercase();
                        if err_lower.contains(
                            "disk attributes may not be changed on the current system disk",
                        ) {
                            self.operation_error = Some("Operation Failed: Cannot modify the system or boot disk.\n\nWindows prevents taking the drive running the OS offline to avoid a system crash.".to_string());
                        } else if err_lower.contains("pagefile")
                            || err_lower.contains("crashdump")
                            || err_lower.contains("hibernation")
                        {
                            self.operation_error = Some("Operation Failed: This partition contains system files (Pagefile, Crashdump, or Hibernation file).\n\nWindows cannot unmount partitions that store these critical system files.".to_string());
                        } else if err_lower.contains("system volume")
                            || err_lower.contains("boot volume")
                        {
                            self.operation_error = Some("Operation Failed: This is a System or Boot partition.\n\nUnmounting it would cause the operating system to crash.".to_string());
                        } else if err_lower.contains("in use") {
                            self.operation_error = Some("Operation Failed: The drive is currently in use.\n\nPlease close any applications (like File Explorer) or files using this drive and try again.".to_string());
                        } else if err_lower.contains("virtual disk service error") {
                            let clean_err = e
                                .lines()
                                .find(|l| l.to_lowercase().contains("virtual disk service error"))
                                .map(|l| l.trim().to_string())
                                .unwrap_or_else(|| {
                                    "Unknown Virtual Disk Service Error".to_string()
                                });
                            self.operation_error = Some(format!("Operation Failed: {}", clean_err));
                        } else {
                            self.operation_error = Some(format!("Operation Failed: {}", e));
                        }
                    }
                }
            }
        }
    }

    fn start_device_monitoring(&mut self, ctx: egui::Context) {
        let (tx, rx) = channel();
        self.device_change_receiver = Some(rx);
        self.last_auto_refresh = Some(Instant::now());
        let monitor_active = Arc::new(AtomicBool::new(true));
        self.device_monitor_active = monitor_active.clone();

        #[cfg(target_os = "windows")]
        {
            thread::spawn(move || {
                monitor_device_changes_windows(tx, ctx, monitor_active);
            });
        }

        #[cfg(target_os = "linux")]
        {
            thread::spawn(move || {
                monitor_device_changes_linux(tx, ctx, monitor_active);
            });
        }
    }

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

    fn start_partition_operation(
        &mut self,
        disk_id: String,
        partition_number: u32,
        drive_letter: Option<String>,
        is_mount: bool,
        mount_letter: Option<char>,
    ) {
        self.processing_disk = Some(disk_id.clone());
        let (tx, rx) = channel();
        self.op_receiver = Some(rx);

        thread::spawn(move || {
            let result = if is_mount {
                #[cfg(target_os = "windows")]
                let r = crate::disk_operations::mount_partition(
                    disk_id,
                    partition_number,
                    mount_letter,
                );
                #[cfg(not(target_os = "windows"))]
                let r = crate::disk_operations::mount_partition(disk_id, partition_number);
                r
            } else if let Some(letter) = drive_letter {
                crate::disk_operations::unmount_partition(letter)
            } else {
                Err(anyhow::anyhow!("No drive letter to unmount"))
            };
            let _ = tx.send(result.map_err(|e| e.to_string()));
        });
    }
}

#[cfg(target_os = "windows")]
fn monitor_device_changes_windows(tx: Sender<()>, ctx: egui::Context, active: Arc<AtomicBool>) {
    use std::ptr;
    use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
    use winapi::shared::windef::HWND;
    use winapi::um::winuser::{
        CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassW,
        TranslateMessage, CS_OWNDC, CW_USEDEFAULT, MSG, WM_DEVICECHANGE, WNDCLASSW,
        WS_OVERLAPPEDWINDOW,
    };

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        use winapi::um::winuser::{
            GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTW, GWLP_USERDATA, WM_CREATE,
        };

        // WM_DEVICECHANGE wparam constants
        const DBT_DEVICEARRIVAL: usize = 0x8000;
        const DBT_DEVICEREMOVECOMPLETE: usize = 0x8004;

        match msg {
            WM_CREATE => {
                let create_struct = &*(lparam as *const CREATESTRUCTW);
                let tx_ptr = create_struct.lpCreateParams;
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, tx_ptr as isize);
                0
            }
            WM_DEVICECHANGE => {
                if wparam == DBT_DEVICEARRIVAL || wparam == DBT_DEVICEREMOVECOMPLETE {
                    let tx_ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Sender<()>;
                    if !tx_ptr.is_null() {
                        if let Some(tx) = tx_ptr.as_ref() {
                            let _ = tx.send(());
                        }
                    }
                }
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    unsafe {
        let class_name: Vec<u16> = "DiskOfflaner_DeviceMonitor\0".encode_utf16().collect();
        let wc = WNDCLASSW {
            style: CS_OWNDC,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: std::mem::size_of::<*const Sender<()>>() as i32,
            hInstance: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
        };
        RegisterClassW(&wc);
        let window_name: Vec<u16> = "DiskOfflaner Device Monitor\0".encode_utf16().collect();
        let _hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &tx as *const Sender<()> as *mut _,
        );
        let mut msg = MSG {
            hwnd: ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: std::mem::zeroed(),
        };
        while active.load(Ordering::Relaxed) && GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            ctx.request_repaint();
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

#[cfg(target_os = "linux")]
fn monitor_device_changes_linux(tx: Sender<()>, ctx: egui::Context, active: Arc<AtomicBool>) {
    use std::collections::HashSet;
    use std::fs;
    let mut previous_devices = HashSet::new();
    if let Ok(entries) = fs::read_dir("/dev") {
        for entry in entries.flatten() {
            if let Ok(name) = entry.file_name().into_string() {
                if name.starts_with("sd") || name.starts_with("nvme") || name.starts_with("mmcblk")
                {
                    previous_devices.insert(name);
                }
            }
        }
    }
    while active.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_secs(1));
        let mut current_devices = HashSet::new();
        if let Ok(entries) = fs::read_dir("/dev") {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.starts_with("sd")
                        || name.starts_with("nvme")
                        || name.starts_with("mmcblk")
                    {
                        current_devices.insert(name);
                    }
                }
            }
        }
        if current_devices != previous_devices {
            let _ = tx.send(());
            ctx.request_repaint();
            previous_devices = current_devices;
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn monitor_device_changes_windows(_tx: Sender<()>, _ctx: egui::Context, _active: Arc<AtomicBool>) {}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn monitor_device_changes_linux(_tx: Sender<()>, _ctx: egui::Context, _active: Arc<AtomicBool>) {}
