# DiskOfflaner

**Safe and simple disk management for Windows and Linux.**

DiskOfflaner is a desktop application designed to manage the online/offline status of physical disk drives. It provides a safe, user-friendly interface for system administrators and power users to disconnect drives via software without the need for physical removal.

## Features

- **Visual Dashboard**: View all physical disks, models, sizes, and partition information in a clean layout.
- **Status Indicators**: Clear visual cues distinguish between Online (Green) and Offline (Red) drives.
- **One-Click Control**: Toggle disk status easily without complex command-line tools.
- **Smart Refresh**: Updates disk information in the background without interrupting the user interface.
- **Safety Mechanisms**:
  - **System Protection**: Identifies and warns against modifying critical boot/system disks.
  - **Confirmation**: Requires explicit confirmation for sensitive operations.
  - **Usage Detection**: Prevents operations on drives that are currently in use to avoid data loss.
- **Modern Interface**: Includes both Dark and Light themes for comfortable usage in any environment.

## System Requirements

### Windows
- Windows 10 or Windows 11
- Administrator privileges (required for disk management)

### Linux
- Modern Linux Distribution (Ubuntu, Fedora, Arch, etc.)
- Root/Sudo privileges (required for disk management)
- `lsblk` utility

## Getting Started

### Windows
1. Download `diskofflaner.exe` from the [Releases](https://github.com/appsjuragan/diskOfflaner-rust/releases) page.
2. Right-click the executable and select **Run as administrator**.
3. The application will launch and scan your disks automatically.

### Linux
1. Download the Linux binary from the [Releases](https://github.com/appsjuragan/diskOfflaner-rust/releases) page.
2. Make the file executable: `chmod +x diskofflaner`
3. Run with privileges: `sudo ./diskofflaner`

## Usage

- **Set Offline**: Click the "Set Offline" button next to any active disk to safely disconnect it.
- **Set Online**: Click the "Set Online" button to remount a disk.
- **Refresh**: Use the Refresh button in the top panel to update the list after connecting new hardware.
- **Theme**: Toggle between Light and Dark modes using the theme button in the header.

## Building from Source

To build from source, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
# Clone the repository
git clone https://github.com/appsjuragan/diskOfflaner-rust.git

# Navigate to the project folder
cd diskOfflaner-rust

# Build the release version
cargo build --release
```

- **Windows Output**: `target/release/diskofflaner.exe`
- **Linux Output**: `target/release/diskofflaner`

### Code Signing (Windows)

For production releases, the executable should be digitally signed to establish trust and avoid Windows SmartScreen warnings.

```powershell
# Build optimized release
cargo build --release

# Sign with your certificate
.\scripts\sign_release.ps1

# Verify signature
.\scripts\verify_signature.ps1
```

For detailed instructions on certificates and signing, see `scripts/SIGNING_QUICK_REFERENCE.md`.

## Target Audience

- **System Administrators**: Manage multiple drives across workstations efficiently.
- **IT Professionals**: Perform maintenance without rebooting or physical access.
- **Data Managers**: Safely handle large collections of external storage.

## Important Notes

- **Backups**: Always ensure you have backups before performing disk operations.
- **System Disks**: Avoid taking system or boot disks offline unless absolutely necessary.
- **Active Files**: Ensure no applications are actively reading from or writing to a disk before setting it offline.

## License

[MIT License](LICENSE)

---

Built with Rust for performance and reliability.