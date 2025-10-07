<script lang="ts">
    import { pipStore } from "$lib/stores/pip.svelte";
    import CameraVideo from "$lib/components/camera-video.svelte";

    let dragState = $state({
        isDragging: false,
        startX: 0,
        startY: 0,
        offsetX: 0,
        offsetY: 0
    });

    let pipElement: HTMLDivElement;

    function handleMouseDown(e: MouseEvent) {
        if (!pipElement) return;

        dragState.isDragging = true;
        const rect = pipElement.getBoundingClientRect();
        dragState.startX = e.clientX - rect.left;
        dragState.startY = e.clientY - rect.top;

        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
        e.preventDefault();
    }

    function handleMouseMove(e: MouseEvent) {
        if (!dragState.isDragging) return;

        dragState.offsetX = e.clientX - dragState.startX;
        dragState.offsetY = e.clientY - dragState.startY;

        // Keep within viewport bounds
        const maxX = window.innerWidth - 320; // PiP width
        const maxY = window.innerHeight - 192; // PiP height

        dragState.offsetX = Math.max(0, Math.min(maxX, dragState.offsetX));
        dragState.offsetY = Math.max(0, Math.min(maxY, dragState.offsetY));
    }

    function handleMouseUp() {
        dragState.isDragging = false;
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
    }

    function getPositionStyle() {
        if (dragState.offsetX === 0 && dragState.offsetY === 0) {
            return "bottom: 1rem; right: 1rem;";
        }
        return `left: ${dragState.offsetX}px; top: ${dragState.offsetY}px;`;
    }
</script>

<!-- Global Picture-in-Picture Window -->
{#if pipStore.isActive && pipStore.camera}
    <div
        bind:this={pipElement}
        class="fixed z-50 w-80 h-48 bg-background border border-border rounded-lg overflow-hidden shadow-lg cursor-move select-none"
        style={getPositionStyle()}
        onmousedown={(e) => {
            // Check if clicking on button or any interactive element
            const target = e.target;
            if (target.tagName === 'BUTTON' ||
                target.closest('button') ||
                target.closest('[role="button"]') ||
                target.classList.contains('pointer-events-auto')) {
                return;
            }
            handleMouseDown(e);
        }}
    >
        <CameraVideo
            camera={pipStore.camera}
            onClose={pipStore.close}
            variant="pip"
        />



        <!-- Drag Hint -->
        {#if dragState.isDragging}
            <div class="absolute inset-0 bg-primary/10 border-2 border-primary/30 rounded-lg pointer-events-none"></div>
        {/if}
    </div>
{/if}
