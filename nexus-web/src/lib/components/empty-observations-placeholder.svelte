<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import { Progress } from "$lib/components/ui/progress/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import PlayIcon from "@tabler/icons-svelte/icons/player-play";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import SatelliteDishIcon from "@lucide/svelte/icons/satellite-dish";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";
</script>

<Card.Root class="border-dashed border-2 opacity-60">
    <Card.Header>
        <Card.Title class="flex items-center gap-2 text-muted-foreground">
            <PlayIcon class="h-5 w-5 text-muted-foreground/50" />
            No running observations
        </Card.Title>
    </Card.Header>

    <Card.Content>
        <div class="space-y-4">
            <!-- Progress Bar Placeholder -->
            <div class="space-y-2">
                <div class="flex justify-between text-sm text-muted-foreground/70">
                    <span>Progress</span>
                    <span class="font-medium">--</span>
                </div>
                <Progress value={0} class="h-2 opacity-50" />
            </div>

            <!-- Stats cells placeholder -->
            <div class="grid grid-cols-3 gap-2 text-sm">
                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div class="flex items-center justify-center gap-1 p-2 rounded bg-muted/20 border-dashed border">
                                <ServerIcon class="h-4 w-4 flex-shrink-0 text-muted-foreground/50" />
                                <span class="font-mono font-medium text-xs text-muted-foreground/70">
                                    --
                                </span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Number of Replicants</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>

                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div class="flex items-center justify-center gap-1 p-2 rounded bg-muted/20 border-dashed border">
                                <SatelliteDishIcon class="h-4 w-4 flex-shrink-0 text-muted-foreground/50" />
                                <span class="font-mono font-medium text-xs text-muted-foreground/70">
                                    --
                                </span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>Number of Antennas</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>

                <Tooltip.Provider>
                    <Tooltip.Root>
                        <Tooltip.Trigger>
                            <div class="flex items-center justify-center gap-1 p-2 rounded bg-muted/20 border-dashed border">
                                <DatabaseIcon class="h-4 w-4 flex-shrink-0 text-muted-foreground/50" />
                                <span class="font-mono font-medium text-xs text-muted-foreground/70">
                                    --
                                </span>
                                <span class="text-muted-foreground/50 text-xs">GB</span>
                            </div>
                        </Tooltip.Trigger>
                        <Tooltip.Content>
                            <p>NVMe Usage (GB)</p>
                        </Tooltip.Content>
                    </Tooltip.Root>
                </Tooltip.Provider>
            </div>
        </div>
    </Card.Content>
</Card.Root>
