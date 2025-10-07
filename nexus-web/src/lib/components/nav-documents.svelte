<script lang="ts">
    import type { Icon } from "@tabler/icons-svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";

    let { items, title, currentPath }: {
        items: { name: string; url: string; icon: Icon }[];
        title: string;
        currentPath: string;
    } = $props();

    function isActive(url: string): boolean {
        if (url === "/") {
            return currentPath === "/";
        }
        return currentPath.startsWith(url);
    }
</script>

<Sidebar.Group class="group-data-[collapsible=icon]:hidden">
    <Sidebar.GroupLabel>{title}</Sidebar.GroupLabel>
    <Sidebar.Menu>
        {#each items as item (item.name)}
            <Sidebar.MenuItem>
                <Sidebar.MenuButton isActive={isActive(item.url)}>
                    {#snippet child({ props })}
                        <a {...props} href={item.url}>
                            <item.icon />
                            <span>{item.name}</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        {/each}
    </Sidebar.Menu>
</Sidebar.Group>
