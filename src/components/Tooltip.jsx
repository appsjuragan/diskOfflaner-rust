import { createSignal, onMount, onCleanup } from "solid-js";
import { Portal } from "solid-js/web";

const Tooltip = () => {
    const [isVisible, setIsVisible] = createSignal(false);
    const [text, setText] = createSignal("");
    const [coords, setCoords] = createSignal({ x: 0, y: 0 });

    let hoverTimer = null;
    let activeTarget = null;

    const showTooltip = () => {
        if (!activeTarget) return;
        const tooltipText = activeTarget.getAttribute("data-tooltip");
        if (!tooltipText) return;

        const rect = activeTarget.getBoundingClientRect();

        // Position: centered below the element
        let x = rect.left + rect.width / 2;
        let y = rect.bottom + 8;

        setText(tooltipText);
        setCoords({ x, y });
        setIsVisible(true);
    };

    const handleMouseOver = (e) => {
        const target = e.target.closest("[data-tooltip]");

        // If moving within the same target, do nothing
        if (target === activeTarget) return;

        // If we had an active target, clear it (we effectively left it)
        if (activeTarget) {
            clearTimeout(hoverTimer);
            setIsVisible(false);
            activeTarget = null;
        }

        // If we entered a new valid target
        if (target) {
            activeTarget = target;
            // Start the 1.5s delay
            hoverTimer = setTimeout(showTooltip, 1500);
        }
    };

    const handleMouseOut = (e) => {
        // If we aren't tracking anything, ignore
        if (!activeTarget) return;

        // Check if we moved to a child of the active target
        // (mouseover handles entry, mouseout handles exit)
        // If relatedTarget (where we went) is inside activeTarget, ignore
        if (activeTarget.contains(e.relatedTarget)) return;

        // We truly left the active target
        clearTimeout(hoverTimer);
        setIsVisible(false);
        activeTarget = null;
    };

    const handleMouseDown = () => {
        // Hide immediately on click
        clearTimeout(hoverTimer);
        setIsVisible(false);
    };

    onMount(() => {
        document.addEventListener("mouseover", handleMouseOver);
        document.addEventListener("mouseout", handleMouseOut);
        document.addEventListener("mousedown", handleMouseDown);
        // Also handle scroll to hide tooltip
        document.addEventListener("scroll", handleMouseDown, true);
    });

    onCleanup(() => {
        document.removeEventListener("mouseover", handleMouseOver);
        document.removeEventListener("mouseout", handleMouseOut);
        document.removeEventListener("mousedown", handleMouseDown);
        document.removeEventListener("scroll", handleMouseDown, true);
    });

    return (
        <Portal>
            <div
                class={`global-tooltip ${isVisible() ? "visible" : ""}`}
                style={{
                    "top": `${coords().y}px`,
                    "left": `${coords().x}px`,
                }}
            >
                {text()}
            </div>
        </Portal>
    );
};

export default Tooltip;
