<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { nexusClient } from "$lib/api/nexus";
    import { dataStore } from "$lib/stores/data.svelte";
    import type { Replicant, Image } from "$lib/proto/generic_pb";

    import { Button } from "$lib/components/ui/button/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";

    import CalendarIcon from "@tabler/icons-svelte/icons/calendar";
    import ClockIcon from "@tabler/icons-svelte/icons/clock";

    import SettingsIcon from "@tabler/icons-svelte/icons/settings";
    import ArrowLeftIcon from "@tabler/icons-svelte/icons/arrow-left";
    import PlayIcon from "@tabler/icons-svelte/icons/player-play";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";
    import CheckIcon from "@tabler/icons-svelte/icons/check";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import WorldIcon from "@tabler/icons-svelte/icons/world";
    import BoltIcon from "@tabler/icons-svelte/icons/bolt";
    import ServerIcon from "@tabler/icons-svelte/icons/server";
    import TagIcon from "@tabler/icons-svelte/icons/tag";
    import ClipboardIcon from "@tabler/icons-svelte/icons/clipboard";

    // State
    let loading = $state(false);
    let validating = $state(false);
    let error = $state<string | null>(null);
    let successMessage = $state<string | null>(null);
    let replicants = $state<Replicant[]>([]);
    let images = $state<Image[]>([]);

    // Form state
    let selectedImage = $state<string>("");
    let numberOfReplicants = $state(1);
    let replicantTags = $state("");
    let startDate = $state("");
    let startTime = $state("");
    let endDate = $state("");
    let endTime = $state("");
    let currentUTCTime = $state("");
    let currentLocalTime = $state("");

    // Validation
    let validationResult = $state<{ success: boolean; message: string } | null>(
        null,
    );
    let validationTimeout: NodeJS.Timeout | null = null;

    // Derived values
    let connectionStatus = $derived(dataStore.connectionStatus);
    let runningReplicants = $derived(
        dataStore.replicants.filter((r) => r.status === "running"),
    );
    let maxReplicants = $derived(runningReplicants.length);

    function getStartDateTime() {
        if (!startDate || !startTime) return null;
        const dateStr = `${startDate}T${startTime}Z`;
        const date = new Date(dateStr);
        return isNaN(date.getTime()) ? null : date;
    }

    function getEndDateTime() {
        if (!endDate || !endTime) return null;
        const dateStr = `${endDate}T${endTime}Z`;
        const date = new Date(dateStr);
        return isNaN(date.getTime()) ? null : date;
    }

    function getDuration() {
        const start = getStartDateTime();
        const end = getEndDateTime();
        if (!start || !end) return null;
        const diff = end.getTime() - start.getTime();
        if (diff <= 0) return null;
        return diff / (1000 * 60); // minutes as decimal
    }

    function formatDuration(minutes) {
        if (!minutes) return null;
        if (minutes >= 1) {
            return `${Math.round(minutes)} minutes`;
        } else {
            const seconds = Math.round(minutes * 60);
            return `${seconds} seconds`;
        }
    }

    let isFormValid = $derived(() => {
        return (
            selectedImage &&
            numberOfReplicants > 0 &&
            numberOfReplicants <= maxReplicants &&
            getStartDateTime() &&
            getEndDateTime() &&
            getDuration() &&
            getDuration() > 0
        );
    });

    onMount(() => {
        updateCurrentTime();
        const interval = setInterval(updateCurrentTime, 1000);

        // Set default times
        const now = new Date();
        startDate = now.toISOString().split("T")[0];
        startTime = now.toISOString().slice(11, 19);

        const oneHourLater = new Date(now.getTime() + 60 * 60 * 1000);
        endDate = oneHourLater.toISOString().split("T")[0];
        endTime = oneHourLater.toISOString().slice(11, 19);

        loadData();

        return () => clearInterval(interval);
    });

    function updateCurrentTime() {
        const now = new Date();
        currentUTCTime =
            now.toISOString().slice(0, 19).replace("T", " ") + " UTC";
        currentLocalTime = now.toLocaleTimeString([], {
            hour12: false,
            timeZoneName: "short",
        });
    }

    async function loadData() {
        loading = true;
        error = null;

        try {
            const [replicantsData, imagesData] = await Promise.all([
                nexusClient.listReplicants(),
                nexusClient.listImages(),
            ]);

            replicants = replicantsData;
            images = imagesData;

            // Set default image if available
            if (images.length > 0 && !selectedImage) {
                selectedImage = images[0].imageId;
            }
        } catch (err) {
            error = err instanceof Error ? err.message : "Failed to load data";
            console.error("Error loading data:", err);
        } finally {
            loading = false;
        }
    }

    async function validateObservation() {
        const startDateTime = getStartDateTime();
        const endDateTime = getEndDateTime();
        if (!isFormValid || !startDateTime || !endDateTime || !selectedImage) {
            validationResult = null;
            return;
        }

        validating = true;
        validationResult = null;

        try {
            const tags = replicantTags
                .split(",")
                .map((t) => t.trim())
                .filter((t) => t.length > 0);

            // For validation-only requests, we don't expect an observation ID
            // The createObservation method will throw if validation fails
            try {
                await nexusClient.createObservation({
                    startTime: startDateTime,
                    endTime: endDateTime,
                    imageId: selectedImage,
                    numberOfReplicants,
                    replicantTags: tags,
                    validateOnly: true,
                });
            } catch (validationError) {
                // Check if this is the "No observation ID returned" error for validation
                if (validationError.message === "No observation ID returned") {
                    // This is expected for validation-only requests, treat as success
                    validationResult = {
                        success: true,
                        message: "Configuration is valid!",
                    };
                    return;
                } else {
                    // This is a real validation error
                    throw validationError;
                }
            }

            validationResult = {
                success: true,
                message: "Configuration is valid!",
            };
        } catch (err) {
            const message =
                err instanceof Error ? err.message : "Validation failed";
            validationResult = { success: false, message };
        } finally {
            validating = false;
        }
    }

    // Debounced validation - triggers when form inputs change
    function triggerValidation() {
        if (validationTimeout) {
            clearTimeout(validationTimeout);
        }

        validationTimeout = setTimeout(() => {
            validateObservation();
        }, 800); // 800ms debounce
    }

    // Reactive validation when form changes
    $effect(() => {
        // Watch form fields
        selectedImage;
        numberOfReplicants;
        replicantTags;
        startDate;
        startTime;
        endDate;
        endTime;

        // Trigger debounced validation
        triggerValidation();
    });

    async function handleSchedule() {
        const startDateTime = getStartDateTime();
        const endDateTime = getEndDateTime();
        if (!isFormValid || !startDateTime || !endDateTime || !selectedImage)
            return;

        loading = true;
        error = null;
        successMessage = null;

        try {
            const tags = replicantTags
                .split(",")
                .map((t) => t.trim())
                .filter((t) => t.length > 0);

            const newObservation = await nexusClient.createObservation({
                startTime: startDateTime,
                endTime: endDateTime,
                imageId: selectedImage,
                numberOfReplicants,
                replicantTags: tags,
                validateOnly: false,
            });

            successMessage = `Observation ${newObservation} scheduled successfully!`;

            // Navigate to the observation after a brief delay
            setTimeout(() => {
                goto(`/observations/${newObservation}`);
            }, 2000);
        } catch (err) {
            error =
                err instanceof Error
                    ? err.message
                    : "Failed to schedule observation";
            console.error("Error scheduling observation:", err);
        } finally {
            loading = false;
        }
    }

    function handleBack() {
        goto("/observations");
    }

    function quickSetDuration(minutes: number) {
        const startDateTime = getStartDateTime();
        if (
            !startDateTime ||
            !(startDateTime instanceof Date) ||
            isNaN(startDateTime.getTime())
        )
            return;
        const newEndTime = new Date(
            startDateTime.getTime() + minutes * 60 * 1000,
        );
        endDate = newEndTime.toISOString().split("T")[0];
        endTime = newEndTime.toISOString().slice(11, 19);
    }

    function scheduleInSeconds(seconds: number) {
        // Get duration first
        const duration = getDuration();

        // Get current time (now)
        const timeNow = new Date();

        // Add delay to current time to get start time
        const startTimeDate = new Date(timeNow.getTime() + seconds * 1000);

        // Set start time
        startDate = startTimeDate.toISOString().split("T")[0];
        startTime = startTimeDate.toISOString().slice(11, 19);

        // Add duration to start time to get stop time
        if (duration) {
            const endTimeDate = new Date(
                startTimeDate.getTime() + duration * 60 * 1000,
            );
            endDate = endTimeDate.toISOString().split("T")[0];
            endTime = endTimeDate.toISOString().slice(11, 19);
        }
    }
