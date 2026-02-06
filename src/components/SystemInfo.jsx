import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { HardDrive, Github } from "lucide-solid";

const SystemInfo = () => {
  const [info, setInfo] = createSignal(null);
  const [disks, setDisks] = createSignal([]);
  const [loading, setLoading] = createSignal(true);

  const fetchInfo = async () => {
    try {
      const [sysInfo, disksList] = await Promise.all([
        invoke("get_system_info_command"),
        invoke("enumerate_disks_command")
      ]);
      setInfo(sysInfo);
      setDisks(disksList);
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

  const openGithub = async () => {
    await invoke("open_url_command", { url: "https://github.com/AppsJuragan" });
  };

  return (
    <div class="info-container">
      {loading() ? (
        <div class="loading">Gathering system info...</div>
      ) : info() ? (
        <>
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
              <h3>Boot Drive</h3>
              <p class="value">{info().system_disk_id !== null ? `Disk ${info().system_disk_id}` : "Unknown"}</p>
              <p class="sub">Primary system drive</p>
            </div>
            <div class="info-card">
              <h3>Software Information</h3>
              <p class="value" style="font-size: 16px; margin-bottom: 2px;">DiskOfflaner v2.0.0</p>
              <p class="sub" style="margin-bottom: 8px;">© 2026 DiskOfflaner Contributors</p>
              <div style="border-top: 1px solid var(--border-sidebar); padding-top: 8px; margin-top: 8px;">
                <p class="sub">License: <span style="color: var(--text-primary);">MIT License</span></p>
                <div class="sub" style="display: flex; align-items: center; gap: 4px;">
                  Author:
                  <a
                    href="https://github.com/AppsJuragan"
                    target="_blank"
                    style="display: flex; align-items: center; gap: 6px; color: var(--text-primary); text-decoration: none; cursor: pointer;"
                    onClick={(e) => { e.preventDefault(); openGithub(); }}
                  >
                    <Github size={14} />
                    <span>AppsJuragan</span>
                  </a>
                </div>
              </div>
            </div>
          </div>

          <div class="disk-details-container">
            <div class="details-header" style={{ "justify-content": "space-between" }}>
              <div style="display: flex; align-items: center; gap: 12px;">
                <HardDrive size={20} class="text-secondary" />
                <span class="details-title">Physical Disk Inventory</span>
              </div>
              <div style="text-align: right;">
                <span style="font-size: 14px; font-weight: 600; color: var(--text-primary); margin-right: 12px;">
                  {info().total_disks} Disks
                </span>
                <span style="font-size: 14px; color: var(--text-secondary);">
                  {formatBytes(info().total_capacity_bytes)} Total
                </span>
              </div>
            </div>
            <table class="disk-table">
              <thead>
                <tr>
                  <th class="col-id">Disk #</th>
                  <th>Model Name</th>
                  <th class="col-type">Type</th>
                  <th class="col-size">Capacity</th>
                  <th class="col-serial">Serial Number</th>
                </tr>
              </thead>
              <tbody>
                <For each={disks()}>
                  {(disk) => (
                    <tr>
                      <td class="col-id">Disk {disk.id}</td>
                      <td class="col-model">{disk.model}</td>
                      <td class="col-type">{disk.disk_type}</td>
                      <td class="col-size">{formatBytes(disk.size_bytes)}</td>
                      <td class="col-serial">{disk.serial_number || "N/A"}</td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </div>
        </>
      ) : (
        <div class="error">Failed to load system info.</div>
      )}
    </div>
  );
};

export default SystemInfo;
