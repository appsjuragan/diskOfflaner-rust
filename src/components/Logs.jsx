import { createSignal, onMount, For } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { RotateCw, Trash2, Clock } from "lucide-solid";

function Logs(props) {
    const [logs, setLogs] = createSignal([]);
    const [loading, setLoading] = createSignal(true);

    const fetchLogs = async () => {
        setLoading(true);
        try {
            const result = await invoke("get_logs_command");
            setLogs(result.reverse()); // Show newest first
        } catch (error) {
            console.error("Failed to fetch logs:", error);
        } finally {
            setLoading(false);
        }
    };

    const clearLogs = async () => {
        props.showConfirm({
            title: "Clear History",
            message: "Are you sure you want to clear all activity logs? This action cannot be undone.",
            confirmLabel: "Clear All",
            isDanger: true,
            onConfirm: async () => {
                try {
                    await invoke("clear_logs_command");
                    setLogs([]);
                } catch (error) {
                    console.error("Failed to clear logs:", error);
                }
            }
        });
    };

    onMount(fetchLogs);

    return (
        <div class="logs-container">
            <div class="logs-header">
                <div class="logs-header-title">
                    <Clock size={20} class="header-icon" />
                    <span>Activity History</span>
                </div>
                <div class="logs-actions">
                    <button
                        class="log-action-btn refresh"
                        onClick={fetchLogs}
                        disabled={loading()}
                        data-tooltip="Refresh Logs"
                    >
                        <RotateCw size={16} class={loading() ? "spin" : ""} />
                    </button>
                    <button
                        class="log-action-btn clear"
                        onClick={clearLogs}
                        disabled={loading() || logs().length === 0}
                        data-tooltip="Clear History"
                    >
                        <Trash2 size={16} />
                    </button>
                </div>
            </div>

            <div class="logs-content">
                {loading() && logs().length === 0 ? (
                    <div class="logs-loading">
                        <div class="spinner"></div>
                        <span>Loading History...</span>
                    </div>
                ) : logs().length === 0 ? (
                    <div class="logs-empty">
                        <Clock size={48} />
                        <p>No activity history found.</p>
                    </div>
                ) : (
                    <div class="logs-list">
                        <For each={logs()}>
                            {(log) => {
                                const parts = log.split("] ");
                                const timestamp = parts[0].replace("[", "");
                                const message = parts[1];
                                return (
                                    <div class="log-item">
                                        <span class="log-timestamp">{timestamp}</span>
                                        <span class="log-message">{message}</span>
                                    </div>
                                );
                            }}
                        </For>
                    </div>
                )}
            </div>
        </div>
    );
}

export default Logs;
