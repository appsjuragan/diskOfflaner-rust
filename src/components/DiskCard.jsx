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

      <style jsx>{`
        .disk-card {
          background: var(--bg-card);
          border: 1px solid var(--border-card);
          border-radius: 16px;
          padding: 20px;
          backdrop-filter: blur(10px);
          -webkit-backdrop-filter: blur(10px);
          transition: transform 0.25s var(--ease-smooth), box-shadow 0.25s var(--ease-smooth), background 0.25s;
          position: relative;
          overflow: hidden;
        }

        .disk-card:hover {
          background: var(--bg-card-hover);
          transform: translateY(-2px);
          box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
        }

        .disk-card[data-status="offline"] {
          border-color: rgba(239, 68, 68, 0.3);
          background: rgba(60, 40, 45, 0.6);
        }

        .disk-card[data-status="offline"]:hover {
          background: rgba(70, 45, 50, 0.7);
        }

        /* Header */
        .card-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 20px;
          padding-bottom: 12px;
          border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }

        .card-title {
          display: flex;
          align-items: center;
          gap: 12px;
        }

        .disk-name {
          font-size: 16px;
          font-weight: 600;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
          max-width: 180px;
        }

        .icon-wrapper {
          display: flex;
          align-items: center;
          justify-content: center;
        }

        /* Status Badge */
        .status-badge {
          padding: 4px 12px;
          border-radius: 12px;
          font-size: 11px;
          font-weight: 700;
          letter-spacing: 0.05em;
          text-transform: uppercase;
        }

        .status-badge.online {
          background: rgba(74, 222, 128, 0.15);
          color: var(--status-online);
          border: 1px solid rgba(74, 222, 128, 0.3);
          box-shadow: 0 0 0 0 rgba(74, 222, 128, 0.4);
          animation: pulse-glow 3s infinite;
        }

        .status-badge.offline {
          background: rgba(239, 68, 68, 0.15);
          color: var(--status-offline);
          border: 1px solid rgba(239, 68, 68, 0.3);
        }

        @keyframes pulse-glow {
          0%, 100% { box-shadow: 0 0 0 0 rgba(74, 222, 128, 0); }
          50% { box-shadow: 0 0 8px 0 rgba(74, 222, 128, 0.2); }
        }

        /* Body */
        .card-body {
          display: flex;
          flex-direction: column;
          gap: 10px;
        }

        .info-row {
          display: flex;
          justify-content: space-between;
          align-items: center;
        }

        .label {
          font-size: 13px;
          color: var(--text-secondary);
          font-weight: 500;
        }

        .value {
          font-size: 14px;
          color: var(--text-value);
          font-weight: 600;
        }

        .value.serial {
          font-family: var(--font-mono);
          letter-spacing: 0.5px;
          font-size: 13px;
        }
      `}</style>
    </div>
  );
}

export default DiskCard;
