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
        formatValue?: (value: number) => string;
    }

    let {
        data,
        color = "#3b82f6",
        xLabel = "",
        xDomain,
        height = 320,
        referenceLine,
        formatValue = (v) => v.toFixed(1),
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

    let xScale = $derived(
        d3.scaleLinear()
            .domain(xDomain || [0, d3.max(data, (d) => d.value) || 100])
            .range([0, chartWidth])
            .nice()
    );

    let yScale = $derived(
        d3.scaleBand<string>()
            .domain(data.map((d) => d.label))
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

                <!-- Bars -->
                {#each data as d}
                    <rect
                        x={0}
                        y={yScale(d.label)}
                        width={Math.max(0, xScale(d.value))}
                        height={yScale.bandwidth()}
                        fill={color}
                        fill-opacity="0.8"
                        rx="2"
                    />
                    <!-- Value label -->
                    <text
                        x={xScale(d.value) + 6}
                        y={(yScale(d.label) || 0) + yScale.bandwidth() / 2}
                        dy="0.35em"
                        font-size="11"
                        fill="currentColor"
                        fill-opacity="0.7"
                    >
                        {formatValue(d.value)}
                    </text>
                {/each}

                <!-- Y axis labels -->
                {#each data as d}
                    <text
                        x={-8}
                        y={(yScale(d.label) || 0) + yScale.bandwidth() / 2}
                        dy="0.35em"
                        text-anchor="end"
                        font-size="10"
                        fill="currentColor"
                        class="font-mono"
                    >
                        {d.label}
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
