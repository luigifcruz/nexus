<script lang="ts">
    import { onMount } from "svelte";
    import * as d3 from "d3";

    interface DataPoint {
        label: string;
        x: number;
        y: number;
    }

    interface Props {
        data: DataPoint[];
        xLabel?: string;
        yLabel?: string;
        xDomain?: [number, number];
        yDomain?: [number, number];
        height?: number;
        referenceLineY?: number;
        colorScale?: d3.ScaleOrdinal<string, string>;
        xLog?: boolean;
    }

    let {
        data,
        xLabel = "",
        yLabel = "",
        xDomain,
        yDomain,
        height = 320,
        referenceLineY,
        colorScale,
        xLog = false,
    }: Props = $props();

    let container: HTMLDivElement;
    let width = $state(0);

    const margin = { top: 20, right: 20, bottom: 50, left: 60 };

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

    let defaultColorScale = $derived(
        d3.scaleOrdinal(d3.schemeTableau10).domain(data.map((d) => d.label))
    );

    let colors = $derived(colorScale || defaultColorScale);

    let xScale = $derived(
        xLog
            ? d3.scaleLog()
                .domain(xDomain || [d3.min(data, (d) => d.x) || 0.01, d3.max(data, (d) => d.x) || 100])
                .range([0, chartWidth])
                .nice()
            : d3.scaleLinear()
                .domain(xDomain || [0, d3.max(data, (d) => d.x) || 100])
                .range([0, chartWidth])
                .nice()
    );

    let yScale = $derived(
        d3.scaleLinear()
            .domain(yDomain || [0, d3.max(data, (d) => d.y) || 100])
            .range([chartHeight, 0])
            .nice()
    );

    let xTicks = $derived(xScale.ticks(6));
    let yTicks = $derived(yScale.ticks(5));

    // Tooltip state
    let hoveredPoint = $state<DataPoint | null>(null);
    let tooltipX = $state(0);
    let tooltipY = $state(0);

    function handleMouseEnter(d: DataPoint, event: MouseEvent) {
        hoveredPoint = d;
        const rect = container.getBoundingClientRect();
        tooltipX = event.clientX - rect.left;
        tooltipY = event.clientY - rect.top;
    }

    function handleMouseLeave() {
        hoveredPoint = null;
    }
</script>

<div bind:this={container} class="w-full relative" style="height: {height}px;">
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
                {#each yTicks as tick}
                    <line
                        x1={0}
                        x2={chartWidth}
                        y1={yScale(tick)}
                        y2={yScale(tick)}
                        stroke="currentColor"
                        stroke-opacity="0.1"
                    />
                {/each}

                <!-- Reference line Y -->
                {#if referenceLineY !== undefined}
                    <line
                        x1={0}
                        x2={chartWidth}
                        y1={yScale(referenceLineY)}
                        y2={yScale(referenceLineY)}
                        stroke="currentColor"
                        stroke-opacity="0.3"
                        stroke-dasharray="4,4"
                    />
                {/if}

                <!-- Points -->
                {#each data as d}
                    <circle
                        cx={xScale(d.x)}
                        cy={yScale(d.y)}
                        r={hoveredPoint === d ? 7 : 5}
                        fill={colors(d.label)}
                        fill-opacity={hoveredPoint === d ? 1 : 0.8}
                        stroke={colors(d.label)}
                        stroke-width="2"
                        class="cursor-pointer transition-all duration-150"
                        onmouseenter={(e) => handleMouseEnter(d, e)}
                        onmouseleave={handleMouseLeave}
                    />
                    <!-- Leader line (135 deg = up-left) -->
                    <line
                        x1={xScale(d.x) - 5}
                        y1={yScale(d.y) - 5}
                        x2={xScale(d.x) - 18}
                        y2={yScale(d.y) - 18}
                        stroke="currentColor"
                        stroke-opacity="0.3"
                        stroke-width="1"
                    />
                    <!-- Label -->
                    <text
                        x={xScale(d.x) - 22}
                        y={yScale(d.y) - 20}
                        text-anchor="end"
                        font-size="10"
                        fill="currentColor"
                        fill-opacity="0.8"
                        class="font-mono"
                    >
                        {d.label.length > 12 ? d.label.slice(0, 12) + "..." : d.label}
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
                            y={40}
                            text-anchor="middle"
                            font-size="12"
                            fill="currentColor"
                            fill-opacity="0.6"
                        >
                            {xLabel}
                        </text>
                    {/if}
                </g>

                <!-- Y axis -->
                <g>
                    <line y1={0} y2={chartHeight} stroke="currentColor" stroke-opacity="0.2" />
                    {#each yTicks as tick}
                        <text
                            x={-10}
                            y={yScale(tick)}
                            dy="0.35em"
                            text-anchor="end"
                            font-size="11"
                            fill="currentColor"
                            fill-opacity="0.6"
                        >
                            {tick}
                        </text>
                    {/each}
                    {#if yLabel}
                        <text
                            transform="rotate(-90)"
                            x={-chartHeight / 2}
                            y={-45}
                            text-anchor="middle"
                            font-size="12"
                            fill="currentColor"
                            fill-opacity="0.6"
                        >
                            {yLabel}
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

    <!-- Tooltip -->
    {#if hoveredPoint}
        <div
            class="absolute pointer-events-none z-10 px-3 py-2 rounded-lg bg-zinc-900/95 backdrop-blur-sm border border-white/10 shadow-lg"
            style="left: {tooltipX + 12}px; top: {tooltipY - 10}px;"
        >
            <div class="font-mono text-sm text-white font-medium">{hoveredPoint.label}</div>
            <div class="text-xs text-white/70 mt-1">
                <span>Score: {hoveredPoint.y.toFixed(1)}</span>
                <span class="mx-1">·</span>
                <span>Cost: ${hoveredPoint.x.toFixed(4)}</span>
            </div>
        </div>
    {/if}
</div>
