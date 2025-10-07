<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Chart from "$lib/components/ui/chart/index.js";
    import { Area, AreaChart } from "layerchart";
    import { curveNatural } from "d3-shape";
    import TrendingUpIcon from "@tabler/icons-svelte/icons/trending-up";

    interface MetricDataPoint {
        timestamp: Date;
        packetDrops: number;
        nvmeUsage: number;
        gpuUsage: number;
        vramUsage: number;
        watts: number;
        temperature: number;
    }

    interface Props {
        metricsHistory: MetricDataPoint[];
    }

    let { metricsHistory }: Props = $props();
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-4">
            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">Packet Drops (Packets)</h3>
                <Chart.Container config={{
                    packetDrops: { label: "Packet Drops", color: "#dc2626" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "packetDrops",
                            label: "Packet Drops",
                            color: "#dc2626",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${Math.round(d)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillPacketDrops" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#dc2626" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#dc2626" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillPacketDrops)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
        </Chart.Container>
    </div>

            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">NVMe (GB)</h3>
                <Chart.Container config={{
                    nvmeUsage: { label: "NVMe Usage", color: "#7c3aed" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "nvmeUsage",
                            label: "NVMe Usage",
                            color: "#7c3aed",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${(d / (1024 ** 3)).toFixed(1)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillNvmeUsage" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#7c3aed" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#7c3aed" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillNvmeUsage)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
        </Chart.Container>
    </div>

            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">GPU (%)</h3>
                <Chart.Container config={{
                    gpuUsage: { label: "GPU Usage", color: "#059669" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "gpuUsage",
                            label: "GPU Usage",
                            color: "#059669",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${Math.round(d)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillGpuUsage" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#059669" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#059669" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillGpuUsage)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
        </Chart.Container>
    </div>

            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">VRAM (GB)</h3>
                <Chart.Container config={{
                    vramUsage: { label: "VRAM Usage", color: "#2563eb" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "vramUsage",
                            label: "VRAM Usage",
                            color: "#2563eb",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${(d / (1024 ** 3)).toFixed(1)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillVramUsage" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#2563eb" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#2563eb" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillVramUsage)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
        </Chart.Container>
    </div>

            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">Power (Watts)</h3>
                <Chart.Container config={{
                    watts: { label: "Power", color: "#d97706" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "watts",
                            label: "Power",
                            color: "#d97706",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${Math.round(d)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillWatts" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#d97706" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#d97706" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillWatts)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
        </Chart.Container>
    </div>

            <div class="space-y-1">
                <h3 class="text-xs font-medium text-muted-foreground mb-2">Temperature (ºC)</h3>
                <Chart.Container config={{
                    temperature: { label: "Temperature", color: "#e11d48" }
                }} class="h-48 w-full">
                <AreaChart
                    data={metricsHistory}
                    x="timestamp"
                    series={[
                        {
                            key: "temperature",
                            label: "Temperature",
                            color: "#e11d48",
                        }
                    ]}
                    props={{
                        area: {
                            "fill-opacity": 0.4,
                            line: { class: "stroke-2" },
                            curve: curveNatural,
                        },
                        xAxis: {
                            ticks: 3,
                            format: (v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            },
                        },
                        yAxis: {
                            format: (d) => `${Math.round(d)}`,
                            ticks: 2
                        },
                    }}
                >
                    {#snippet marks({ series, getAreaProps })}
                        <defs>
                            <linearGradient id="fillTemperature" x1="0" y1="0" x2="0" y2="1">
                                <stop offset="5%" stop-color="#e11d48" stop-opacity={0.3} />
                                <stop offset="95%" stop-color="#e11d48" stop-opacity={0.05} />
                            </linearGradient>
                        </defs>
                        {#each series as s, i (s.key)}
                            <Area {...getAreaProps(s, i)} fill="url(#fillTemperature)" />
                        {/each}
                    {/snippet}
                    {#snippet tooltip()}
                        <Chart.Tooltip
                            labelFormatter={(v) => {
                                return new Date(v).toLocaleTimeString("en-US", {
                                    hour: "2-digit",
                                    minute: "2-digit",
                                });
                            }}
                            indicator="line"
                        />
                    {/snippet}
                </AreaChart>
                </Chart.Container>
            </div>
</div>
