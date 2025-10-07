<script lang="ts">
    import { dataStore } from "$lib/stores/data.svelte";
    import { apiSync } from "$lib/services/sync";
    import CircleIcon from "@tabler/icons-svelte/icons/circle";
    import CircleDottedIcon from "@tabler/icons-svelte/icons/circle-dotted";
    import CircleXIcon from "@tabler/icons-svelte/icons/circle-x";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";

    let connectionStatus = $derived(dataStore.connectionStatus);

    // Determine status from connectionStatus object
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
            icon: CircleIcon,
            color: "text-green-500",
            label: "Connected to Nexus",
            fill: true,
        },
        connecting: {
            icon: CircleDottedIcon,
            color: "text-yellow-500",
            label: "Connecting to Nexus...",
            fill: false,
        },
        disconnected: {
            icon: CircleIcon,
            color: "text-gray-400",
            label: "Disconnected",
            fill: false,
        },
        error: {
            icon: CircleXIcon,
            color: "text-red-500",
            label: "Connection error",
            fill: true,
        },
    };

    function handleClick() {
        if (status === "disconnected" || status === "error") {
            apiSync.reconnect();
        }
    }
</script>

<Tooltip.Provider>
    <Tooltip.Root>
        <Tooltip.Trigger asChild>
            {#snippet child({ props })}
                <button
                    {...props}
                    class="inline-flex h-8 items-center gap-2 rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50"
                    onclick={handleClick}
                    type="button"
                >
                    {#if status === "connected"}
                        <CircleIcon class="h-4 w-4 fill-green-500 text-green-500 animate-pulse" />
                    {:else if status === "connecting"}
                        <CircleDottedIcon
                            class="h-4 w-4 animate-pulse text-yellow-500"
                        />
                    {:else if status === "error"}
                        <CircleXIcon class="h-4 w-4 fill-red-500 text-red-500" />
                    {:else}
                        <CircleIcon class="h-4 w-4 text-gray-400" />
                    {/if}
                    <span class="text-xs text-muted-foreground">
                        {statusConfig[status].label}
                    </span>
                </button>
            {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>
            <p>
                {statusConfig[status].label}
                {#if connectionStatus.error}
                    <br />
                    <span class="text-xs text-muted-foreground"
                        >{connectionStatus.error}</span
                    >
                {/if}
            </p>
            {#if status === "disconnected" || status === "error"}
                <p class="mt-1 text-xs text-muted-foreground">
                    Click to reconnect
                </p>
            {/if}
        </Tooltip.Content>
    </Tooltip.Root>
</Tooltip.Provider>
