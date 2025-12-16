<script lang="ts">
    import StatusBadge from "$lib/components/status-badge.svelte";
    import QualityBadge from "$lib/components/quality-badge.svelte";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Table from "$lib/components/ui/table/index.js";

    // Icons
    import TrendingUpIcon from "@tabler/icons-svelte/icons/trending-up";
    import TrendingDownIcon from "@tabler/icons-svelte/icons/trending-down";

    // Test data
    const statusValues = [
        "running",
        "online",
        "stopped",
        "completed",
        "errored",
        "error",
        "failed",
        "standby",
        "starting",
        "stopping",
        "maintenance",
        "scheduled",
        "pending",
        "offline",
        "unknown",
    ];

    const correlationSamples = [
        { id: "BL-001", correlation: 0.95, quality: "Excellent" },
        { id: "BL-002", correlation: 0.87, quality: "Good" },
        { id: "BL-003", correlation: 0.73, quality: "Poor" },
    ];

    const sampleTableData = [
        { id: "OBS-001", name: "Pulsar Survey Alpha", status: "running" },
        { id: "OBS-002", name: "Fast Radio Burst Hunt", status: "completed" },
        { id: "OBS-003", name: "Galaxy Mapping", status: "errored" },
        { id: "OBS-004", name: "Calibration Run", status: "standby" },
        { id: "OBS-005", name: "Data Archive Task", status: "stopped" },
    ];
</script>

<svelte:head>
    <title>Nexus - Test Style</title>
</svelte:head>

