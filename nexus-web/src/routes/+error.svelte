<script lang="ts">
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import HomeIcon from "@tabler/icons-svelte/icons/home";
    import ArrowLeftIcon from "@tabler/icons-svelte/icons/arrow-left";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import SearchIcon from "@tabler/icons-svelte/icons/search";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";

    function navigateHome() {
        goto("/");
    }

    function navigateBack() {
        history.back();
    }

    // Get error details
    $: status = $page.status;
    $: message = $page.error?.message || "An unexpected error occurred";

    // Determine error type and styling
    $: errorDetails = (() => {
        switch (status) {
            case 404:
                return {
                    title: "Page Not Found",
                    description: "The page you're looking for doesn't exist or has been moved.",
                    icon: SearchIcon,
                    color: "text-blue-600",
                    bgColor: "bg-blue-50 dark:bg-blue-900/20",
                    borderColor: "border-blue-200 dark:border-blue-800"
                };
            case 500:
                return {
                    title: "Server Error",
                    description: "Something went wrong on our end. Please try again later.",
                    icon: AlertTriangleIcon,
                    color: "text-red-600",
                    bgColor: "bg-red-50 dark:bg-red-900/20",
                    borderColor: "border-red-200 dark:border-red-800"
                };
            case 403:
                return {
                    title: "Access Forbidden",
                    description: "You don't have permission to access this resource.",
                    icon: AlertTriangleIcon,
                    color: "text-yellow-600",
                    bgColor: "bg-yellow-50 dark:bg-yellow-900/20",
                    borderColor: "border-yellow-200 dark:border-yellow-800"
                };
            default:
                return {
                    title: "Something Went Wrong",
                    description: "An unexpected error occurred. Please try again.",
                    icon: AlertTriangleIcon,
                    color: "text-gray-600",
                    bgColor: "bg-gray-50 dark:bg-gray-900/20",
                    borderColor: "border-gray-200 dark:border-gray-800"
                };
        }
    })();
</script>

<div class="min-h-screen flex flex-col items-center justify-center px-4 lg:px-6 py-12">
    <div class="max-w-lg w-full text-center space-y-8">
        <!-- Error Card -->
        <Card.Root class="bg-gray-50 dark:bg-gray-900/20 border-gray-200 dark:border-gray-800">
            <Card.Header class="pb-6">

                <div class="space-y-2">
                    <div class="flex items-center justify-center gap-2">
                        <Badge variant="outline" class="text-6xl px-6 py-3 {errorDetails.color} border-current font-bold">
                            {status}
                        </Badge>
                    </div>
                    <Card.Title class="text-2xl font-bold">
                        {errorDetails.title}
                    </Card.Title>
                    <Card.Description class="text-base">
                        {errorDetails.description}
                    </Card.Description>
                </div>
            </Card.Header>
            <Card.Content>
                <div class="flex flex-col sm:flex-row gap-3 justify-center">
                    <Button onclick={navigateHome} class="flex items-center gap-2">
                        <HomeIcon class="h-4 w-4" />
                        Go Home
                    </Button>
                    <Button variant="outline" onclick={navigateBack} class="flex items-center gap-2">
                        <ArrowLeftIcon class="h-4 w-4" />
                        Go Back
                    </Button>
                </div>
            </Card.Content>
        </Card.Root>

        <!-- Additional Help -->
        <div class="text-center space-y-3 mt-20">
            <h3 class="text-lg font-semibold text-muted-foreground">Need Help?</h3>
            <div class="flex flex-wrap justify-center gap-3 text-sm">
                <Button variant="link" onclick={() => goto("/")} class="text-muted-foreground hover:text-foreground">
                    Dashboard
                </Button>
                <Button variant="link" onclick={() => goto("/replicants")} class="text-muted-foreground hover:text-foreground">
                    Replicants
                </Button>
                <Button variant="link" onclick={() => goto("/observations")} class="text-muted-foreground hover:text-foreground">
                    Observations
                </Button>
                <Button variant="link" onclick={() => goto("/settings")} class="text-muted-foreground hover:text-foreground">
                    Settings
                </Button>
            </div>
        </div>

        <!-- Branding -->
        <div class="text-center pt-8 border-t border-border">
            <div class="text-sm text-muted-foreground">
                <span class="font-bold">NEXUS</span>
                <span class="mx-2">•</span>
                <span>Allen Telescope Array</span>
            </div>
        </div>
    </div>
</div>
