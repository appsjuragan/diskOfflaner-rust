# DiskOfflaner

A fast and simple command-line utility for Windows that allows you to list all physical disks and partitions, and toggle their Online/Offline state. Written in Rust for performance and safety.

## Features

- List all available disks with size, health, and online/offline status
- Show partitions under each disk in a tree-like view
- Toggle disk online/offline state with a single command
- Warn before changing the state of critical system disks
- Colorized console output for easier reading
- Supports both interactive mode and command-line arguments
- Built-in administrator privilege checking

## Requirements

- Windows 10, Windows 11, or Windows Server
- Administrator privileges (required for disk management operations)
- Rust 1.70+ (for building from source)

## Installation

### Building from Source

1. Clone this repository:
```
git clone https://github.com/appsjuragan/diskOfflaner.git
cd diskOfflaner
```

2. Build the release version:
```
cargo build --release
```
3. The executable will be located at:
```
target\release\diskofflaner.exe
```
## Usage

**Important:** This program requires administrator privileges. Always run as administrator.

### Interactive Mode

Run without arguments to enter interactive mode:
```
diskofflaner.exe
```
Output:
```
DiskOfflaner - Disk Management Tool
====================================

Disk 0: Disk 0 - Online - Health: 0 - Size: 9314.00 GB
   └─ Partition 0: 5273.63 GB (GPT: 100000) [G:]
   └─ Partition 0: 4040.37 GB (GPT: 52668200000) [H:]
Disk 1: Disk 1 - Online - Health: 0 - Size: 9314.00 GB
   └─ Partition 0: 6205.44 GB (GPT: 100000) [E:]
   └─ Partition 0: 3108.56 GB (GPT: 60F5C400000) [F:]
Disk 2: Disk 2 - Offline - Health: 0 - Size: 11176.00 GB
Disk 3: Disk 3 - Online - Health: 0 - Size: 465.76 GB
   └─ Partition 0: 464.92 GB (GPT: 7500000) [C:]

Enter disk number to toggle (or 'q' to quit): 
```
### Direct Command Mode

Toggle a specific disk directly by providing the disk number as an argument:
```
diskofflaner.exe 2
```
Output:
```
Checking disk 2 status...
Disk 2 is currently Offline. Bringing it Online...
Disk is now Online.
```
## Running as Administrator

### Method 1: Right-Click
1. Navigate to the executable location
2. Right-click on diskofflaner.exe
3. Select "Run as administrator"

### Method 2: PowerShell/CMD as Admin
1. Press Win + X and select "Windows PowerShell (Admin)"
2. Navigate to the directory:
```
cd path\to\diskofflaner
```
3. Run the program:
```
.\diskofflaner.exe
```
### Method 3: Run Dialog
1. Press Win + R
2. Type powershell and press Ctrl + Shift + Enter
3. Navigate and run the executable

## Safety Notes

**Use with caution!**

- Taking a critical or system disk offline can cause Windows to become unstable
- The program will warn you before modifying system disks (typically Disk 0)
- Always confirm when prompted about critical disk operations
- Offline disks become inaccessible until brought back online

## How It Works

DiskOfflaner uses Windows API calls through the winapi crate to:
- Query physical disk information via IOCTL_DISK_GET_DRIVE_GEOMETRY_EX
- Enumerate volume extents with IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS
- Execute diskpart commands to toggle disk online/offline status
- Check administrator privileges using Windows security tokens

## Dependencies

- winapi - Windows API bindings
- colored - Terminal color output
- anyhow - Error handling

## Project Structure
```
diskofflaner/
├── src/
│   ├── main.rs              # Main entry point and UI
│   ├── disk_operations.rs   # Disk enumeration and management
│   └── structs.rs           # Data structures
├── Cargo.toml
├── .gitignore
└── README.md
```
## License
```
This project is a Rust port of the original .NET diskOfflaner (https://github.com/appsjuragan/diskOfflaner) by appsjuragan.
```
## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Troubleshooting

### "Access Denied" Error
Make sure you're running the program as administrator. Disk management operations require elevated privileges.

### Disk Not Found
Verify the disk number using Windows Disk Management (diskmgmt.msc) or diskpart.

### Program Doesn't Detect Offline Disks Correctly
The program uses diskpart to check disk status. Ensure diskpart is available in your system PATH.

## Acknowledgments
```
- Original .NET implementation: appsjuragan/diskOfflaner (https://github.com/appsjuragan/diskOfflaner)
- Rust Windows API bindings: winapi-rs (https://github.com/retep998/winapi-rs)
```