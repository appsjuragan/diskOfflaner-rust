import { createSignal, onMount, onCleanup } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { Moon, Sun, ZoomIn, ZoomOut, RotateCw } from "lucide-solid";
import Sidebar from "./components/Sidebar";
import DiskCard from "./components/DiskCard";
import SystemInfo from "./components/SystemInfo";
import Tooltip from "./components/Tooltip";

function App() {
  const [activePage, setActivePage] = createSignal("drives");
  const [theme, setTheme] = createSignal("dark");
  const [disks, setDisks] = createSignal([]);
  const [togglingDiskId, setTogglingDiskId] = createSignal(null);
  const [loading, setLoading] = createSignal(true);
  const [zoomLevel, setZoomLevel] = createSignal(1);

  const fetchDisks = async (silent = false) => {
    if (!silent) setLoading(true);
    try {
      const result = await invoke("enumerate_disks_command");
      if (!silent) console.log("Disks fetched:", result);
      setDisks(result);
    } catch (error) {
      console.error("Failed to fetch disks:", error);
    } finally {
      if (!silent) setLoading(false);
      // Hide the initial HTML loader after first load
      const initialLoader = document.getElementById("initial-loader");
      if (initialLoader) {
        initialLoader.classList.add("hidden");
        setTimeout(() => initialLoader.remove(), 300);
      }
    }
  };

  const toggleDisk = async (disk) => {
    try {
      setTogglingDiskId(disk.id);
      if (disk.is_online) {
        await invoke("set_disk_offline_command", { diskId: disk.id });
      } else {
        await invoke("set_disk_online_command", { diskId: disk.id });
      }
      await fetchDisks();
    } catch (error) {
      console.error("Failed to toggle disk:", error);
    } finally {
      setTogglingDiskId(null);
    }
  };

  const mountPartition = async (diskId, partitionNumber) => {
    try {
      setTogglingDiskId(diskId);
      await invoke("mount_partition_command", {
        diskId: diskId,
        partitionNumber: partitionNumber,
        letter: null
      });
      await fetchDisks();
    } catch (error) {
      console.error("Failed to mount partition:", error);
    } finally {
      setTogglingDiskId(null);
    }
  };

  const unmountPartition = async (diskId, driveLetter) => {
    try {
      setTogglingDiskId(diskId);
      await invoke("unmount_partition_command", {
        volumeOrLetter: driveLetter
      });
      await fetchDisks(true);
    } catch (error) {
      console.error("Failed to unmount partition:", error);
    } finally {
      setTogglingDiskId(null);
    }
  };

  const toggleTheme = () => {
    setTheme(t => t === "dark" ? "light" : "dark");
  };

  const adjustZoom = (delta) => {
    setZoomLevel(z => Math.max(0.7, Math.min(1.5, z + delta)));
  };

  const openExplorer = async (driveLetter) => {
    try {
      const path = `${driveLetter}:\\`;
      await invoke("open_file_explorer_command", { path });
    } catch (error) {
      console.error("Failed to open explorer:", error);
    }
  };

  onMount(() => {
    fetchDisks();
    const interval = setInterval(() => fetchDisks(true), 15000);
    onCleanup(() => clearInterval(interval));
  });

  return (
    <div class="app-container" data-theme={theme()} style={{ "--card-scale": zoomLevel() }}>
      <Tooltip />

      <Sidebar activePage={activePage()} setActivePage={setActivePage} />

      <main class="main-content">
        <header class="main-header">
          <h1 class="main-title">
            {activePage() === "drives" && "Dashboard"}
            {activePage() === "settings" && "Settings"}
            {activePage() === "logs" && "Logs"}
            {activePage() === "info" && "System Info"}
          </h1>

          <div class="header-actions">
            <div class="action-group">
              <button class="icon-btn" onClick={() => adjustZoom(-0.1)} data-tooltip="Zoom Out">
                <ZoomOut size={20} />
              </button>
              <button class="icon-btn" onClick={() => adjustZoom(0.1)} data-tooltip="Zoom In">
                <ZoomIn size={20} />
              </button>
            </div>

            <button class="icon-btn" onClick={toggleTheme} data-tooltip="Toggle Theme">
              {theme() === "dark" ? <Sun size={20} /> : <Moon size={20} />}
            </button>

            <button
              class="refresh-btn"
              onClick={fetchDisks}
              disabled={loading() || togglingDiskId() !== null}
              data-tooltip="Refresh Disk List"
            >
              <RotateCw size={18} class={loading() ? "spin" : ""} />
              <span>Refresh</span>
            </button>
          </div>
        </header>

        {activePage() === "drives" && (
          <div class="disk-grid">
            {disks().map((disk) => (
              <DiskCard
                disk={disk}
                onToggle={() => toggleDisk(disk)}
                onMount={(diskId, partNum) => mountPartition(diskId, partNum)}
                onUnmount={(diskId, letter) => unmountPartition(diskId, letter)}
                onOpenExplorer={openExplorer}
                isToggling={togglingDiskId() === disk.id}
                isAnyToggling={togglingDiskId() !== null}
              />
            ))}
          </div>
        )}

        {activePage() === "info" && <SystemInfo />}
      </main>
    </div>
  );
}

export default App;
