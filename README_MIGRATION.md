# DiskOfflaner - Tauri v2 Migration

This project has been migrated to Tauri v2 with a SolidJS + Bun frontend.

## Prerequisites

- **Bun**: [Install Bun](https://bun.sh/)
- **Rust**: [Install Rust](https://rustup.rs/) (v1.77+)
- **Build Tools**: Visual Studio Build Tools with C++ workload (Windows)

## Setup

1. Install frontend dependencies:
   ```bash
   bun install
   ```

2. Run the development server (Frontend + Backend):
   ```bash
   bun run tauri dev
   ```

3. Build for production:
   ```bash
   bun run tauri build
   ```

## Project Structure

- `src-tauri/`: Rust backend (Tauri v2)
  - `src/lib.rs`: Tauri commands and setup
  - `src/disk_operations/`: Disk management logic (Windows/Linux)
- `src/`: SolidJS frontend
  - `components/`: UI components (DiskCard, Sidebar)
  - `styles/`: CSS variables and global styles
  - `App.jsx`: Main application logic

## New Features (Migration)

- **UI**: Modern glassmorphic design matching the style guide.
- **Frontend**: SolidJS for high performance and reactivity.
- **Backend**: Tauri v2 for better security and modularity.
- **Data**: 
  - **Serial Number**: Real-time extraction via `IOCTL_STORAGE_QUERY_PROPERTY` (Windows) and `lsblk` (Linux).
  - **Health Percentage**: Basic health check via `IOCTL_STORAGE_PREDICT_FAILURE` (Windows). Returns 100% (Healthy) or 10% (Critical). Linux health check pending integration.


## Troubleshooting

- If `bun install` fails on Windows, ensure you have the latest Bun version.
- If backend fails to build, check `src-tauri/build_log.txt` or run `cargo check` inside `src-tauri`.
- Ensure you have Administrator privileges when running the app for disk operations.
