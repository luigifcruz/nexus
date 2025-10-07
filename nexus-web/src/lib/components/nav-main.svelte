<script lang="ts">
    import CirclePlusFilledIcon from "@tabler/icons-svelte/icons/circle-plus-filled";
    import MailIcon from "@tabler/icons-svelte/icons/mail";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import type { Icon } from "@tabler/icons-svelte";

    let { items, currentPath }: {
        items: { title: string; url: string; icon?: Icon }[];
        currentPath: string;
    } = $props();

    function isActive(url: string): boolean {
        if (url === "/") {
            return currentPath === "/";
        }
        return currentPath.startsWith(url);
    }
</script>

<Sidebar.Group>
    <Sidebar.GroupContent class="flex flex-col gap-2">
        <Sidebar.Menu>
            <Sidebar.MenuItem class="flex items-center gap-2">
                <Sidebar.MenuButton
                    class="bg-primary text-primary-foreground hover:bg-primary/90 hover:text-primary-foreground active:bg-primary/90 active:text-primary-foreground min-w-8 duration-200 ease-linear"
                    tooltipContent="Schedule Observation"
                    isActive={isActive("/observations/new")}
                >
                    {#snippet child({ props })}
                        <a href="/observations/new" {...props}>
                            <CirclePlusFilledIcon />
                            <span>Schedule Observation</span>
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
        <Sidebar.Menu>
            {#each items as item (item.title)}
                <Sidebar.MenuItem>
                    <Sidebar.MenuButton
                        tooltipContent={item.title}
                        isActive={isActive(item.url)}
                    >
                        {#snippet child({ props })}
                            <a href={item.url} {...props}>
                                <item.icon />
                                <span>{item.title}</span>
                            </a>
                        {/snippet}
                    </Sidebar.MenuButton>
                </Sidebar.MenuItem>
            {/each}
        </Sidebar.Menu>
    </Sidebar.GroupContent>
</Sidebar.Group>
