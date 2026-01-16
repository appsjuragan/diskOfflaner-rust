# DiskOfflaner

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-lightgrey.svg)](https://github.com/appsjuragan/diskOfflaner-rust)
[![Version](https://img.shields.io/badge/version-1.0.5-green.svg)](CHANGELOG.md)

**Safe and simple disk management for Windows and Linux.**

DiskOfflaner is a cross-platform desktop application designed to manage the online/offline status of physical disk drives. It provides a safe, user-friendly interface for system administrators and power users to control disk availability without physical hardware manipulation.

## Features

- **Visual Dashboard**: View all physical disks, models, sizes, and partition information in a clean layout
- **Status Indicators**: Clear visual cues distinguish between Online (Green) and Offline (Red) drives
- **One-Click Control**: Toggle disk status easily without complex command-line tools
- **USB Auto-Detection**: Automatically detects when USB drives are connected or disconnected
- **Improved Disk Detection**: Accurately distinguishes between SSD, HDD, and NVMe drives on Windows using seek penalty checks
- **Safety Mechanisms**:
  - System Protection: Robust identification of critical boot/system disks using volume extents
  - Confirmation: Requires explicit confirmation for sensitive operations
  - Usage Detection: Prevents operations on drives that are currently in use to avoid data loss
- **Modern Interface**: Includes both Dark and Light themes for comfortable usage in any environment
- **Modular GUI**: Refactored component-based architecture for better maintainability and performance
- **Cross-Platform**: Native support for Windows and Linux with platform-optimized disk operations

## System Requirements

### Windows
- Windows 10 or Windows 11
- Administrator privileges (required for disk management)

### Linux
- Modern Linux Distribution (Ubuntu, Fedora, Arch, etc.)
- Root/Sudo privileges (required for disk management)
- `lsblk` utility (pre-installed on most distributions)

## Getting Started

### Windows
1. Download `diskofflaner.exe` from the [Releases](https://github.com/appsjuragan/diskOfflaner-rust/releases) page
2. Right-click the executable and select **Run as administrator**  
3. The application will launch and scan your disks automatically

### Linux
1. Download the Linux binary from the [Releases](https://github.com/appsjuragan/diskOfflaner-rust/releases) page
2. Make the file executable: `chmod +x diskofflaner`
3. Run with privileges: `sudo ./diskofflaner`

Or install with icon support:
```bash
chmod +x scripts/install_linux.sh
sudo ./scripts/install_linux.sh
```

## Usage

- **Set Offline**: Click the "Set Offline" button next to any active disk to safely disconnect it
- **Set Online**: Click the "Set Online" button to remount a disk
- **Refresh**: Use the Refresh button in the top panel to update the list after connecting new hardware
- **Theme**: Toggle between Light and Dark modes using the theme button in the header
- **Partition Management**: View partition details and manage mount/unmount operations (Linux)

## Building from Source

To build from source, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed (version 1.80 or later recommended).

```bash
# Clone the repository
git clone https://github.com/appsjuragan/diskOfflaner-rust.git

# Navigate to the project folder
cd diskOfflaner-rust

# Build the release version
cargo build --release
```

**Output locations:**
- Windows: `target/release/diskofflaner.exe`
- Linux: `target/release/diskofflaner`

### Development

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features

# Run tests
cargo test --all

# Build and run in debug mode
cargo run
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed development guidelines.

## Architecture

DiskOfflaner is built with:
- **Language**: Rust (2021 edition)
- **GUI Framework**: egui/eframe for cross-platform native UI
- **Platform Integration**: 
  - Windows: WinAPI for IOCTL disk operations
  - Linux: Direct system calls and `lsblk` integration

## Important Notes

⚠️ **Safety First**:
- **Backups**: Always ensure you have backups before performing disk operations
- **System Disks**: Avoid taking system or boot disks offline unless absolutely necessary
- **Active Files**: Ensure no applications are actively reading from or writing to a disk before setting it offline
- **Data Loss Prevention**: The application includes safeguards, but user discretion is essential

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and release notes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/appsjuragan/diskOfflaner-rust/issues)
- **Discussions**: Join the conversation in [GitHub Discussions](https://github.com/appsjuragan/diskOfflaner-rust/discussions)

---

Built with ❤️ and Rust for performance, safety, and reliability.