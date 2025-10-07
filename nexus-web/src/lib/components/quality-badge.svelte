<script lang="ts">
    import { Badge } from "$lib/components/ui/badge/index.js";

    interface Props {
        value: number;
        class?: string;
        thresholds?: {
            excellent: number;
            good: number;
        };
    }

    let {
        value,
        class: className,
        thresholds = { excellent: 0.9, good: 0.8 }
    }: Props = $props();

    const quality = $derived(
        value >= thresholds.excellent
            ? {
                label: "Excellent",
                variant: "outline" as const,
                className: "border-green-500 text-green-700 dark:text-green-400"
            }
            : value >= thresholds.good
            ? {
                label: "Good",
                variant: "outline" as const,
                className: "border-yellow-500 text-yellow-700 dark:text-yellow-400"
            }
            : {
                label: "Poor",
                variant: "outline" as const,
                className: "border-red-500 text-red-700 dark:text-red-400"
            }
    );
</script>

<Badge
    variant={quality.variant}
    class={[quality.className, className].filter(Boolean).join(" ")}
>
    {quality.label}
</Badge>
