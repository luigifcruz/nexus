<script lang="ts">
    import { onMount } from "svelte";
    import * as d3 from "d3";

    interface DataPoint {
        label: string;
        values: { key: string; value: number }[];
    }

    interface Props {
        data: DataPoint[];
        colors?: string[];
        keys?: string[];
        xLabel?: string;
        xDomain?: [number, number];
        height?: number;
        formatValue?: (value: number) => string;
    }

    let {
        data,
        colors = ["#3b82f6", "#f59e0b"],
        keys = ["Input", "Output"],
        xLabel = "",
        xDomain,
        height = 320,
        formatValue = (v) => v.toFixed(0),
    }: Props = $props();

    let container: HTMLDivElement;
    let width = $state(0);

    const margin = { top: 30, right: 50, bottom: 40, left: 170 };

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

    let maxValue = $derived(
        d3.max(data, (d) => d3.max(d.values, (v) => v.value)) || 100
    );

    let xScale = $derived(
        d3.scaleLinear()
            .domain(xDomain || [0, maxValue])
            .range([0, chartWidth])
            .nice()
    );

    let yScale = $derived(
        d3.scaleBand<string>()
            .domain(data.map((d) => d.label))
            .range([0, chartHeight])
            .padding(0.2)
    );

    let ySubScale = $derived(
        d3.scaleBand<number>()
            .domain(d3.range(keys.length))
            .range([0, yScale.bandwidth()])
            .padding(0.1)
    );

    let xTicks = $derived(xScale.ticks(5));
</script>

<div bind:this={container} class="w-full" style="height: {height}px;">
    {#if width > 0 && data.length > 0}
        <svg {width} {height}>
            <g transform="translate({margin.left}, {margin.top})">
                <!-- Legend -->
                <g transform="translate({chartWidth - 120}, -20)">
                    {#each keys as key, i}
                        <g transform="translate({i * 70}, 0)">
                            <rect width="12" height="12" fill={colors[i]} rx="2" />
                            <text x="16" y="10" font-size="11" fill="currentColor" fill-opacity="0.7">
                                {key}
                            </text>
                        </g>
                    {/each}
                </g>

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

                <!-- Grouped bars -->
                {#each data as d}
                    {#each d.values as v, i}
                        <rect
                            x={0}
                            y={(yScale(d.label) || 0) + (ySubScale(i) || 0)}
                            width={Math.max(0, xScale(v.value))}
                            height={ySubScale.bandwidth()}
                            fill={colors[i]}
                            fill-opacity="0.8"
                            rx="2"
                        />
                        <!-- Value label -->
                        <text
                            x={xScale(v.value) + 6}
                            y={(yScale(d.label) || 0) + (ySubScale(i) || 0) + ySubScale.bandwidth() / 2}
                            dy="0.35em"
                            font-size="10"
                            fill="currentColor"
                            fill-opacity="0.7"
                        >
                            {formatValue(v.value)}
                        </text>
                    {/each}
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
