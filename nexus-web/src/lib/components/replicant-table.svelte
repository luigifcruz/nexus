<script lang="ts">
    import type { ReplicantData } from "$lib/types";
    import { goto } from "$app/navigation";
    import { getStatusIconWithClass } from "$lib/utils/status";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";
    import ThermometerIcon from "@tabler/icons-svelte/icons/thermometer";

    interface Props {
        replicants: ReplicantData[];
    }

    let { replicants }: Props = $props();

    function navigateToReplicant(id: string) {
        goto(`/replicants/${id}`);
    }
</script>

<style>
    :global(.nvme-progress[data-slot="progress"]) {
        background-color: rgb(147 197 253 / 0.2);
    }

    :global(.nvme-progress[data-slot="progress"] [data-slot="progress-indicator"]) {
        background-color: rgb(37 99 235);
        transition: all 0.3s ease;
    }

    :global(.nvme-progress.nvme-critical[data-slot="progress"] [data-slot="progress-indicator"]) {
        background-color: rgb(220 38 38);
    }

    :global(.dark .nvme-progress[data-slot="progress"]) {
        background-color: rgb(59 130 246 / 0.2);
    }

    :global(.dark .nvme-progress[data-slot="progress"] [data-slot="progress-indicator"]) {
        background-color: rgb(59 130 246);
    }

    :global(.dark .nvme-progress.nvme-critical[data-slot="progress"] [data-slot="progress-indicator"]) {
        background-color: rgb(239 68 68);
    }

    :global(.text-white) {
        color: white !important;
    }

    :global(.dark .text-white) {
        color: rgb(229 231 235) !important;
    }
</style>

<div class="rounded-md border">
    <Table.Root>
        <Table.Header>
            <Table.Row>
                <Table.Head>Name</Table.Head>
                <Table.Head class="text-center">Bandwidth</Table.Head>
                <Table.Head class="text-center">GPU</Table.Head>
                <Table.Head class="text-center">SRAM</Table.Head>
                <Table.Head class="text-center">VRAM</Table.Head>
                <Table.Head class="text-center">NVMe</Table.Head>
                <Table.Head class="text-center">Temperature</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each replicants as replicant (replicant.replicantId)}
                {@const statusInfo = getStatusIconWithClass(replicant.status)}
                <Table.Row
                    class="cursor-pointer hover:bg-muted/50"
                    onclick={() => navigateToReplicant(replicant.replicantId)}
                >
                    <Table.Cell class="font-medium">
                        <div class="flex items-center gap-2">
                            <statusInfo.icon class="h-4 w-4 {statusInfo.class}" />
                            <span class="font-mono text-sm">
                                {replicant.replicantId}
                            </span>
                        </div>
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.networkBandwidth !== undefined}
                            <div class="flex items-center gap-2">
                                <Progress
                                    value={(replicant.currentMetrics.networkBandwidth / 200) * 100}
                                    class="w-16 h-1 nvme-progress{(replicant.currentMetrics.networkBandwidth / 200) * 100 >= 95 ? ' nvme-critical' : ''}"
                                />
                                <span class="text-xs font-mono text-gray-900 dark:text-white">
                                    {replicant.currentMetrics.networkBandwidth.toFixed(1)} Gbps
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center gap-2">
                                <div class="w-16 h-1 bg-muted rounded animate-pulse"></div>
                                <div class="w-12 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.gpuUsage !== undefined}
                            <div class="flex items-center gap-2">
                                <Progress
                                    value={replicant.currentMetrics.gpuUsage}
                                    class="w-16 h-1 nvme-progress{replicant.currentMetrics.gpuUsage >= 95 ? ' nvme-critical' : ''}"
                                />
                                <span class="text-xs font-mono text-gray-900 dark:text-white">
                                    {replicant.currentMetrics.gpuUsage.toFixed(1)}%
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center gap-2">
                                <div class="w-16 h-1 bg-muted rounded animate-pulse"></div>
                                <div class="w-8 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.memoryUsage !== undefined}
                            <div class="flex items-center gap-2">
                                <Progress
                                    value={replicant.currentMetrics.memoryUsage}
                                    class="w-16 h-1 nvme-progress{replicant.currentMetrics.memoryUsage >= 95 ? ' nvme-critical' : ''}"
                                />
                                <span class="text-xs font-mono text-gray-900 dark:text-white">
                                    {((replicant.currentMetrics.memoryUsage / 100) * (replicant.hardware?.memorySize || 128000) / 1000).toFixed(1)} GB
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center gap-2">
                                <div class="w-16 h-1 bg-muted rounded animate-pulse"></div>
                                <div class="w-10 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.gpuMemoryUsage !== undefined}
                            <div class="flex items-center gap-2">
                                <Progress
                                    value={replicant.currentMetrics.gpuMemoryUsage}
                                    class="w-16 h-1 nvme-progress{replicant.currentMetrics.gpuMemoryUsage >= 95 ? ' nvme-critical' : ''}"
                                />
                                <span class="text-xs font-mono text-gray-900 dark:text-white">
                                    {(replicant.currentMetrics.gpuMemoryUsage * 0.48).toFixed(1)} GB
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center gap-2">
                                <div class="w-16 h-1 bg-muted rounded animate-pulse"></div>
                                <div class="w-10 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.storageUsage !== undefined}
                            <div class="flex items-center gap-2">
                                <Progress
                                    value={replicant.currentMetrics.storageUsage}
                                    class="w-16 h-1 nvme-progress{replicant.currentMetrics.storageUsage >= 95 ? ' nvme-critical' : ''}"
                                />
                                <span class="text-xs font-mono text-gray-900 dark:text-white">
                                    {replicant.currentMetrics.storageUsage.toFixed(1)}%
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center gap-2">
                                <div class="w-16 h-1 bg-muted rounded animate-pulse"></div>
                                <div class="w-8 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>

                    <Table.Cell>
                        {#if replicant.currentMetrics?.temperature !== undefined}
                            <div class="flex items-center justify-center gap-1">
                                <ThermometerIcon class="h-3 w-3" />
                                <span
                                    class="text-sm font-mono {replicant.currentMetrics.temperature > 70
                                        ? 'text-red-600'
                                        : replicant.currentMetrics.temperature > 50
                                          ? 'text-yellow-600'
                                          : 'text-green-600'}"
                                >
                                    {replicant.currentMetrics.temperature}°C
                                </span>
                            </div>
                        {:else}
                            <div class="flex items-center justify-center gap-1">
                                <div class="w-3 h-3 bg-muted rounded animate-pulse"></div>
                                <div class="w-8 h-3 bg-muted rounded animate-pulse"></div>
                            </div>
                        {/if}
                    </Table.Cell>
                </Table.Row>
            {/each}
        </Table.Body>
    </Table.Root>
</div>
