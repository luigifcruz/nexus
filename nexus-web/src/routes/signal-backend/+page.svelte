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
        // Signal Backend topology - Antennas -> Amps -> RFSoCs -> Switch
        const antennaNames = ["1A", "1B", "2A", "2B", "3A", "3B", "4A", "4B"];

        const antennaNodes = antennaNames.map((name, index) => ({
            id: `antenna-${name}`,
            type: "default",
            position: { x: index * 150, y: 50 },
            data: {
                label: `Antenna ${name}\nAz: ${Math.floor(Math.random() * 360)}°\nEl: ${Math.floor(Math.random() * 90)}°`,
                type: "antenna",
                status: Math.random() > 0.2 ? "online" : "offline",
                azimuth: Math.floor(Math.random() * 360),
                elevation: Math.floor(Math.random() * 90),
            },
            style: "background-color: #e3f2fd; border: 2px solid #2196f3; border-radius: 8px; padding: 10px; font-weight: bold; color: #1976d2; width: 140px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        }));

        const ampNodes = antennaNames.map((name, index) => ({
            id: `amp-${name}`,
            type: "default",
            position: { x: index * 150, y: 250 },
            data: {
                label: `Amp ${name}`,
                type: "amp",
                status: Math.random() > 0.1 ? "online" : "offline",
            },
            style: "background-color: #f3e5f5; border: 2px solid #9c27b0; border-radius: 8px; padding: 10px; font-weight: bold; color: #7b1fa2; width: 120px; text-align: center;",
        }));

        const rfsocNodes = Array.from({ length: 2 }, (_, index) => ({
            id: `rfsoc-${index + 1}`,
            type: "default",
            position: { x: index * 600 + 300, y: 450 },
            data: {
                label: `RFSoC ${index + 1}\nIP: 10.10.2.${index + 1}`,
                type: "rfsoc",
                status: Math.random() > 0.05 ? "online" : "offline",
            },
            style: "background-color: #e8f5e8; border: 2px solid #4caf50; border-radius: 8px; padding: 10px; font-weight: bold; color: #388e3c; width: 120px; text-align: center; white-space: pre-line; font-size: 12px; line-height: 1.3;",
        }));

        const signalSwitchNode = {
            id: "main-switch",
            type: "default",
            position: { x: 500, y: 650 },
            data: {
                label: "Main Switch",
                type: "switch",
                status: "online",
            },
            style: "background-color: #fff3e0; border: 2px solid #ff9800; border-radius: 8px; padding: 15px; font-weight: bold; color: #f57c00; width: 140px; text-align: center;",
        };

        nodes = [
            ...antennaNodes,
            ...ampNodes,
            ...rfsocNodes,
            signalSwitchNode,
        ];

        const antennaToAmpEdges = antennaNames.map((name) => ({
            id: `antenna-${name}-to-amp-${name}`,
            source: `antenna-${name}`,
            target: `amp-${name}`,
            style: "stroke: #8b4513; stroke-width: 2px;",
            animated: false,
        }));

        const ampToRfsocEdges = antennaNames.map((name, index) => ({
            id: `amp-${name}-to-rfsoc-${Math.floor(index / 4) + 1}`,
            source: `amp-${name}`,
            target: `rfsoc-${Math.floor(index / 4) + 1}`,
            style: "stroke: #9c27b0; stroke-width: 2px;",
            animated: false,
        }));

        const rfsocToSwitchEdges = Array.from({ length: 2 }, (_, index) => ({
            id: `rfsoc-${index + 1}-to-switch`,
            source: `rfsoc-${index + 1}`,
            target: "main-switch",
            style: "stroke: #4caf50; stroke-width: 3px;",
            animated: false,
        }));

        edges = [
            ...antennaToAmpEdges,
            ...ampToRfsocEdges,
            ...rfsocToSwitchEdges,
        ];
    }

    onMount(() => {
        initializeTopology();

        // Simulate real-time status updates
        const interval = setInterval(() => {
            // Randomly update node statuses
            nodes = nodes.map((node) => {
                if (node.data.type === "antenna" || node.data.type === "amp") {
                    const shouldChange = Math.random() > 0.95;
                    if (shouldChange) {
                        return {
                            ...node,
                            data: {
                                ...node.data,
                                status:
                                    node.data.status === "online"
                                        ? "offline"
                                        : "online",
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

<div class="flex flex-col gap-6 py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">
                    Signal Backend Topology
                </h1>
                <p class="text-muted-foreground">
                    Allen Telescope Array signal processing chain from antennas to network switch
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
                        class="w-4 h-1 bg-[#8b4513] rounded"
                        style="background-color: #8b4513;"
                    ></div>
                    <span class="text-muted-foreground">Copper (Coax)</span>
                </div>
                <div class="flex items-center gap-2">
                    <div
                        class="w-4 h-1 bg-[#9c27b0] rounded"
                        style="background-color: #9c27b0;"
                    ></div>
                    <span class="text-muted-foreground">Fiber Optic</span>
                </div>
                <div class="flex items-center gap-2">
                    <div
                        class="w-4 h-1 bg-[#4caf50] rounded"
                        style="background-color: #4caf50;"
                    ></div>
                    <span class="text-muted-foreground">100 GbE</span>
                </div>
            </div>
        </div>
    </div>
</div>
