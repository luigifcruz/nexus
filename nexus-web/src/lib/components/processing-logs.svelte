<script lang="ts">
    export interface LogEntry {
        timestamp: Date | string | { seconds?: bigint | number };
        level: "info" | "warning" | "error" | "trace" | "debug" | "warn" | number;
        message: string;
    }

    interface Props {
        logs?: LogEntry[];
        maxHeight?: string;
        showTimestamp?: boolean;
    }

    let {
        logs = [],
        maxHeight = "max-h-100",
        showTimestamp = true
    }: Props = $props();

    // Use provided logs directly
    let displayLogs = $derived(logs || []);

    // Auto-scroll functionality
    let scrollContainer: HTMLElement;
    let isNearBottom = $state(true);
    let lastLogCount = $state(0);

    // Check if user is near bottom of scroll area
    function handleScroll() {
        if (!scrollContainer) return;
        const { scrollTop, scrollHeight, clientHeight } = scrollContainer;
        isNearBottom = scrollTop + clientHeight >= scrollHeight - 50; // 50px threshold
    }

    // Auto-scroll to bottom when new logs arrive (only if user was already at bottom)
    $effect(() => {
        if (displayLogs.length > lastLogCount && isNearBottom && scrollContainer) {
            scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
        lastLogCount = displayLogs.length;
    });

    function normalizeTimestamp(timestamp: Date | string | { seconds?: bigint | number }): Date {
        if (timestamp instanceof Date) {
            return timestamp;
        }
        if (typeof timestamp === "string") {
            return new Date(timestamp);
        }
        if (timestamp && typeof timestamp === "object" && timestamp.seconds) {
            const seconds = typeof timestamp.seconds === "bigint" ? Number(timestamp.seconds) : timestamp.seconds;
            return new Date(seconds * 1000);
        }
        return new Date();
    }

    function normalizeLevel(level: string | number): { name: string; color: string } {
        if (typeof level === "number") {
            const levelNames = ["TRACE", "DEBUG", "INFO", "WARN", "ERROR"];
            const name = levelNames[level] || "UNKNOWN";
            const color = level === 4 ? "text-red-600 dark:text-red-400"
                        : level === 3 ? "text-yellow-600 dark:text-yellow-400"
                        : "text-green-600 dark:text-green-400";
            return { name, color };
        }

        const normalizedLevel = level.toLowerCase();
        switch (normalizedLevel) {
            case "error":
                return { name: "ERROR", color: "text-red-600 dark:text-red-400" };
            case "warning":
            case "warn":
                return { name: "WARN", color: "text-yellow-600 dark:text-yellow-400" };
            case "info":
                return { name: "INFO", color: "text-green-600 dark:text-green-400" };
            case "debug":
                return { name: "DEBUG", color: "text-blue-600 dark:text-blue-400" };
            case "trace":
                return { name: "TRACE", color: "text-gray-600 dark:text-gray-400" };
            default:
                return { name: level.toUpperCase(), color: "text-gray-600 dark:text-gray-400" };
        }
    }
</script>

<div class="bg-gray-50 dark:bg-gray-900 rounded-md overflow-hidden border border-border">
    {#if displayLogs && displayLogs.length > 0}
        <div
            bind:this={scrollContainer}
            on:scroll={handleScroll}
            class="p-4 font-mono text-xs {maxHeight} overflow-y-auto space-y-1 min-h-48"
        >
            {#each displayLogs as log}
                {@const normalizedTimestamp = normalizeTimestamp(log.timestamp)}
                {@const levelInfo = normalizeLevel(log.level)}
                <div class="flex gap-3 items-start">
                    {#if showTimestamp}
                        <span class="text-gray-600 dark:text-gray-400 shrink-0">
                            {normalizedTimestamp.toLocaleTimeString("en-US", { hour12: false })}
                        </span>
                    {/if}
                    <span class="shrink-0 {levelInfo.color}">
                        [{levelInfo.name}]
                    </span>
                    <span class="text-gray-800 dark:text-gray-200">
                        {log.message}
                    </span>
                </div>
            {/each}
        </div>
    {:else}
        <div class="min-h-100 flex items-center justify-center">
            <p class="text-sm font-mono text-gray-500 dark:text-gray-400">No logs available</p>
        </div>
    {/if}
</div>
