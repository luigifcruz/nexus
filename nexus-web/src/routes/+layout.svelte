<script lang="ts">
    import "../app.css";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import GlobalPip from "$lib/components/global-pip.svelte";
    import { apiSync } from "$lib/services/sync";
    import { dataStore } from "$lib/stores/data.svelte";

    import { onMount } from "svelte";
    import LoaderIcon from "@tabler/icons-svelte/icons/loader-2";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";
    import { Button } from "$lib/components/ui/button/index.js";

    let sidebarOpen = $state(true);
    let mounted = $state(false);
    let { children } = $props();

    let connectionStatus = $derived(dataStore.connectionStatus);

    onMount(() => {
        apiSync.initialize();
        mounted = true;
    });
</script>

<Sidebar.Provider
    bind:open={sidebarOpen}
    style="--sidebar-width: calc(var(--spacing) * 60); --header-height: calc(var(--spacing) * 12);"
>
    <AppSidebar />
    <Sidebar.Inset>
        <div class="flex flex-1 flex-col">
            {#if !mounted}
                <div class="flex flex-col items-center justify-center h-screen gap-4">
                    <LoaderIcon class="h-12 w-12 text-primary animate-spin" />
                    <div class="text-center">
                        <h2 class="text-xl font-semibold">Loading Client</h2>
                        <p class="text-sm text-muted-foreground">Initializing Application</p>
                    </div>
                </div>
            {:else if !connectionStatus.connected && connectionStatus.reconnecting}
                <div class="flex flex-col items-center justify-center h-screen gap-4">
                    <LoaderIcon class="h-12 w-12 text-primary animate-spin" />
                    <div class="text-center">
                        <h2 class="text-xl font-semibold">Connecting to Nexus</h2>
                        <p class="text-sm text-muted-foreground">Establishing Connection</p>
                    </div>
                </div>
            {:else if !connectionStatus.connected && connectionStatus.error}
                <div class="flex flex-col items-center justify-center h-screen gap-6">
                    <div class="p-4 rounded-full bg-destructive/10">
                        <ServerIcon class="h-10 w-10 text-destructive" />
                    </div>
                    <div class="text-center max-w-md space-y-3">
                        <h2 class="text-xl font-semibold">Connection Failed</h2>
                        <code class="block text-xs text-muted-foreground px-3 py-2 bg-muted rounded-md font-mono">{connectionStatus.error}</code>
                    </div>
                    <Button variant="outline" onclick={() => window.location.reload()}>
                        <RefreshIcon class="h-4 w-4" />
                        Retry
                    </Button>
                </div>
            {:else}
                <div class="@container/main flex flex-1 flex-col gap-2">
                    {@render children()}
                </div>
            {/if}
        </div>
    </Sidebar.Inset>
    <GlobalPip />
</Sidebar.Provider>
