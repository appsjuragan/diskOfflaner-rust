# DiskOfflaner

DiskOfflaner is a robust utility designed for Windows to easily manage the online/offline status of your physical disks. It provides a user-friendly Graphical User Interface (GUI) to visualize disk status, partitions, and perform safe operations.

## Features

*   **Visual Dashboard**: View all connected physical disks, their models, sizes, and partition layouts at a glance.
*   **Status Indicators**: Clear visual cues (Green/Red icons) indicating whether a disk is Online or Offline.
*   **One-Click Toggling**: Easily switch a disk between Online and Offline states with a single button.
*   **Safety First**:
    *   **System Disk Protection**: Critical system/boot disks are highlighted with a warning tag.
    *   **Confirmation Dialogs**: Prevents accidental offlining of system disks with a mandatory confirmation prompt.
    *   **In-Use Detection**: Intelligently detects if a disk is currently in use by active processes and prevents forced offlining to avoid data loss, providing a clear error notification.
*   **Theme Support**: Toggle between Dark Mode (default) and Light Mode to suit your preference.
*   **Asynchronous Operations**: Disk operations run in the background, keeping the interface responsive.

## Requirements

*   **Operating System**: Windows 10 or Windows 11.
*   **Privileges**: This application **must be run as Administrator** to perform disk operations (Diskpart commands).

## Installation & Usage

1.  Download the latest release (`diskofflaner.exe`).
2.  Right-click the executable and select **"Run as administrator"**.
3.  The GUI will launch and scan your disks.
    *   **To set a disk Offline**: Click the "Set Offline" button next to an Online disk.
    *   **To set a disk Online**: Click the "Set Online" button next to an Offline disk.
    *   **Refresh**: If you plug in a new drive, click "Refresh List" to update the view.

## Building from Source

To build this project, you need [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/appsjuragan/diskOfflaner-rust.git
cd diskOfflaner-rust
cargo build --release
```

The executable will be located in `target/release/diskofflaner.exe`.

## License

[MIT License](LICENSE)