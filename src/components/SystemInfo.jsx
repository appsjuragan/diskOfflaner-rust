import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

const SystemInfo = () => {
  const [info, setInfo] = createSignal(null);
  const [loading, setLoading] = createSignal(true);

  const fetchInfo = async () => {
    try {
      const result = await invoke("get_system_info_command");
      setInfo(result);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  onMount(fetchInfo);

  const formatBytes = (bytes) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  return (
    <div class="info-container">
      {loading() ? (
        <div class="loading">Gathering system info...</div>
      ) : info() ? (
        <div class="info-grid">
          <div class="info-card">
            <h3>Operating System</h3>
            <p class="value">{info().os_name}</p>
            <p class="sub">{info().os_version}</p>
          </div>
          <div class="info-card">
            <h3>Privileges</h3>
            <p class="value">{info().is_admin ? "Administrator" : "Standard User"}</p>
            <p class="sub">{info().is_admin ? "Full Access" : "Read-only access"}</p>
          </div>
          <div class="info-card">
            <h3>Storage Summary</h3>
            <p class="value">{info().total_disks} Physical Disks</p>
            <p class="sub">Total Capacity: {formatBytes(info().total_capacity_bytes)}</p>
          </div>
          <div class="info-card">
            <h3>Boot Drive</h3>
            <p class="value">{info().system_disk_id !== null ? `Disk ${info().system_disk_id}` : "Unknown"}</p>
            <p class="sub">Primary system drive</p>
          </div>
        </div>
      ) : (
        <div class="error">Failed to load system info.</div>
      )}
    </div>
  );
};

export default SystemInfo;
