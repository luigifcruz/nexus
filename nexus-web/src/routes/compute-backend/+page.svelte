<script lang="ts">
    import { onMount } from "svelte";
    import { browser } from "$app/environment";
    import {
        SvelteFlow,
        Controls,
        Background,
        type Node,
        type Edge,
    } from "@xyflow/svelte";
    import "@xyflow/svelte/dist/style.css";

    let nodes = $state<Node[]>([]);
    let edges = $state<Edge[]>([]);
    let isDark = $state(false);

    // Initialize theme from document class
    if (browser) {
        isDark = document.documentElement.classList.contains("dark");
    }

    // Initialize topology data
    function initializeTopology() {
        // Compute Backend topology - Switch -> Replicants -> Storage
        const computeSwitchNode = {
            id: "main-switch",
            type: "default",
            position: { x: 550, y: 50 },
            data: {
                label: "Main Switch",
                type: "switch",
                status: "online",
            },
            style: "background-color: #fff3e0; border: 2px solid #ff9800; border-radius: 8px; padding: 15px; font-weight: bold; color: #f57c00; width: 140px; text-align: center;",
        };

        const replicantNodes = Array.from({ length: 8 }, (_, index) => ({
            id: `replicant-${index + 1}`,
            type: "default",
            position: {
                x: (index % 4) * 200 + 250,
                y: Math.floor(index / 4) * 150 + 250,
            },
            data: {
                label: `Replicant ${index + 1}`,
                type: "replicant",
                status: Math.random() > 0.1 ? "online" : "offline",
                cpu: Math.floor(Math.random() * 100),
                memory: Math.floor(Math.random() * 100),
            },
            style: "background-color: #e8f5e8; border: 2px solid #4caf50; border-radius: 8px; padding: 10px; font-weight: bold; color: #388e3c; width: 140px; text-align: center;",
        }));

        const storageNode = {
            id: "hdd-storage",
            type: "default",
            position: { x: 550, y: 600 },
            data: {
                label: "HDD Storage\n1.2 PB Available",
                type: "storage",
                status: "online",
                capacity: "1.2 PB",
                used: "847 TB",
            },
            style: "background-color: #f3e5f5; border: 2px solid #9c27b0; border-radius: 8px; padding: 15px; font-weight: bold; color: #7b1fa2; width: 160px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        };

        nodes = [computeSwitchNode, ...replicantNodes, storageNode];

        const switchToReplicantEdges = replicantNodes.map((_, index) => ({
            id: `switch-to-replicant-${index + 1}`,
            source: "main-switch",
            target: `replicant-${index + 1}`,
            style: "stroke: #1a73e8; stroke-width: 3px;",
            animated: false,
        }));

        const replicantToStorageEdges = replicantNodes.map((_, index) => ({
            id: `replicant-${index + 1}-to-storage`,
            source: `replicant-${index + 1}`,
            target: "hdd-storage",
            style: "stroke: #34a853; stroke-width: 2px;",
            animated: false,
        }));

        edges = [...switchToReplicantEdges, ...replicantToStorageEdges];
    }

    onMount(() => {
        initializeTopology();

        // Simulate real-time status updates
        const interval = setInterval(() => {
            // Randomly update replicant statuses and metrics
            nodes = nodes.map((node) => {
                if (node.data.type === "replicant") {
                    const shouldChange = Math.random() > 0.9;
                    if (shouldChange) {
                        return {
                            ...node,
                            data: {
                                ...node.data,
                                status:
                                    node.data.status === "online"
                                        ? "offline"
                                        : "online",
                                cpu: Math.floor(Math.random() * 100),
                                memory: Math.floor(Math.random() * 100),
                            },
                        };
                    }
                }
                return node;
            });
        }, 5000);

        return () => clearInterval(interval);
    });
</script>

<svelte:head>
    <title>Nexus - Compute Backend</title>
</svelte:head>

<div class="flex flex-col gap-6 py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">
                    Compute Backend Topology
                </h1>
                <p class="text-muted-foreground">
                    Replicant compute nodes and data storage infrastructure
                </p>
            </div>
        </div>
    </div>

    <!-- Topology View -->
    <div class="px-4 lg:px-6">
        <div class="h-[calc(100vh-12rem)]">
            <div class="h-full rounded-lg border bg-card">
                <SvelteFlow
                    {nodes}
                    {edges}
                    fitView
                    colorMode={isDark ? "dark" : "light"}
                >
                    <Background />
                    <Controls />
                </SvelteFlow>
            </div>
        </div>

        <!-- Legend -->
        <div class="px-4 lg:px-6 pb-4 pt-4">
            <div class="flex items-center gap-6 text-sm">
                <div class="flex items-center gap-2">
                    <div
                        class="w-4 h-1 bg-[#1a73e8] rounded"
                        style="background-color: #1a73e8;"
                    ></div>
                    <span class="text-muted-foreground">200 GbE Network</span>
                </div>
                <div class="flex items-center gap-2">
                    <div
                        class="w-4 h-1 bg-[#34a853] rounded"
                        style="background-color: #34a853;"
                    ></div>
                    <span class="text-muted-foreground">10 GbE Network</span>
                </div>
            </div>
        </div>
    </div>
</div>
