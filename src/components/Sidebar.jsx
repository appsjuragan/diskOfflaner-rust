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
                <div class="logo-icon"></div>
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

            <style jsx>{`
        .sidebar {
          width: 220px;
          background-color: var(--bg-sidebar);
          padding: 24px 16px;
          border-right: 1px solid var(--border-sidebar);
          display: flex;
          flex-direction: column;
        }

        .sidebar-logo {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 12px;
          margin-bottom: 32px;
        }

        .logo-icon {
          width: 32px;
          height: 32px;
          background: linear-gradient(135deg, var(--accent-primary), var(--accent-secondary));
          border-radius: 8px;
        }

        .logo-text {
          font-size: 18px;
          font-weight: 600;
        }

        .sidebar-nav {
          display: flex;
          flex-direction: column;
          gap: 8px;
        }

        .nav-item {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 12px 16px;
          border-radius: 8px;
          border: none;
          background: transparent;
          color: var(--text-secondary);
          transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
          text-align: left;
          width: 100%;
        }

        .nav-item:hover {
          background-color: var(--bg-sidebar-item-hover);
          color: var(--text-primary);
        }

        .nav-item:hover .nav-icon {
          transform: translateX(2px);
          opacity: 1;
        }

        .nav-item.active {
          background-color: var(--bg-sidebar-item);
          color: var(--text-primary);
          font-weight: 600;
        }

        .nav-icon {
          stroke-width: 2px;
          opacity: 0.7;
          transition: transform 0.2s, opacity 0.2s;
        }
      `}</style>
        </aside>
    );
}

export default Sidebar;
