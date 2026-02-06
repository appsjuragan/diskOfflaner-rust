import { createSignal } from "solid-js";
import { Monitor, Shield, Database, Moon, Sun } from "lucide-solid";

function Settings(props) {
    // Helpers for toggle and range
    const Toggle = (localProps) => (
        <div
            class={`toggle-switch ${localProps.checked ? "checked" : ""}`}
            onClick={localProps.onChange}
        >
            <div class="toggle-knob" />
        </div>
    );

    return (
        <div class="settings-container">
            {/* Appearance Section */}
            <section class="settings-section">
                <div class="settings-title">
                    <Monitor size={20} />
                    <span>Appearance</span>
                </div>

                <div class="settings-row">
                    <div class="settings-info">
                        <span class="setting-label">Theme Mode</span>
                        <span class="setting-desc">Toggle between dark and light appearance</span>
                    </div>
                    <button class="icon-btn" onClick={props.toggleTheme} style={{ width: "auto", padding: "8px 16px", gap: "8px" }}>
                        {props.theme === "dark" ? <Moon size={16} /> : <Sun size={16} />}
                        <span>{props.theme === "dark" ? "Dark Mode" : "Light Mode"}</span>
                    </button>
                </div>

                <div class="settings-row">
                    <div class="settings-info">
                        <span class="setting-label">UI Zoom Level</span>
                        <span class="setting-desc">Adjust the interface scaling ({Math.round(props.zoomLevel * 100)}%)</span>
                    </div>
                    <input
                        type="range"
                        min="0.7"
                        max="1.5"
                        step="0.1"
                        value={props.zoomLevel}
                        class="range-slider"
                        onInput={(e) => props.setZoomLevel(parseFloat(e.target.value))}
                    />
                </div>
            </section>

            {/* Behavior Section */}
            <section class="settings-section">
                <div class="settings-title">
                    <Shield size={20} />
                    <span>Behavior</span>
                </div>

                <div class="settings-row">
                    <div class="settings-info">
                        <span class="setting-label">Safe Mode (Confirm Actions)</span>
                        <span class="setting-desc">Require confirmation before unmounting or offlining disks</span>
                    </div>
                    <Toggle checked={props.safeMode} onChange={() => props.setSafeMode(!props.safeMode)} />
                </div>

                <div class="settings-row">
                    <div class="settings-info">
                        <span class="setting-label">Auto-Refresh Disks</span>
                        <span class="setting-desc">Automatically refresh disk list every 15 seconds</span>
                    </div>
                    <Toggle checked={props.autoRefresh} onChange={() => props.setAutoRefresh(!props.autoRefresh)} />
                </div>
            </section>

            {/* Data Management */}
            <section class="settings-section">
                <div class="settings-title">
                    <Database size={20} />
                    <span>Data Management</span>
                </div>

                <div class="settings-row">
                    <div class="settings-info">
                        <span class="setting-label">Clear Application Logs</span>
                        <span class="setting-desc">Permanently remove all activity history</span>
                    </div>
                    <button class="btn-danger" onClick={props.onClearLogs}>
                        Clear History
                    </button>
                </div>
            </section>

        </div>
    );
}

export default Settings;
