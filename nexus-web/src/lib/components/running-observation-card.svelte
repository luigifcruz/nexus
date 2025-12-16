<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import type { ObservationData } from "$lib/types";
    import { goto } from "$app/navigation";
    import { calculateProgress } from "$lib/utils/observations";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import PlayIcon from "@tabler/icons-svelte/icons/player-play";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import SatelliteDishIcon from "@lucide/svelte/icons/satellite-dish";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";

    interface Props {
        observation: ObservationData;
    }

    let { observation }: Props = $props();

    let currentTime = $state(Date.now());
    let timer: number;

    onMount(() => {
        timer = setInterval(() => {
            currentTime = Date.now();
        }, 100);
    });

    onDestroy(() => {
        if (timer) {
            clearInterval(timer);
        }
    });

    function navigateToObservation(id: string) {
        goto(`/observations/${id}`);
    }

    const progress = $derived.by(() => {
        currentTime;
        return calculateProgress(observation);
    });
</script>

<Card.Root
    class="cursor-pointer transition-all hover:shadow-md hover:scale-[1.02] border-2 border-green-500/15 bg-gradient-to-br from-green-500/[0.03] to-green-500/[0.06]"
    onclick={() => navigateToObservation(observation.observationId)}
>
    <Card.Header>
        <Card.Title class="flex items-center gap-2">
            <div class="p-1.5 rounded-md bg-green-100 dark:bg-green-900/30">
                <PlayIcon class="h-4 w-4 text-green-600 dark:text-green-400" />
            </div>
            {observation.observationId}
        </Card.Title>
    </Card.Header>

    <Card.Content>
        <div class="space-y-4">
            <!-- TODO:API - Frequency not provided by API -->
            <!-- <div class="flex items-center justify-center p-4 bg-muted/50 rounded-lg">
                <span class="text-lg font-mono font-semibold text-primary">
                    Frequency data unavailable
                </span>
            </div> -->

            <!-- Progress Bar -->
            {#if progress > 0}
                <div class="space-y-2">
                    <div class="flex justify-between text-sm">
                        <span class="text-muted-foreground"
                            >Progress (30m left)</span
                        >
                        <span class="font-medium">{Math.round(progress)}%</span>
                    </div>
                    <Progress value={progress} class="h-2" />
                </div>
            {/if}

            <!-- Stats cells -->
            <div class="grid grid-cols-3 gap-2 text-sm">
                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div
                                class="flex items-center justify-center gap-1 p-2 rounded bg-muted/50"
                            >
                                <ServerIcon
                                    class="h-4 w-4 flex-shrink-0 text-blue-500"
                                />
                                <span class="font-mono font-medium text-xs">
                                    {observation.numberOfInstances}
                                </span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Number of Replicants</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>
                <!-- TODO:API - Number of Antennas not provided by API -->
                <!-- <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div class="flex items-center justify-center gap-1 p-2 rounded bg-muted/50">
                                <SatelliteDishIcon class="h-4 w-4 flex-shrink-0 text-green-500" />
                                <span class="font-mono font-medium text-xs">
                                    N/A
                                </span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Number of Antennas</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider> -->
                <!-- TODO:API - NVMe Usage not provided by API -->
                <!-- <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div class="flex items-center justify-center gap-1 p-2 rounded bg-muted/50">
                                <DatabaseIcon class="h-4 w-4 flex-shrink-0 text-purple-500" />
                                <span class="font-mono font-medium text-xs">
                                    N/A
                                </span>
                                <span class="text-muted-foreground text-xs">GB</span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>NVMe Usage (GB)</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider> -->
            </div>
        </div>
    </Card.Content>
</Card.Root>
