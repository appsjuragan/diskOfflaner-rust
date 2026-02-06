import { createSignal, onMount, For, Show, createEffect } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

function DriveLetterModal(props) {
    const [letters, setLetters] = createSignal([]);
    const [selectedLetter, setSelectedLetter] = createSignal(null);
    const [loading, setLoading] = createSignal(true);

    const fetchLetters = async () => {
        setLoading(true);
        try {
            const available = await invoke("get_available_drive_letters_command");
            setLetters(available);
        } catch (error) {
            console.error("Failed to get available letters:", error);
        } finally {
            setLoading(false);
        }
    };

    createEffect(() => {
        if (props.show) {
            fetchLetters();
            setSelectedLetter(null);
        }
    });

    const handleMount = () => {
        props.onConfirm(selectedLetter());
    };

    const filteredLetters = () => letters().filter(l => !props.assignedLetters.has(l));

    return (
        <Show when={props.show}>
            <div class="modal-overlay" onClick={props.onCancel}>
                <div class="modal-container drive-letter-modal" onClick={(e) => e.stopPropagation()}>
                    <div class="modal-header">
                        Select Drive Letter
                    </div>
                    <div class="modal-body">
                        <p class="modal-description">Choose a drive letter for the partition, or use Auto to assign automatically.</p>

                        {loading() ? (
                            <div class="letter-loading">
                                <div class="spinner"></div>
                                <span>Loading available letters...</span>
                            </div>
                        ) : (
                            <div class="letter-grid">
                                <button
                                    class={`letter-btn auto ${selectedLetter() === null ? 'selected' : ''}`}
                                    onClick={() => setSelectedLetter(null)}
                                >
                                    Auto
                                </button>
                                <For each={filteredLetters()}>
                                    {(letter) => (
                                        <button
                                            class={`letter-btn ${selectedLetter() === letter ? 'selected' : ''}`}
                                            onClick={() => setSelectedLetter(letter)}
                                        >
                                            {letter}:
                                        </button>
                                    )}
                                </For>
                            </div>
                        )}
                    </div>
                    <div class="modal-footer">
                        <button class="modal-btn cancel" onClick={props.onCancel}>
                            Cancel
                        </button>
                        <button
                            class="modal-btn confirm"
                            onClick={handleMount}
                            disabled={loading()}
                        >
                            Mount {selectedLetter() ? `as ${selectedLetter()}:` : '(Auto)'}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    );
}

export default DriveLetterModal;
