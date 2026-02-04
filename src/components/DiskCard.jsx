import { HardDrive, Usb, Database } from "lucide-solid";

function DiskCard(props) {
  const { disk } = props;

  const getIcon = () => {
    switch (disk.disk_type) {
      case "NVMe": return HardDrive;
      case "SSD": return HardDrive;
      case "USBFlash": return Usb;
      case "HDD": return Database;
      case "ExtHDD": return Usb;
      default: return HardDrive;
    }
  };

  const formatBytes = (bytes) => {
    if (!bytes) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  const getIconColor = () => {
    switch (disk.disk_type) {
      case "NVMe": return "var(--icon-nvme)";
      case "SSD": return "var(--icon-ssd)";
      case "USBFlash": return "var(--icon-usb)";
      case "HDD": return "var(--icon-hdd)";
      default: return "var(--text-secondary)";
    }
  };

  const Icon = getIcon();

  return (
    <div class="disk-card" data-status={disk.is_online ? "online" : "offline"}>
      <header class="card-header">
        <div class="card-title">
          <div class="icon-wrapper" style={{ color: getIconColor() }}>
            <Icon size={24} />
          </div>
          <span class="disk-name" title={disk.model}>{disk.model}</span>
        </div>
        <button
          class={`status-badge ${disk.is_online ? "online" : "offline"}`}
          onClick={(e) => { e.stopPropagation(); props.onToggle && props.onToggle(); }}
          title={disk.is_online ? "Click to set Offline" : "Click to set Online"}
        >
          {disk.is_online ? "ONLINE" : "OFFLINE"}
        </button>
      </header>

      <div class="card-body">
        <div class="info-row">
          <span class="label">Model</span>
          <span class="value">{disk.model}</span>
        </div>
        <div class="info-row">
          <span class="label">Capacity</span>
          <span class="value">{formatBytes(disk.size_bytes)}</span>
        </div>
        <div class="info-row">
          <span class="label">Health</span>
          <span class="value health" style={{ color: (disk.health_percentage || 100) >= 70 ? "var(--health-good)" : "var(--health-critical)" }}>
            {disk.health_percentage !== null ? `${disk.health_percentage}%` : "N/A"}
          </span>
        </div>
        <div class="info-row">
          <span class="label">Serial</span>
          <span class="value serial">{disk.serial_number || "N/A"}</span>
        </div>
      </div>
    </div>
  );
}

export default DiskCard;
