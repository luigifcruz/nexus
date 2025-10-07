<script lang="ts">
    import { page } from "$app/stores";
    import HomeIcon from "@tabler/icons-svelte/icons/home";
    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import SatelliteDishIcon from "@lucide/svelte/icons/satellite-dish";
    import CpuIcon from "@tabler/icons-svelte/icons/cpu";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import ContainerIcon from "@tabler/icons-svelte/icons/container";
    import HelpIcon from "@tabler/icons-svelte/icons/help";
    import SearchIcon from "@tabler/icons-svelte/icons/search";
    import SettingsIcon from "@tabler/icons-svelte/icons/settings";
    import SunIcon from "@tabler/icons-svelte/icons/sun";
    import MoonIcon from "@tabler/icons-svelte/icons/moon";
    import CameraIcon from "@tabler/icons-svelte/icons/camera";
    import MenuIcon from "@tabler/icons-svelte/icons/menu";
    import NavMain from "./nav-main.svelte";
    import NavSecondary from "./nav-secondary.svelte";
    import NavDocuments from "./nav-documents.svelte";
    import ConnectionIndicator from "./connection-indicator.svelte";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { browser } from "$app/environment";
    import type { ComponentProps } from "svelte";

    // Track current path for active menu highlighting
    let currentPath = $derived($page.url.pathname);

    let isDark = $state(false);

    // Initialize theme from localStorage or system preference
    if (browser) {
        isDark =
            localStorage.getItem("theme") === "dark" ||
            (!localStorage.getItem("theme") &&
                window.matchMedia("(prefers-color-scheme: dark)").matches);
        updateTheme();
    }

    function toggleDarkMode() {
        isDark = !isDark;
        if (browser) {
            localStorage.setItem("theme", isDark ? "dark" : "light");
            updateTheme();
        }
    }

    function updateTheme() {
        if (browser) {
            if (isDark) {
                document.documentElement.classList.add("dark");
            } else {
                document.documentElement.classList.remove("dark");
            }
        }
    }

    const data = {
        navMain: [
            {
                title: "Home",
                url: "/",
                icon: HomeIcon,
            },
        ],
        services: [
            {
                name: "Observations",
                url: "/observations",
                icon: EyeIcon,
            },
            {
                name: "Replicants",
                url: "/replicants",
                icon: ServerIcon,
            },
            {
                name: "Images",
                url: "/images",
                icon: ContainerIcon,
            },
        ],
        systems: [
            {
                name: "Site Overview",
                url: "/site",
                icon: CameraIcon,
            },
            {
                name: "Signal Backend",
                url: "/signal-backend",
                icon: SatelliteDishIcon,
            },
            {
                name: "Compute Backend",
                url: "/compute-backend",
                icon: CpuIcon,
            },
        ],
        navSecondary: [
            {
                title: "Settings",
                url: "/settings",
                icon: SettingsIcon,
            },
            {
                title: "Documentation",
                url: "#",
                icon: HelpIcon,
            },
        ],
    };

    let { ...restProps }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root collapsible="offcanvas" {...restProps}>
    <Sidebar.Header>
        <Sidebar.Menu>
            <Sidebar.MenuItem>
                <Sidebar.MenuButton
                    class="data-[slot=sidebar-menu-button]:!p-1.5"
                >
                    {#snippet child({ props })}
                        <a href="/" {...props} class="flex flex-col pl-1.5 py-2">
                            <span class="text-3xl font-black tracking-wider">NEXUS</span>
                            <span class="text-sm font-medium text-gray-600 dark:text-gray-400 tracking-wide -mt-1"
                                >Allen Telescope Array</span
                            >
                        </a>
                    {/snippet}
                </Sidebar.MenuButton>
            </Sidebar.MenuItem>
        </Sidebar.Menu>
    </Sidebar.Header>
    <Sidebar.Content>
        <NavMain items={data.navMain} {currentPath} />
        <NavDocuments items={data.services} title="Services" {currentPath} />
        <NavDocuments items={data.systems} title="Systems" {currentPath} />
        <NavSecondary items={data.navSecondary} class="mt-auto" {currentPath} />
    </Sidebar.Content>
    <Separator />
    <Sidebar.Footer>
        <div class="flex items-center justify-between p-2">
            <Sidebar.Menu>
                <Sidebar.MenuItem>
                    <ConnectionIndicator />
                </Sidebar.MenuItem>
            </Sidebar.Menu>
            <Button
                variant="ghost"
                size="sm"
                onclick={toggleDarkMode}
                class="size-8 p-0"
            >
                {#if isDark}
                    <SunIcon class="h-4 w-4" />
                    <span class="sr-only">Switch to light mode</span>
                {:else}
                    <MoonIcon class="h-4 w-4" />
                    <span class="sr-only">Switch to dark mode</span>
                {/if}
            </Button>
        </div>
    </Sidebar.Footer>
</Sidebar.Root>
