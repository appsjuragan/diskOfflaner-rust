# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.8] - 2026-01-24

### Added
- **Partition Management**: Added Mount/Unmount buttons for all disk partitions.
- **Drive Letter Selection**: Added a dialog to select a specific drive letter (or Auto) when mounting a partition.
- **Unit Tests**: Added comprehensive unit tests for data structures (`DiskInfo`, `DiskType`, `PartitionInfo`).
- **USB Auto-Refresh**: Automatically detects when USB drives are connected or disconnected.
  - Windows: Uses WM_DEVICECHANGE messages for instant detection.
  - Linux: Monitors /dev directory for block device changes.
  - Debounced to prevent excessive refreshes.
- **Application Icon**: Professional custom icon representing disk management.

### Changed
- **UI Improvements**:
  - Moved partition action buttons to the right for better alignment.
  - Added distinct blue color for mounted partition labels.
  - Disabled partition actions when the disk is offline.
- **Error Handling**: Enhanced error messages for failed unmount operations (e.g., Pagefile, System Volume, In Use).
- **Code Refactoring**: Centralized theme configuration into `src/gui/themes.rs` to eliminate duplication.
- **Type Safety**: Improved `DiskAction` enum with named struct variants for clearer code intent.
- **Maintainability**: Replaced magic numbers with named constants in Windows disk operations.
- **UI**: Improved `DiskType` display with human-readable names.

### Fixed
- Fixed clippy warnings in build script.


## [1.0.5] - 2025-12-06

### Fixed
- **Windows USB Detection**: Fixed an issue where USB drive connection/disconnection events were not correctly triggering a refresh due to incorrect message parameter handling in the window procedure.

## [1.0.4] - 2025-12-06

### Added
- Release quality assurance and optimization
- Comprehensive code linting and formatting
- MIT License file
- Changelog for version tracking
- Enhanced documentation

### Changed
- Applied cargo fmt to all source files
- Verified clippy compliance with zero warnings
- Optimized release build configuration

### Fixed
- Code formatting consistency across all modules
- Documentation improvements

## [1.0.2] - Previous Release

### Added
- Linux support with platform-specific disk operations
- Cross-platform disk identifier handling (numeric for Windows, string for Linux)

### Changed
- Refactored disk_operations module for platform independence
- Updated DiskInfo struct for cross-platform compatibility

## [1.0.0] - Initial Release

### Added
- Windows disk management functionality
- Online/Offline disk status control
- System disk protection
- Dark/Light theme support
- Visual dashboard for disk management
- Partition information display
- Confirmation dialogs for critical operations
- Error notification system
