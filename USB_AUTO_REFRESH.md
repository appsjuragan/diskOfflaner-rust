# USB Auto-Refresh Feature

## Overview

DiskOfflaner now automatically detects when USB drives are connected or disconnected and refreshes the disk list automatically, providing a more responsive user experience.

## How It Works

### Windows
On Windows, the application creates a hidden window that listens for `WM_DEVICECHANGE` messages from the operating system. When a device is added or removed, Windows sends a notification, and the GUI auto-refreshes the disk list.

### Linux
On Linux, the application monitors the `/dev` directory for changes to block devices (sd*, nvme*, mmcblk*). It polls every second to detect new or removed devices and triggers a refresh.

## Features

- **Automatic Detection**: No manual refresh needed when plugging/unplugging USB drives
- **Debouncing**: Refreshes are limited to once every 2 seconds to prevent excessive updates
- **Non-Intrusive**: Runs in a background thread, doesn't block the UI
- **Cross-Platform**: Works on both Windows and Linux

## Technical Details

### Architecture
- Background monitoring thread started when GUI launches
- Uses message passing (mpsc channels) to communicate device changes to the main GUI thread
- Respects existing refresh operations to avoid conflicts

### Performance
- Minimal overhead on Windows (event-driven)
- Approximately 1 Hz polling on Linux (acceptable overhead)
- Auto-refresh only triggers if not currently refreshing

## User Experience

When you plug in a USB drive:
1. The system detects the device change
2. After a brief moment (< 2 seconds), theGUI automatically refreshes
3. The new drive appears in the list without manual intervention

When you unplug a USB drive:
1. The system detects the removal
2. The GUI auto-refreshes
3. The removed drive disappears from the list

## Code Structure

### Files Modified
- `src/gui.rs`: Added device monitoring logic and background thread management

### New Functions
- `start_device_monitoring()`: Initializes the background monitor thread
- `monitor_device_changes_windows()`: Windows-specific WinAPI device change detection
- `monitor_device_changes_linux()`: Linux-specific /dev monitoring

### Data Structures
Added to `DiskApp`:
- `device_change_receiver`: Channel to receive device change notifications
- `last_auto_refresh`: Timestamp to implement debouncing
- `device_monitor_active`: Flag to control the monitoring thread lifecycle

## Testing

To test the feature:
1. Launch DiskOfflaner
2. Plug in a USB drive
3. Observe the list automatically refresh
4. Unplug the USB drive
5. Observe the list refresh again

## Future Enhancements

Potential improvements:
- Visual indicator when auto-refresh is triggered
- User preference to enable/disable auto-refresh
- Detection of other device types (SD cards, optical drives)
- Faster detection on Linux using inotify or udev

## Related Files
- `src/gui.rs` - Main implementation
- `Cargo.toml` - Dependencies (requires winapi on Windows)

## Compatibility

- Windows 10/11: Full support
- Linux (Ubuntu, Fedora, etc.): Full support
- Other platforms: Gracefully degrades (no auto-refresh, manual refresh still works)
