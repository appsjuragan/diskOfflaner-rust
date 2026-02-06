import { HardDrive, Settings, FileText, Info } from "lucide-solid";

function Sidebar(props) {
  const navItems = [
    { id: "drives", label: "Drives", icon: HardDrive },
    { id: "settings", label: "Settings", icon: Settings },
    { id: "logs", label: "Logs", icon: FileText },
    { id: "info", label: "System Info", icon: Info },
  ];

  return (
    <aside class="sidebar">
      <div class="sidebar-logo">
        <img src="/g1.png" class="logo-img" alt="DiskOfflaner Logo" />
        <span class="logo-text">DiskOfflaner</span>
      </div>

      <nav class="sidebar-nav">
        {navItems.map((item) => (
          <button
            class={`nav-item ${props.activePage === item.id ? "active" : ""}`}
            onClick={() => props.setActivePage(item.id)}
          >
            <item.icon class="nav-icon" size={20} />
            <span class="nav-label">{item.label}</span>
          </button>
        ))}
      </nav>
    </aside>
  );
}

export default Sidebar;
