import { createSignal, onMount, onCleanup } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { Moon, Sun, ZoomIn, ZoomOut, RotateCw } from "lucide-solid";
import Sidebar from "./components/Sidebar";
import DiskCard from "./components/DiskCard";
import SystemInfo from "./components/SystemInfo";
import Logs from "./components/Logs";
import Tooltip from "./components/Tooltip";
import Modal from "./components/Modal";
import DriveLetterModal from "./components/DriveLetterModal";
import Settings from "./components/Settings";

function App() {
  const [activePage, setActivePage] = createSignal("drives");
  const [theme, setTheme] = createSignal("dark");
  const [disks, setDisks] = createSignal([]);
  const [togglingDiskId, setTogglingDiskId] = createSignal(null);
  const [loading, setLoading] = createSignal(true);
  const [zoomLevel, setZoomLevel] = createSignal(1);
  const [modal, setModal] = createSignal({ show: false, title: "", message: "", onConfirm: null, isDanger: false });
  const [pendingMount, setPendingMount] = createSignal(null);
  const [assignedLetters, setAssignedLetters] = createSignal(new Set());
  const [safeMode, setSafeMode] = createSignal(false);
  const [autoRefresh, setAutoRefresh] = createSignal(true);

  const showConfirm = (config) => {
    setModal({ ...config, show: true });
  };

  const closeModal = () => {
    setModal({ show: false, title: "", message: "", onConfirm: null, isDanger: false });
  };

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
    const action = async () => {
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

    // Only confirm if Safe Mode is on AND we are taking disk Offline
    if (safeMode() && disk.is_online) {
      showConfirm({
        title: "Set Disk Offline",
        message: `Are you sure you want to set Disk ${disk.id} offline?`,
        confirmLabel: "Set Offline",
        isDanger: true,
        onConfirm: action
      });
    } else {
      action();
    }
  };

  const showMountModal = (diskId, partitionNumber) => {
    setPendingMount({ diskId, partitionNumber });
  };

  const closeMountModal = () => {
    setPendingMount(null);
  };

  const mountPartition = async (letter) => {
    const pending = pendingMount();
    if (!pending) return;

    try {
      setTogglingDiskId(pending.diskId);
      setPendingMount(null);
      const assigned = await invoke("mount_partition_command", {
        diskId: pending.diskId,
        partitionNumber: pending.partitionNumber,
        letter: letter
      });
      if (assigned) {
        setAssignedLetters(prev => new Set(prev).add(assigned.toUpperCase()));
      }
      await fetchDisks();
    } catch (error) {
      console.error("Failed to mount partition:", error);
    } finally {
      setTogglingDiskId(null);
    }
  };

  const unmountPartition = async (diskId, driveLetter) => {
    const action = async () => {
      try {
        setTogglingDiskId(diskId);
        const letter = driveLetter.substring(0, 1).toUpperCase();
        await invoke("unmount_partition_command", {
          volumeOrLetter: driveLetter
        });
        setAssignedLetters(prev => {
          const next = new Set(prev);
          next.delete(letter);
          return next;
        });
        await fetchDisks(true);
      } catch (error) {
        console.error("Failed to unmount partition:", error);
      } finally {
        setTogglingDiskId(null);
      }
    };

    if (safeMode()) {
      showConfirm({
        title: "Unmount Partition",
        message: `Are you sure you want to unmount partition ${driveLetter}?`,
        confirmLabel: "Unmount",
        isDanger: false,
        onConfirm: action
      });
    } else {
      action();
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

  const handleClearLogs = async () => {
    showConfirm({
      title: "Clear History",
      message: "Are you sure you want to delete all application logs? This cannot be undone.",
      confirmLabel: "Clear All",
      isDanger: true,
      onConfirm: async () => {
        try {
          await invoke("clear_logs_command");
        } catch (error) {
          console.error("Failed to clear logs:", error);
        }
      }
    });
  };

  onMount(() => {
    fetchDisks();
    const interval = setInterval(() => {
      if (autoRefresh()) fetchDisks(true);
    }, 15000);
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

          {activePage() !== "logs" && activePage() !== "info" && activePage() !== "settings" && (
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
          )}
        </header>

        {activePage() === "drives" && (
          <div class="disk-grid">
            {disks().map((disk) => (
              <DiskCard
                disk={disk}
                onToggle={() => toggleDisk(disk)}
                onMount={(diskId, partNum) => showMountModal(diskId, partNum)}
                onUnmount={(diskId, letter) => unmountPartition(diskId, letter)}
                onOpenExplorer={openExplorer}
                isToggling={togglingDiskId() === disk.id}
                isAnyToggling={togglingDiskId() !== null}
              />
            ))}
          </div>
        )}

        {activePage() === "info" && <SystemInfo />}
        {activePage() === "logs" && <Logs showConfirm={showConfirm} />}
        {activePage() === "settings" && (
          <Settings
            theme={theme()}
            toggleTheme={toggleTheme}
            zoomLevel={zoomLevel()}
            setZoomLevel={setZoomLevel}
            safeMode={safeMode()}
            setSafeMode={setSafeMode}
            autoRefresh={autoRefresh()}
            setAutoRefresh={setAutoRefresh}
            onClearLogs={handleClearLogs}
          />
        )}

        <Modal
          show={modal().show}
          title={modal().title}
          message={modal().message}
          isDanger={modal().isDanger}
          confirmLabel={modal().confirmLabel}
          onConfirm={() => {
            modal().onConfirm && modal().onConfirm();
            closeModal();
          }}
          onCancel={closeModal}
        />

        <DriveLetterModal
          show={pendingMount() !== null}
          assignedLetters={assignedLetters()}
          onConfirm={mountPartition}
          onCancel={closeMountModal}
        />
      </main>
    </div>
  );
}

export default App;
