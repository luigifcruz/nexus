<script lang="ts">
    import { dataStore } from "$lib/stores/data.svelte";
    import { apiSync } from "$lib/services/sync";
    import CircleIcon from "@tabler/icons-svelte/icons/circle";
    import CircleDottedIcon from "@tabler/icons-svelte/icons/circle-dotted";
    import CircleXIcon from "@tabler/icons-svelte/icons/circle-x";

    let connectionStatus = $derived(dataStore.connectionStatus);

    let status = $derived.by(() => {
        if (!connectionStatus.connected && connectionStatus.reconnecting) {
            return "connecting";
        } else if (!connectionStatus.connected && connectionStatus.error) {
            return "error";
        } else if (!connectionStatus.connected) {
            return "disconnected";
        }
        return "connected";
    });

    const statusConfig = {
        connected: {
            label: "Connected to Nexus",
        },
        connecting: {
            label: "Connecting to Nexus...",
        },
        disconnected: {
            label: "Disconnected",
        },
        error: {
            label: "Connection error",
        },
    };

    function handleClick() {
        if (status === "disconnected" || status === "error") {
            apiSync.reconnect();
        }
    }
</script>

<button
    class="inline-flex h-8 items-center gap-2 rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
    onclick={handleClick}
    type="button"
>
    {#if status === "connected"}
        <CircleIcon class="h-4 w-4 fill-green-500 text-green-500 animate-pulse" />
    {:else if status === "connecting"}
        <CircleDottedIcon class="h-4 w-4 animate-pulse text-yellow-500" />
    {:else if status === "error"}
        <CircleXIcon class="h-4 w-4 fill-red-500 text-red-500" />
    {:else}
        <CircleIcon class="h-4 w-4 text-gray-400" />
    {/if}
    <span class="text-xs text-muted-foreground">
        {statusConfig[status].label}
    </span>
</button>
