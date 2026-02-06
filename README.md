# DiskOfflaner v2.0.0

**Safe, Simple, and Reliable Disk Management for Windows.**

DiskOfflaner is a modern utility built with **Rust** and **Tauri** designed to help you strictly control your physical drives. Easily mount, unmount, and toggle disks offline/online with a safety-first approach.

![DiskOfflaner Screenshot](https://raw.githubusercontent.com/AppsJuragan/diskofflaner/main/screenshot.png)

## 🚀 Features

- **🛡️ Safe Mode**: Prevent accidental data loss by requiring confirmation before critical actions (Offline, Unmount).
- **🔌 Toggle Disk Status**: Instantly switch disks between **current Online** and **Offline** states.
- **📂 Mount & Unmount**: Assign drive letters (Auto or Manual) and unmount partitions on the fly.
- **📊 Detailed System Info**: View comprehensive hardware details including Model, Serial Number, and SMART health prediction.
- **🎨 Modern UI**: Beautiful interface with **Dark/Light** themes, adjustable **Zoom**, and smooth animations.
- **⚡ High Performance**: Built on Rust for near-instant startup and minimal resource usage.

## 🛠️ Tech Stack

- **Backend**: Rust (Tauri v2)
- **Frontend**: SolidJS + Vite
- **Styling**: Vanilla CSS (Variables & Glassmorphism)
- **Icons**: Lucide Icons

## 📦 Installation

Download the latest release from the [Releases Page](https://github.com/AppsJuragan/diskofflaner/releases).

**Portable**: No installation required. Just run `diskofflaner.exe`.

## 💻 Development

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) or [Bun](https://bun.sh/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Build from Source

1.  **Clone the repository**
    ```bash
    git clone https://github.com/AppsJuragan/diskofflaner.git
    cd diskofflaner
    ```

2.  **Install dependencies**
    ```bash
    bun install
    ```

3.  **Run in Development Mode**
    ```bash
    bun tauri dev
    ```

4.  **Build Release**
    ```bash
    bun tauri build --no-bundle
    ```

## 📝 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

**Author**: [AppsJuragan](https://github.com/AppsJuragan)