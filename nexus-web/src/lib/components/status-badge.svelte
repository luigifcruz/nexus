<script lang="ts">
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { getStatusBadgeConfig, getStatusIcon } from "$lib/utils/status";
    import type { ComponentType } from "svelte";

    interface Props {
        status: string;
        class?: string;
        showIcon?: boolean;
        icon?: ComponentType;
    }

    let { status, class: className, showIcon = false, icon }: Props = $props();

    const config = $derived(getStatusBadgeConfig(status));
    const IconComponent = $derived(icon || (showIcon ? getStatusIcon(status) : null));
</script>

<Badge
    variant={config.variant}
    class={[config.className, className].filter(Boolean).join(" ")}
>
    {#if IconComponent}
        <IconComponent class="h-3 w-3" />
    {/if}
    {config.text}
</Badge>