</script>

<svelte:head>
    <title>Nexus - New Observation</title>
</svelte:head>

<div class="flex flex-col gap-6 py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div class="flex items-center gap-4">
            <Button variant="ghost" size="sm" onclick={handleBack}>
                <ArrowLeftIcon class="h-4 w-4" />
            </Button>
            <div class="flex-1">
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">
                        Schedule Observation
                    </h1>
                    <p class="text-muted-foreground">
                        Configure and schedule a new observation
                    </p>
                </div>
            </div>
        </div>
    </div>

    <!-- Success Message -->
    {#if successMessage}
        <div class="px-4 lg:px-6">
            <div
                class="rounded-lg border border-green-500/50 bg-green-50 dark:bg-green-900/20 p-4"
            >
                <div class="flex items-center gap-3">
                    <CheckIcon
                        class="h-5 w-5 text-green-600 dark:text-green-400"
                    />
                    <div>
                        <p
                            class="font-semibold text-green-800 dark:text-green-300"
                        >
                            {successMessage}
                        </p>
                        <p class="text-sm text-green-700 dark:text-green-400">
                            Redirecting to observation details...
                        </p>
                    </div>
                </div>
            </div>
        </div>
    {/if}

    <!-- Error Message -->
    {#if error}
        <div class="px-4 lg:px-6">
            <div
                class="rounded-lg border border-red-500/50 bg-red-50 dark:bg-red-900/20 p-4"
            >
                <div class="flex items-center gap-3">
                    <AlertTriangleIcon
                        class="h-5 w-5 text-red-600 dark:text-red-400"
                    />
                    <div>
                        <p class="font-semibold text-red-800 dark:text-red-300">
                            Error
                        </p>
                        <p class="text-sm text-red-700 dark:text-red-400">
                            {error}
                        </p>
                    </div>
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => (error = null)}
                        class="ml-auto"
                    >
                        <ArrowLeftIcon class="h-4 w-4 rotate-90" />
                    </Button>
                </div>
            </div>
        </div>
    {/if}

    <div class="px-4 lg:px-6">
        <div class="grid gap-6 @5xl/main:grid-cols-3">
            <!-- Main Form -->
            <div class="@5xl/main:col-span-2 space-y-6">
                <!--Configuration -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <SettingsIcon class="h-5 w-5" />
                            Configuration
                        </Card.Title>
                        <Card.Description>
                            Choose the image and configure replicant deployment
                        </Card.Description>
                    </Card.Header>
                    <Card.Content class="space-y-6">
                        <!-- Image Selection -->
                        <div class="space-y-3">
                            <Label class="text-sm font-medium"
                                >Image <span class="text-red-500">*</span
                                ></Label
                            >
                            {#if images.length > 0}
                                <select
                                    bind:value={selectedImage}
                                    class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                                >
                                    <option value="">Select an image</option>
                                    {#each images as image}
                                        <option value={image.imageId}
                                            >{image.imageId}</option
                                        >
                                    {/each}
                                </select>
                                <p class="text-xs text-muted-foreground">
                                    Image that will run on each replicant
                                </p>
                            {:else}
                                <div
                                    class="flex items-center gap-2 p-4 border rounded-md bg-muted/50"
                                >
                                    <AlertTriangleIcon
                                        class="h-4 w-4 text-yellow-600"
                                    />
                                    <span class="text-sm"
                                        >No images available</span
                                    >
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        onclick={loadData}
                                    >
                                        <RefreshIcon class="h-3 w-3" />
                                    </Button>
                                </div>
                            {/if}
                        </div>

                        <!-- Replicant Configuration -->
                        <div class="grid gap-4 @lg/main:grid-cols-2">
                            <div class="space-y-3">
                                <Label class="text-sm font-medium"
                                    >Replicant Count <span class="text-red-500"
                                        >*</span
                                    ></Label
                                >
                                <Input
                                    type="number"
                                    min="1"
                                    max={maxReplicants}
                                    bind:value={numberOfReplicants}
                                    placeholder="1"
                                    class="[&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none [-moz-appearance:textfield]"
                                />
                                <div
                                    class="flex items-center gap-2 text-xs text-muted-foreground"
                                >
                                    <ServerIcon class="h-3 w-3" />
                                    <span
                                        >{numberOfReplicants} of {maxReplicants} available
                                        replicants</span
                                    >
                                </div>
                            </div>

                            <div class="space-y-3">
                                <Label class="text-sm font-medium"
                                    >Replicant Tags</Label
                                >
                                <Input
                                    bind:value={replicantTags}
                                    placeholder="gpu, high-memory, nvme"
                                />
                                <div
                                    class="flex items-center gap-2 text-xs text-muted-foreground"
                                >
                                    <TagIcon class="h-3 w-3" />
                                    <span>Optional replicant capabilities</span>
                                </div>
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>

                <!-- Observation Schedule -->
                <Card.Root>
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <CalendarIcon class="h-5 w-5" />
                            Observation Schedule
                        </Card.Title>
                        <Card.Description>
                            Set the observation start and end times in UTC
                        </Card.Description>
                    </Card.Header>
                    <Card.Content class="space-y-6">
                        <!-- Quick Actions -->
                        <div class="grid gap-4 @lg/main:grid-cols-2">
                            <!-- Quick Start -->
                            <div
                                class="p-4 rounded-lg border bg-gradient-to-r from-blue-50/50 to-purple-50/50 dark:from-blue-950/30 dark:to-purple-950/30"
                            >
                                <div class="space-y-3">
                                    <div class="flex items-center gap-2">
                                        <BoltIcon
                                            class="h-4 w-4 text-blue-600"
                                        />
                                        <span class="font-medium text-sm"
                                            >Quick Start</span
                                        >
                                    </div>

                                    <div class="flex flex-wrap gap-1">
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => scheduleInSeconds(5)}
                                            class="h-7 px-2 text-xs">5s</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() =>
                                                scheduleInSeconds(15)}
                                            class="h-7 px-2 text-xs">15s</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() =>
                                                scheduleInSeconds(30)}
                                            class="h-7 px-2 text-xs">30s</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() =>
                                                scheduleInSeconds(60)}
                                            class="h-7 px-2 text-xs">60s</Button
                                        >
                                    </div>
                                </div>
                            </div>

                            <!-- Quick Duration -->
                            <div
                                class="p-4 rounded-lg border bg-gradient-to-r from-green-50/50 to-emerald-50/50 dark:from-green-950/30 dark:to-emerald-950/30"
                            >
                                <div class="space-y-3">
                                    <div class="flex items-center gap-2">
                                        <ClockIcon
                                            class="h-4 w-4 text-green-600"
                                        />
                                        <span class="font-medium text-sm"
                                            >Quick Duration</span
                                        >
                                    </div>

                                    <div class="flex flex-wrap gap-1">
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() =>
                                                quickSetDuration(0.25)}
                                            class="h-7 px-2 text-xs">15s</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => quickSetDuration(1)}
                                            class="h-7 px-2 text-xs">60s</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => quickSetDuration(5)}
                                            class="h-7 px-2 text-xs">5m</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => quickSetDuration(15)}
                                            class="h-7 px-2 text-xs">15m</Button
                                        >
                                        <Button
                                            variant="outline"
                                            size="sm"
                                            onclick={() => quickSetDuration(30)}
                                            class="h-7 px-2 text-xs">30m</Button
                                        >
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Time Configuration -->
                        <div class="space-y-4">
                            <!-- Start Time -->
                            <div class="space-y-3">
                                <Label class="text-sm font-medium"
                                    >Start Time <span class="text-red-500"
                                        >*</span
                                    ></Label
                                >
                                <div class="grid gap-3 @sm/main:grid-cols-2">
                                    <div class="space-y-1">
                                        <Label
                                            class="text-xs text-muted-foreground"
                                            >Date</Label
                                        >
                                        <Input
                                            type="date"
                                            bind:value={startDate}
                                        />
                                    </div>
                                    <div class="space-y-1">
                                        <Label
                                            class="text-xs text-muted-foreground"
                                            >Time (UTC)</Label
                                        >
                                        <Input
                                            type="time"
                                            step="1"
                                            bind:value={startTime}
                                        />
                                    </div>
                                </div>
                            </div>

                            <!-- End Time -->
                            <div class="space-y-3">
                                <Label class="text-sm font-medium"
                                    >End Time <span class="text-red-500">*</span
                                    ></Label
                                >
                                <div class="grid gap-3 @sm/main:grid-cols-2">
                                    <div class="space-y-1">
                                        <Label
                                            class="text-xs text-muted-foreground"
                                            >Date</Label
                                        >
                                        <Input
                                            type="date"
                                            bind:value={endDate}
                                        />
                                    </div>
                                    <div class="space-y-1">
                                        <Label
                                            class="text-xs text-muted-foreground"
                                            >Time (UTC)</Label
                                        >
                                        <Input
                                            type="time"
                                            step="1"
                                            bind:value={endTime}
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>
            </div>

            <!-- Sidebar -->
            <div
                class="space-y-6 sticky top-4 self-start max-h-screen overflow-y-auto"
            >
                <!-- Observatory Time -->
                <Card.Root
                    class="border-2 border-primary/20 bg-gradient-to-br from-primary/5 to-primary/10"
                >
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            <WorldIcon class="h-6 w-6 text-primary" />
                            Observatory Time
                        </Card.Title>
                    </Card.Header>
                    <Card.Content>
                        <div class="text-center space-y-2">
                            <div
                                class="text-3xl font-mono font-bold text-primary"
                            >
                                {currentUTCTime}
                            </div>
                            <div
                                class="text-sm font-mono text-muted-foreground"
                            >
                                {currentLocalTime}
                            </div>
                        </div>
                    </Card.Content>
                </Card.Root>

                <!-- Summary -->
                <Card.Root
                    class={validating
                        ? "border-blue-200 bg-blue-50/50 dark:border-blue-800 dark:bg-blue-950/30"
                        : validationResult?.success
                          ? "border-green-200 bg-green-50/50 dark:border-green-800 dark:bg-green-950/30"
                          : validationResult && !validationResult.success
                            ? "border-red-200 bg-red-50/50 dark:border-red-800 dark:bg-red-950/30"
                            : ""}
                >
                    <Card.Header>
                        <Card.Title class="flex items-center gap-2">
                            {#if validating}
                                <RefreshIcon
                                    class="h-5 w-5 animate-spin text-blue-600"
                                />
                                Summary
                                <Badge
                                    variant="secondary"
                                    class="ml-auto text-xs bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300"
                                    >Validating</Badge
                                >
                            {:else if validationResult?.success}
                                <CheckIcon class="h-5 w-5 text-green-600" />
                                Summary
                                <Badge
                                    variant="secondary"
                                    class="ml-auto text-xs bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300"
                                    >Valid</Badge
                                >
                            {:else if validationResult && !validationResult.success}
                                <AlertTriangleIcon
                                    class="h-5 w-5 text-red-600"
                                />
                                Summary
                                <Badge
                                    variant="secondary"
                                    class="ml-auto text-xs bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300"
                                    >Invalid</Badge
                                >
                            {:else}
                                <ClipboardIcon class="h-5 w-5" />
                                Summary
                            {/if}
                        </Card.Title>
                    </Card.Header>
                    <Card.Content class="space-y-3 text-sm">
                        <div class="flex justify-between items-center">
                            <span class="text-muted-foreground">Image:</span>
                            <Badge
                                variant="outline"
                                class="font-mono text-xs max-w-[120px] truncate"
                            >
                                {selectedImage || "None selected"}
                            </Badge>
                        </div>
                        <div class="flex justify-between items-center">
                            <span class="text-muted-foreground"
                                >Replicants:</span
                            >
                            <span class="font-mono">{numberOfReplicants}</span>
                        </div>
                        {#if getStartDateTime()}
                            <div class="flex justify-between items-center">
                                <span class="text-muted-foreground">Start:</span
                                >
                                <span class="font-mono text-xs"
                                    >{getStartDateTime()
                                        ?.toISOString()
                                        ?.slice(0, 19)
                                        .replace("T", " ") || "Invalid"} UTC</span
                                >
                            </div>
                        {/if}
                        {#if getEndDateTime()}
                            <div class="flex justify-between items-center">
                                <span class="text-muted-foreground">End:</span>
                                <span class="font-mono text-xs"
                                    >{getEndDateTime()
                                        ?.toISOString()
                                        ?.slice(0, 19)
                                        .replace("T", " ") || "Invalid"} UTC</span
                                >
                            </div>
                        {/if}
                        {#if getDuration()}
                            <div class="flex justify-between items-center">
                                <span class="text-muted-foreground"
                                    >Duration:</span
                                >
                                <span class="font-mono"
                                    >{formatDuration(getDuration())}</span
                                >
                            </div>
                        {/if}

                        {#if validationResult && !validationResult.success}
                            <div
                                class="p-2 rounded-md bg-red-50 border border-red-200 dark:bg-red-900/20 dark:border-red-800 mb-3"
                            >
                                <div
                                    class="flex items-start gap-2 text-xs text-red-800 dark:text-red-300"
                                >
                                    <AlertTriangleIcon
                                        class="h-3 w-3 mt-0.5 flex-shrink-0"
                                    />
                                    <span>{validationResult.message}</span>
                                </div>
                            </div>
                        {/if}
                        <div class="pt-2">
                            <Button
                                variant="default"
                                onclick={handleSchedule}
                                disabled={!isFormValid ||
                                    loading ||
                                    !validationResult?.success}
                                class="w-full"
                            >
                                {#if loading}
                                    <RefreshIcon class="h-4 w-4 animate-spin" />
                                    Scheduling...
                                {:else}
                                    <PlayIcon class="h-4 w-4" />
                                    Schedule Observation
                                {/if}
                            </Button>
                        </div>
                    </Card.Content>
                </Card.Root>
            </div>
        </div>
    </div>
</div>