<div class="flex flex-col gap-8 py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div>
            <h1 class="text-4xl font-bold tracking-tight">Badge System Test</h1>
            <p class="text-muted-foreground mt-2">
                Testing all badge components and variants
            </p>
        </div>
    </div>

    <div class="px-4 lg:px-6 space-y-8">
        <!-- Status Badges -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Status Badges</Card.Title>
                <Card.Description
                    >Unified transparent badge system with consistent styling</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-6">
                <!-- All status states -->
                <div>
                    <h4 class="font-semibold mb-3">All Status States</h4>
                    <div
                        class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4"
                    >
                        {#each statusValues as status}
                            <div
                                class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                            >
                                <StatusBadge {status} />
                                <StatusBadge {status} showIcon />
                                <code class="text-xs text-muted-foreground"
                                    >{status}</code
                                >
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- In table context -->
                <div>
                    <h4 class="font-semibold mb-3">Status Badges in Tables</h4>
                    <div class="rounded-md border">
                        <Table.Root>
                            <Table.Header>
                                <Table.Row>
                                    <Table.Head>ID</Table.Head>
                                    <Table.Head>Name</Table.Head>
                                    <Table.Head>Status</Table.Head>
                                    <Table.Head>Status + Icon</Table.Head>
                                </Table.Row>
                            </Table.Header>
                            <Table.Body>
                                {#each sampleTableData as row}
                                    <Table.Row class="hover:bg-muted/50">
                                        <Table.Cell class="font-mono text-xs"
                                            >{row.id}</Table.Cell
                                        >
                                        <Table.Cell class="font-medium"
                                            >{row.name}</Table.Cell
                                        >
                                        <Table.Cell>
                                            <StatusBadge status={row.status} />
                                        </Table.Cell>
                                        <Table.Cell>
                                            <StatusBadge
                                                status={row.status}
                                                showIcon
                                            />
                                        </Table.Cell>
                                    </Table.Row>
                                {/each}
                            </Table.Body>
                        </Table.Root>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Quality Badges -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Quality Badges</Card.Title>
                <Card.Description
                    >For correlation analysis and quality indicators</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-6">
                <div>
                    <h4 class="font-semibold mb-3">Correlation Quality</h4>
                    <div class="grid grid-cols-3 gap-4">
                        {#each correlationSamples as sample}
                            <div
                                class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                            >
                                <QualityBadge value={sample.correlation} />
                                <code class="text-xs text-muted-foreground"
                                    >{sample.correlation}</code
                                >
                                <span class="text-xs text-muted-foreground"
                                    >{sample.id}</span
                                >
                            </div>
                        {/each}
                    </div>
                </div>

                <!-- Test different thresholds -->
                <div>
                    <h4 class="font-semibold mb-3">Custom Thresholds</h4>
                    <div class="grid grid-cols-4 gap-4">
                        <div
                            class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                        >
                            <QualityBadge value={0.85} />
                            <code class="text-xs text-muted-foreground"
                                >0.85 (default)</code
                            >
                        </div>
                        <div
                            class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                        >
                            <QualityBadge
                                value={0.85}
                                thresholds={{ excellent: 0.95, good: 0.85 }}
                            />
                            <code class="text-xs text-muted-foreground"
                                >0.85 (strict)</code
                            >
                        </div>
                        <div
                            class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                        >
                            <QualityBadge
                                value={0.75}
                                thresholds={{ excellent: 0.8, good: 0.6 }}
                            />
                            <code class="text-xs text-muted-foreground"
                                >0.75 (lenient)</code
                            >
                        </div>
                        <div
                            class="flex flex-col items-center gap-2 p-3 rounded-lg border bg-muted/20"
                        >
                            <QualityBadge value={0.95} class="text-xs" />
                            <code class="text-xs text-muted-foreground"
                                >0.95 (small)</code
                            >
                        </div>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Other Badge Types -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Other Badge Types</Card.Title>
                <Card.Description
                    >Standard badges for labels, counts, and indicators</Card.Description
                >
            </Card.Header>
            <Card.Content class="space-y-6">
                <div>
                    <h4 class="font-semibold mb-3">Label Badges</h4>
                    <div class="flex flex-wrap gap-2">
                        <Badge variant="outline">Draft</Badge>
                        <Badge variant="outline">Production</Badge>
                        <Badge variant="outline">v2.1.0</Badge>
                        <Badge variant="outline">Beta</Badge>
                        <Badge variant="outline" class="font-mono"
                            >N6MCC51544</Badge
                        >
                    </div>
                </div>

                <div>
                    <h4 class="font-semibold mb-3">Trend Badges</h4>
                    <div class="flex flex-wrap gap-2">
                        <Badge variant="outline">
                            <TrendingUpIcon class="h-3 w-3" />
                            +12.5%
                        </Badge>
                        <Badge variant="outline">
                            <TrendingDownIcon class="h-3 w-3" />
                            -8.2%
                        </Badge>
                        <Badge variant="outline">
                            <TrendingUpIcon class="h-3 w-3" />
                            +4.5%
                        </Badge>
                    </div>
                </div>

                <div>
                    <h4 class="font-semibold mb-3">Count Badges</h4>
                    <div class="flex flex-wrap gap-2">
                        <Badge variant="secondary">5</Badge>
                        <Badge variant="secondary">23</Badge>
                        <Badge variant="secondary">147</Badge>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Design Principles -->
        <Card.Root>
            <Card.Header>
                <Card.Title>Design Principles</Card.Title>
                <Card.Description
                    >Badge system rules and color meanings</Card.Description
                >
            </Card.Header>
            <Card.Content>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                        <h4 class="font-semibold mb-2">Status Colors</h4>
                        <ul class="space-y-1 text-sm text-muted-foreground">
                            <li>
                                <strong class="text-foreground">Green:</strong> running,
                                online
                            </li>
                            <li>
                                <strong class="text-foreground">Blue:</strong> completed,
                                stopped
                            </li>
                            <li>
                                <strong class="text-foreground">Red:</strong> errored,
                                error, failed
                            </li>
                            <li>
                                <strong class="text-foreground">Yellow:</strong> standby,
                                starting, stopping, maintenance, scheduled, pending
                            </li>
                            <li>
                                <strong class="text-foreground">Gray:</strong> offline,
                                unknown
                            </li>
                        </ul>
                    </div>
                    <div>
                        <h4 class="font-semibold mb-2">Design Rules</h4>
                        <ul class="space-y-1 text-sm text-muted-foreground">
                            <li>
                                <strong class="text-foreground"
                                    >All transparent:</strong
                                > outline variant only
                            </li>
                            <li>
                                <strong class="text-foreground"
                                    >Consistent colors:</strong
                                > semantic meaning
                            </li>
                            <li>
                                <strong class="text-foreground"
                                    >Auto capitalization:</strong
                                > text formatting
                            </li>
                            <li>
                                <strong class="text-foreground"
                                    >Optional icons:</strong
                                > showIcon prop
                            </li>
                        </ul>
                    </div>
                </div>

                <div class="mt-6 pt-4 border-t">
                    <h4 class="font-semibold mb-2">Usage Examples</h4>
                    <div
                        class="bg-muted/50 p-3 rounded text-sm font-mono space-y-1"
                    >
                        <div>{`<StatusBadge status="running" />`}</div>
                        <div>{`<StatusBadge status="running" showIcon />`}</div>
                        <div>{`<QualityBadge value={0.85} />`}</div>
                        <div>{`<Badge variant="outline">Label</Badge>`}</div>
                    </div>
                </div>
            </Card.Content>
        </Card.Root>
    </div>
</div>
