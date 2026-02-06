pub mod disk_operations;
pub mod structs;
pub mod utils;
pub mod logger;

use crate::disk_operations::*;
use crate::structs::{DiskInfo, SystemInfo};
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct CacheState {
    system_info: Mutex<Option<(SystemInfo, Instant)>>,
}

#[tauri::command]
fn get_system_info_command(state: tauri::State<CacheState>) -> Result<SystemInfo, String> {
    let mut cache = state.system_info.lock().map_err(|e| e.to_string())?;

    if let Some((info, timestamp)) = &*cache {
        if timestamp.elapsed() < Duration::from_secs(30 * 60) {
            return Ok(info.clone());
        }
    }

    let info = get_system_info().map_err(|e| e.to_string())?;
    *cache = Some((info.clone(), Instant::now()));
    Ok(info)
}

#[tauri::command]
fn enumerate_disks_command() -> Result<Vec<DiskInfo>, String> {
    enumerate_disks().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_disk_online_command(disk_id: String) -> Result<(), String> {
    let res = set_disk_online(disk_id.clone()).map_err(|e| e.to_string());
    if res.is_ok() {
        logger::log_activity(&format!("Set Disk {} Online", disk_id));
    }
    res
}

#[tauri::command]
fn set_disk_offline_command(disk_id: String) -> Result<(), String> {
    let res = set_disk_offline(disk_id.clone()).map_err(|e| e.to_string());
    if res.is_ok() {
        logger::log_activity(&format!("Set Disk {} Offline", disk_id));
    }
    res
}

#[tauri::command]
fn mount_partition_command(
    disk_id: String,
    partition_number: u32,
    letter: Option<char>,
) -> Result<Option<char>, String> {
    let res = mount_partition(disk_id.clone(), partition_number, letter).map_err(|e| e.to_string());
    if let Ok(assigned) = &res {
        let l_str = assigned.map(|c| format!(" to {}:\\", c)).unwrap_or_default();
        logger::log_activity(&format!("Mounted Partition {} on Disk {}{}", partition_number, disk_id, l_str));
    }
    res
}

#[tauri::command]
fn unmount_partition_command(volume_or_letter: String) -> Result<(), String> {
    let res = unmount_partition(volume_or_letter.clone()).map_err(|e| e.to_string());
    if res.is_ok() {
        logger::log_activity(&format!("Unmounted/Ejected Partition {}", volume_or_letter));
    }
    res
}

#[tauri::command]
fn get_logs_command() -> Vec<String> {
    logger::get_logs()
}

#[tauri::command]
fn clear_logs_command() {
    logger::clear_logs()
}

#[tauri::command]
#[cfg(target_os = "windows")]
fn get_available_drive_letters_command() -> Vec<String> {
    get_available_drive_letters()
}

#[tauri::command]
#[cfg(not(target_os = "windows"))]
fn get_available_drive_letters_command() -> Vec<String> {
    vec![]
}

#[tauri::command]
fn open_file_explorer_command(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("Not supported on this OS".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Check elevation
    if !crate::utils::is_elevated() {
        // In GUI mode, we can't easily print to console.
        // We could show a dialog, but for now allow Tauri to run
        // and let operations fail or let user handle it.
        // Or we could use a raw WinAPI MessageBox if purely Windows.
        // But better to just log it (if we had logging).
    }

    tauri::Builder::default()
        .manage(CacheState {
            system_info: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            enumerate_disks_command,
            set_disk_online_command,
            set_disk_offline_command,
            mount_partition_command,
            unmount_partition_command,
            get_available_drive_letters_command,
            get_system_info_command,
            open_file_explorer_command,
            get_logs_command,
            clear_logs_command
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                std::process::exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
