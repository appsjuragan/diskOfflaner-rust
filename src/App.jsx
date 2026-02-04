import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/Sidebar";
import DiskCard from "./components/DiskCard";

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
      </main>

      <style jsx>{`
        .app-container {
          display: flex;
          height: 100vh;
          background-color: var(--bg-app);
          color: var(--text-primary);
        }

        .main-content {
          flex: 1;
          padding: 32px 40px;
          overflow-y: auto;
          background-color: var(--bg-main);
        }

        .main-header {
          margin-bottom: 32px;
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .main-title {
          font-size: 32px;
          font-weight: 700;
          letter-spacing: -0.02em;
        }

        .refresh-btn {
          padding: 8px 16px;
          background: var(--bg-card);
          border: 1px solid var(--border-card);
          color: var(--text-primary);
          border-radius: 8px;
          transition: all 0.2s;
        }

        .refresh-btn:hover:not(:disabled) {
          background: var(--bg-card-hover);
        }

        .disk-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(380px, 1fr));
          gap: 24px;
        }

        @media (max-width: 879px) {
          .disk-grid {
            grid-template-columns: 1fr;
          }
        }
      `}</style>
    </div>
  );
}

export default App;
