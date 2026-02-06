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
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + sizes[i];
  };

  const getDiskTypeLabel = () => {
    switch (disk.disk_type) {
      case "NVMe": return "NVMe";
      case "SSD": return "SSD";
      case "HDD": return "HDD";
      case "USBFlash": return "USB Drive";
      case "ExtHDD": return "External HDD";
      default: return "Unknown";
    }
  };

  const getIconColor = () => {
    if (props.isToggling) return "var(--text-tertiary)";
    return disk.is_online ? "var(--status-online)" : "var(--status-offline)";
  };

  const getHealthColor = () => {
    if (disk.health_percentage === null || disk.health_percentage === undefined) {
      return "var(--health-warning)";
    }
    if (disk.health_percentage >= 90) return "var(--health-good)";
    if (disk.health_percentage >= 70) return "var(--health-warning)";
    return "var(--health-critical)";
  };

  const Icon = getIcon();
  const isUsb = disk.disk_type === "USBFlash" || disk.disk_type === "ExtHDD";

  return (
    <div
      class={`disk-card ${props.isToggling ? "loading" : ""}`}
      data-status={disk.is_online ? "online" : "offline"}
    >
      <header class="card-header">
        <div class="card-title">
          <div class="icon-wrapper" style={{ color: getIconColor() }}>
            <Icon size={24} />
          </div>
          <span class="disk-name" data-tooltip={disk.model}>Disk {disk.id}</span>
        </div>
        <button
          class={`status-badge ${disk.is_online ? "online" : "offline"}`}
          disabled={props.isToggling || props.isAnyToggling}
          onClick={(e) => { e.stopPropagation(); props.onToggle && props.onToggle(); }}
          data-tooltip={disk.is_online ? "Click to set Offline" : "Click to set Online"}
        >
          {props.isToggling ? (
            <div class="spinner"></div>
          ) : (
            disk.is_online ? "ONLINE" : "OFFLINE"
          )}
        </button>
      </header>

      <div class="card-body two-column">
        <div class="disk-info">
          <div class="info-row">
            <span class="label">Model</span>
            <span class="value">{getDiskTypeLabel()}</span>
          </div>
          <div class="info-row">
            <span class="label">Capacity</span>
            <span class="value">{formatBytes(disk.size_bytes)}</span>
          </div>
          <div class="info-row">
            <span class="label">Health</span>
            <span class="value health" style={{ color: getHealthColor() }}>
              {disk.health_percentage !== null && disk.health_percentage !== undefined ? `${disk.health_percentage}%` : "N/A"}
            </span>
          </div>
          <div class="info-row stacked">
            <span class="label">Serial</span>
            <span class="value serial">{disk.serial_number || "N/A"}</span>
          </div>
        </div>

        <div class="partition-list">
          <div class="partition-header">Partition:</div>
          {disk.partitions && disk.partitions.length > 0 ? (
            disk.partitions.map((partition) => (
              <div class="partition-row" key={partition.partition_id}>
                <span
                  class={`partition-info ${partition.drive_letter ? "link" : ""}`}
                  onClick={(e) => {
                    if (partition.drive_letter) {
                      e.stopPropagation();
                      props.onOpenExplorer && props.onOpenExplorer(partition.drive_letter);
                    }
                  }}
                  data-tooltip={partition.drive_letter ? "Open in Explorer" : ""}
                >
                  [{partition.drive_letter || "?"}:\] - {formatBytes(partition.size_bytes)}
                </span>
                {partition.drive_letter ? (
                  <button
                    class={`partition-btn ${isUsb ? "eject" : "mounted"}`}
                    onClick={(e) => {
                      e.stopPropagation();
                      if (isUsb) {
                        props.onUnmount && props.onUnmount(disk.id, partition.drive_letter);
                      } else {
                        props.onOpenExplorer && props.onOpenExplorer(partition.drive_letter);
                      }
                    }}
                    onContextMenu={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      props.onUnmount && props.onUnmount(disk.id, partition.drive_letter);
                    }}
                    data-tooltip={isUsb ? "Safely Remove (Eject)" : "Open in Explorer (Right-click to Unmount)"}
                  >
                    {isUsb ? "Eject" : "Mounted"}
                  </button>
                ) : (
                  <button
                    class="partition-btn unmounted"
                    disabled={props.isAnyToggling || !disk.is_online}
                    onClick={(e) => {
                      e.stopPropagation();
                      props.onMount && props.onMount(disk.id, partition.partition_number);
                    }}
                    data-tooltip={disk.is_online ? "Click to mount" : "Disk is offline"}
                  >
                    Mount
                  </button>
                )}
              </div>
            ))
          ) : (
            <div class="no-partitions">No partitions</div>
          )}
        </div>
      </div>
    </div>
  );
}

export default DiskCard;
