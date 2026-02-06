import { Show } from "solid-js";

function Modal(props) {
    return (
        <Show when={props.show}>
            <div class="modal-overlay" onClick={props.onCancel}>
                <div class="modal-container" onClick={(e) => e.stopPropagation()}>
                    <div class="modal-header">
                        {props.title || "Confirm Action"}
                    </div>
                    <div class="modal-body">
                        {props.message}
                    </div>
                    <div class="modal-footer">
                        <button class="modal-btn cancel" onClick={props.onCancel}>
                            {props.cancelLabel || "Cancel"}
                        </button>
                        <button
                            class={`modal-btn confirm ${props.isDanger ? 'danger' : ''}`}
                            onClick={props.onConfirm}
                        >
                            {props.confirmLabel || "Confirm"}
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    );
}

export default Modal;
