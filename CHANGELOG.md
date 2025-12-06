# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **USB Auto-Refresh**: Automatically detects when USB drives are connected or disconnected
  - Windows: Uses WM_DEVICECHANGE messages for instant detection
  - Linux: Monitors /dev directory for block device changes
  - Debounced to prevent excessive refreshes (2-second minimum interval)
  - Runs in background thread for non-blocking operation
- Documentation for USB auto-refresh feature (USB_AUTO_REFRESH.md)

## [1.0.3] - 2025-12-06

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
