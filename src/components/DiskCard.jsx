import { HardDrive, Usb, Database } from "lucide-solid";

function DiskCard(props) {
  const getIcon = () => {
    switch (props.disk.disk_type) {
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
    switch (props.disk.disk_type) {
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
    return props.disk.is_online ? "var(--status-online)" : "var(--status-offline)";
  };

  const getHealthColor = () => {
    if (props.disk.health_percentage === null || props.disk.health_percentage === undefined) {
      return "var(--health-warning)";
    }
    if (props.disk.health_percentage >= 90) return "var(--health-good)";
    if (props.disk.health_percentage >= 70) return "var(--health-warning)";
    return "var(--health-critical)";
  };

  const Icon = getIcon();
  const isUsb = () => props.disk.disk_type === "USBFlash";
  const canEject = () => props.disk.disk_type === "USBFlash" || props.disk.disk_type === "ExtHDD";

  return (
    <div
      class={`disk-card ${props.isToggling ? "loading" : ""}`}
      data-status={props.disk.is_online ? "online" : "offline"}
    >
      <header class="card-header">
        <div class="card-title">
          <div class="icon-wrapper" style={{ color: getIconColor() }}>
            <Icon size={24} />
          </div>
          <span class="disk-name">Disk {props.disk.id} - {props.disk.model}</span>
        </div>
        {!isUsb() && props.isAdmin && (
          <button
            class={`status-badge ${props.disk.is_online ? "online" : "offline"}`}
            disabled={props.isToggling || props.isAnyToggling || !props.isAdmin}
            style={!props.isAdmin ? { opacity: 1, cursor: "default" } : {}}
            onClick={(e) => {
              if (!props.isAdmin) return;
              e.stopPropagation();
              props.onToggle && props.onToggle();
            }}
            data-tooltip={
              !props.isAdmin
                ? "Administrator privileges required"
                : (props.disk.is_online ? "Click to set Offline" : "Click to set Online")
            }
          >
            {props.isToggling ? (
              <div class="spinner"></div>
            ) : (
              props.disk.is_online ? "ONLINE" : "OFFLINE"
            )}
          </button>
        )}
        {!isUsb() && !props.isAdmin && (
          <div class={`status-badge ${props.disk.is_online ? "online" : "offline"}`}
            style={{ opacity: 1, cursor: "not-allowed" }}
            data-tooltip="Administrator privileges required">
            {props.disk.is_online ? "ONLINE" : "OFFLINE"}
          </div>
        )}
      </header>

      <div class="card-body two-column">
        <div class="disk-info">
          <div class="info-row">
            <span class="label">Type</span>
            <span class="value">{getDiskTypeLabel()}</span>
          </div>
          <div class="info-row">
            <span class="label">Capacity</span>
            <span class="value">{formatBytes(props.disk.size_bytes)}</span>
          </div>
          <div class="info-row">
            <span class="label">Health</span>
            <span class="value health" style={{ color: getHealthColor() }}>
              {props.disk.health_percentage !== null && props.disk.health_percentage !== undefined ? `${props.disk.health_percentage}%` : "N/A"}
            </span>
          </div>
          <div class="info-row stacked">
            <span class="label">Serial</span>
            <span class="value serial">{props.disk.serial_number || "N/A"}</span>
          </div>
        </div>

        <div class="partition-list">
          <div class="partition-header">Partition:</div>
          {props.disk.partitions && props.disk.partitions.length > 0 ? (
            props.disk.partitions.map((partition) => (
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
                  (canEject() || props.isAdmin) && (
                    <button
                      class={`partition-btn ${canEject() ? "eject" : "mounted"}`}
                      onClick={(e) => {
                        e.stopPropagation();
                        props.onUnmount && props.onUnmount(props.disk.id, partition.drive_letter);
                      }}
                      data-tooltip={canEject() ? "Safely Remove (Eject)" : "Unmount Drive"}
                    >
                      {canEject() ? "Eject" : "Mounted"}
                    </button>
                  )
                ) : (
                  props.isAdmin && (
                    <button
                      class="partition-btn unmounted"
                      disabled={props.isAnyToggling || !props.disk.is_online}
                      onClick={(e) => {
                        e.stopPropagation();
                        props.onMount && props.onMount(props.disk.id, partition.partition_number);
                      }}
                      data-tooltip={props.disk.is_online ? "Click to mount" : "Disk is offline"}
                    >
                      Mount
                    </button>
                  )
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
