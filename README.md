# 💾 DiskOfflaner

**Take control of your Windows disk drives with confidence and ease.**

DiskOfflaner is a powerful yet intuitive desktop application that lets you manage the online/offline status of your physical disk drives on Windows. Whether you're a system administrator, IT professional, or power user who needs to safely disconnect drives without physically unplugging them, DiskOfflaner makes it simple and safe.

## ✨ Why Use DiskOfflaner?

Ever needed to temporarily disable a disk drive without physically removing it? DiskOfflaner gives you that power with a beautiful, user-friendly interface that keeps your system safe from accidental mishaps.

## 🎯 Key Features

### 📊 **Crystal Clear Dashboard**
See all your physical disks at a glance with their models, sizes, and partition information beautifully displayed in an easy-to-read interface.

### 🚦 **Instant Status Visibility**
Color-coded indicators (Green for Online, Red for Offline) make it immediately obvious which drives are active and which are disabled.

### 🔄 **Smart Refresh System**
- **Always Visible**: Your disk list stays on screen during refreshes - no annoying blank screens
- **Visual Feedback**: Subtle greying effect lets you know when data is updating
- **One-Click Update**: Conveniently placed refresh button (⟳) in the top bar for instant access

### ⚡ **Simple One-Click Controls**
Toggle any disk between Online and Offline states with a single button click. No command-line expertise required!

### 🛡️ **Safety-First Design**
- **🔴 System Disk Protection**: Critical boot and system disks are clearly highlighted with warning labels
- **⚠️ Confirmation Dialogs**: Extra confirmation required before taking system disks offline, preventing catastrophic mistakes
- **🚫 Smart Detection**: Automatically detects when a disk is in use and prevents dangerous operations that could cause data loss

### 🎨 **Modern Interface**
- **Theme Options**: Switch between Dark Mode (default) and Light Mode to match your preference
- **Responsive Design**: All operations run in the background, keeping the interface smooth and snappy
- **Clean Layout**: Intuitive design that doesn't require a manual to understand

## 💻 System Requirements

- **Windows 10** or **Windows 11**
- **Administrator privileges** (required for disk management operations)

## 🚀 Getting Started

### Quick Start

1. **Download** the latest `diskofflaner.exe` from the [Releases](https://github.com/appsjuragan/diskOfflaner-rust/releases) page
2. **Right-click** the executable and select **"Run as administrator"**
3. **You're ready!** The application will automatically scan and display all your disks

### How to Use

- **📴 Take a disk offline**: Click the "Set Offline" button next to any Online disk
- **📳 Bring a disk online**: Click the "Set Online" button next to any Offline disk  
- **🔄 Refresh the list**: Click the "⟳ Refresh" button in the top panel (useful after connecting new drives)
- **🌙 Change theme**: Click "Light Mode" or "Dark Mode" button to switch themes

## 🔧 Building from Source

Want to build it yourself? You'll need [Rust](https://www.rust-lang.org/tools/install) installed on your system.

```bash
# Clone the repository
git clone https://github.com/appsjuragan/diskOfflaner-rust.git

# Navigate to the project folder
cd diskOfflaner-rust

# Build the release version
cargo build --release
```

Your executable will be ready at `target/release/diskofflaner.exe`

## 🤝 Who Should Use This?

- **System Administrators**: Managing multiple drives across workstations
- **IT Professionals**: Quick disk management without rebooting
- **Power Users**: Advanced disk control for multi-drive setups
- **Data Hoarders**: Safely manage large collections of external drives
- **Anyone**: Who needs safe, simple disk drive control

## ⚠️ Important Notes

- Always ensure you have backups before performing disk operations
- System/boot disks should only be taken offline if you know what you're doing
- Make sure no applications are accessing a disk before setting it offline

## 📄 License

[MIT License](LICENSE) - Free and open source!

---

**Made with ❤️ using Rust** - Built for speed, safety, and reliability.