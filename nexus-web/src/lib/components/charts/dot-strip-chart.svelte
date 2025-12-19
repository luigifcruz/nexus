<script lang="ts">
    import { onMount } from "svelte";
    import * as d3 from "d3";

    interface DataPoint {
        label: string;
        value: number;
    }

    interface Props {
        data: DataPoint[];
        color?: string;
        xLabel?: string;
        xDomain?: [number, number];
        height?: number;
        referenceLine?: number;
    }

    let {
        data,
        color = "#3b82f6",
        xLabel = "",
        xDomain,
        height = 320,
        referenceLine,
    }: Props = $props();

    let container: HTMLDivElement;
    let width = $state(0);

    const margin = { top: 20, right: 40, bottom: 40, left: 170 };

    onMount(() => {
        const resizeObserver = new ResizeObserver((entries) => {
            for (const entry of entries) {
                width = entry.contentRect.width;
            }
        });
        resizeObserver.observe(container);
        return () => resizeObserver.disconnect();
    });

    let chartWidth = $derived(Math.max(0, width - margin.left - margin.right));
    let chartHeight = $derived(Math.max(0, height - margin.top - margin.bottom));

    let uniqueLabels = $derived([...new Set(data.map((d) => d.label))]);

    let xScale = $derived(
        d3.scaleLinear()
            .domain(xDomain || [0, d3.max(data, (d) => d.value) || 100])
            .range([0, chartWidth])
            .nice()
    );

    let yScale = $derived(
        d3.scaleBand<string>()
            .domain(uniqueLabels)
            .range([0, chartHeight])
            .padding(0.3)
    );

    let xTicks = $derived(xScale.ticks(5));
</script>

<div bind:this={container} class="w-full" style="height: {height}px;">
    {#if width > 0 && data.length > 0}
        <svg {width} {height}>
            <g transform="translate({margin.left}, {margin.top})">
                <!-- Grid lines -->
                {#each xTicks as tick}
                    <line
                        x1={xScale(tick)}
                        x2={xScale(tick)}
                        y1={0}
                        y2={chartHeight}
                        stroke="currentColor"
                        stroke-opacity="0.1"
                    />
                {/each}

                <!-- Reference line -->
                {#if referenceLine !== undefined}
                    <line
                        x1={xScale(referenceLine)}
                        x2={xScale(referenceLine)}
                        y1={0}
                        y2={chartHeight}
                        stroke="currentColor"
                        stroke-opacity="0.3"
                        stroke-dasharray="4,4"
                    />
                {/if}

                <!-- Dots -->
                {#each data as d}
                    <circle
                        cx={xScale(d.value)}
                        cy={(yScale(d.label) || 0) + yScale.bandwidth() / 2}
                        r={5}
                        fill={color}
                        fill-opacity="0.6"
                        stroke={color}
                        stroke-width="1"
                    />
                {/each}

                <!-- Y axis labels -->
                {#each uniqueLabels as label}
                    <text
                        x={-8}
                        y={(yScale(label) || 0) + yScale.bandwidth() / 2}
                        dy="0.35em"
                        text-anchor="end"
                        font-size="10"
                        fill="currentColor"
                        class="font-mono"
                    >
                        {label}
                    </text>
                {/each}

                <!-- X axis -->
                <g transform="translate(0, {chartHeight})">
                    <line x1={0} x2={chartWidth} stroke="currentColor" stroke-opacity="0.2" />
                    {#each xTicks as tick}
                        <text
                            x={xScale(tick)}
                            y={20}
                            text-anchor="middle"
                            font-size="11"
                            fill="currentColor"
                            fill-opacity="0.6"
                        >
                            {tick}
                        </text>
                    {/each}
                    {#if xLabel}
                        <text
                            x={chartWidth / 2}
                            y={35}
                            text-anchor="middle"
                            font-size="12"
                            fill="currentColor"
                            fill-opacity="0.6"
                        >
                            {xLabel}
                        </text>
                    {/if}
                </g>
            </g>
        </svg>
    {:else if data.length === 0}
        <div class="h-full flex items-center justify-center text-muted-foreground">
            No data available
        </div>
    {/if}
</div>
