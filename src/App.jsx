import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar";
import DiskCard from "./components/DiskCard";
import SystemInfo from "./components/SystemInfo";

function App() {
  const [activePage, setActivePage] = createSignal("drives");
  const [theme, setTheme] = createSignal("dark");
  const [disks, setDisks] = createSignal([]);
  const [loading, setLoading] = createSignal(true);

  const fetchDisks = async () => {
    try {
      const result = await invoke("enumerate_disks_command");
      console.log("Disks fetched:", result);
      setDisks(result);
    } catch (error) {
      console.error("Failed to fetch disks:", error);
    } finally {
      setLoading(false);
    }
  };

  const toggleDisk = async (disk) => {
    try {
      setLoading(true);
      if (disk.is_online) {
        await invoke("set_disk_offline_command", { diskId: disk.id });
      } else {
        await invoke("set_disk_online_command", { diskId: disk.id });
      }
      await fetchDisks();
    } catch (error) {
      console.error("Failed to toggle disk:", error);
      setLoading(false);
    }
  };

  onMount(() => {
    fetchDisks();
  });

  return (
    <div class="app-container" data-theme={theme()}>
      <Sidebar activePage={activePage()} setActivePage={setActivePage} />

      <main class="main-content">
        <header class="main-header">
          <h1 class="main-title">
            {activePage() === "drives" && "Dashboard"}
            {activePage() === "settings" && "Settings"}
            {activePage() === "logs" && "Logs"}
            {activePage() === "info" && "System Info"}
          </h1>
          <button class="refresh-btn" onClick={fetchDisks} disabled={loading()}>
            Refresh
          </button>
        </header>

        {activePage() === "drives" && (
          <div class="disk-grid">
            {loading() && disks().length === 0 ? (
              <div class="loading">Loading disks...</div>
            ) : (
              disks().map((disk) => (
                <DiskCard disk={disk} onToggle={() => toggleDisk(disk)} />
              ))
            )}
          </div>
        )}

        {activePage() === "info" && <SystemInfo />}
      </main>
    </div>
  );
}

export default App;
