<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import SunIcon from "@tabler/icons-svelte/icons/sun";
    import MoonIcon from "@tabler/icons-svelte/icons/moon";
    import { browser } from "$app/environment";

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
</script>

<header
    class="h-(--header-height) group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height) flex shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear"
>
    <div class="flex w-full items-center gap-1 px-4 lg:gap-2 lg:px-6">
        <Sidebar.Trigger class="-ml-1" />
        <Separator
            orientation="vertical"
            class="mx-2 data-[orientation=vertical]:h-4"
        />
        <h1 class="text-base font-medium">Control System</h1>
        <div class="ml-auto flex items-center gap-2">
            <Button
                variant="ghost"
                size="sm"
                onclick={toggleDarkMode}
                class="size-9 p-0"
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
    </div>
</header>
