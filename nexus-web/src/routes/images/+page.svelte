<script lang="ts">
    import { goto } from "$app/navigation";
    import { dataStore } from "$lib/stores/data.svelte";
    import { apiSync } from "$lib/services/sync";
    import type { ImageData, ConnectionStatus } from "$lib/types";

    import ContainerIcon from "@tabler/icons-svelte/icons/container";
    import SearchIcon from "@tabler/icons-svelte/icons/search";
    import PlusIcon from "@tabler/icons-svelte/icons/plus";
    import TrashIcon from "@tabler/icons-svelte/icons/trash";
    import CopyIcon from "@tabler/icons-svelte/icons/copy";
    import XIcon from "@tabler/icons-svelte/icons/x";
    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import CodeIcon from "@tabler/icons-svelte/icons/code";
    import DatabaseIcon from "@tabler/icons-svelte/icons/database";
    import SettingsIcon from "@tabler/icons-svelte/icons/settings";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";

    import * as Card from "$lib/components/ui/card/index.js";
    import * as Sheet from "$lib/components/ui/sheet/index.js";
    import * as Table from "$lib/components/ui/table/index.js";

    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Separator } from "$lib/components/ui/separator/index.js";

    // Connection state from data store
    let connectionStatus = $derived(dataStore.connectionStatus);
    let images = $derived(dataStore.images || []);

    // UI state
    let searchTerm = $state("");
    let createSheetOpen = $state(false);
    let deleteDialogOpen = $state(false);
    let selectedImageId = $state("");
    let isSubmitting = $state(false);
    let isEditMode = $state(false);
    let editingImageId = $state("");
    let showDeleteConfirm = $state(false);
    let deleteConfirmInput = $state("");


    // Form state
    let formData = $state({
        imageId: "",
        dockerImage: "",
        dockerEntrypoint: "",
        dockerArgs: [] as string[],
        dockerEnv: [] as string[],
    });

    let newArg = $state("");
    let newEnv = $state("");

    // Filtered images
    let filteredImages = $derived(
        images.filter((image) =>
            image.imageId.toLowerCase().includes(searchTerm.toLowerCase()) ||
            image.dockerImage.toLowerCase().includes(searchTerm.toLowerCase())
        )
    );

    // Reset form data
    function resetForm() {
        formData = {
            imageId: "",
            dockerImage: "",
            dockerEntrypoint: "",
            dockerArgs: [] as string[],
            dockerEnv: [] as string[],
        };
        newArg = "";
        newEnv = "";
        isSubmitting = false;
        isEditMode = false;
        editingImageId = "";
        showDeleteConfirm = false;
        deleteConfirmInput = "";

    }

    // Docker args management
    function addArg() {
        if (newArg.trim()) {
            formData.dockerArgs = [...formData.dockerArgs, newArg.trim()];
            newArg = "";
        }
    }

    function removeArg(index: number) {
        formData.dockerArgs = formData.dockerArgs.filter((_, i) => i !== index);
    }

    // Docker env management
    function addEnv() {
        if (newEnv.trim()) {
            formData.dockerEnv = [...formData.dockerEnv, newEnv.trim()];
            newEnv = "";
        }
    }

    function removeEnv(index: number) {
        formData.dockerEnv = formData.dockerEnv.filter((_, i) => i !== index);
    }

    // Form submission
    async function handleSubmit() {
        if (isSubmitting) return;

        isSubmitting = true;
        try {
            if (isEditMode) {
                // Delete old image and create new one
                await apiSync.destroyImage(editingImageId);
                await apiSync.createImage(formData);
            } else {
                await apiSync.createImage(formData);
            }
            resetForm();
            createSheetOpen = false;
        } catch (error) {
            console.error("Failed to create image:", error);
        } finally {
            isSubmitting = false;
        }
    }

    // Delete image
    async function handleDelete() {
        if (isSubmitting || !selectedImageId) return;

        isSubmitting = true;
        try {
            await apiSync.destroyImage(selectedImageId);
            deleteDialogOpen = false;
            selectedImageId = "";
        } catch (error) {
            console.error("Failed to delete image:", error);
        } finally {
            isSubmitting = false;
        }
    }

    // Copy to clipboard
    async function copyToClipboard(text: string) {
        try {
            await navigator.clipboard.writeText(text);
        } catch (error) {
            console.error("Failed to copy to clipboard:", error);
        }
    }

    // Navigation and actions
    function handleDeleteClick(imageId: string) {
        selectedImageId = imageId;
        deleteDialogOpen = true;
    }

    function navigateToImage(imageId: string) {
        // Load image data into form for viewing/editing
        const image = images.find(img => img.imageId === imageId);
        if (image) {
            formData = {
                imageId: image.imageId,
                dockerImage: image.dockerImage,
                dockerEntrypoint: image.dockerEntrypoint || "",
                dockerArgs: [...image.dockerArgs],
                dockerEnv: [...image.dockerEnv],
            };
            isEditMode = true;
            editingImageId = imageId;
            createSheetOpen = true;
        }
    }

    function handleCreateClick() {
        resetForm();
        createSheetOpen = true;
    }
</script>

<!-- Page Header -->
<div class="flex flex-col gap-6 py-6">
    <div class="px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">Images</h1>
                <p class="text-muted-foreground">
                    Manage container images for observations
                </p>
            </div>
            <Button onclick={handleCreateClick}>
                <PlusIcon class="h-4 w-4" />
                Create Image
            </Button>
        </div>

        <!-- Search -->
        <div class="mt-6 relative max-w-sm">
            <SearchIcon class="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
            <Input
                bind:value={searchTerm}
                placeholder="Search images..."
                class="pl-9"
            />
        </div>
    </div>

    <!-- Content -->
    <div class="px-4 lg:px-6">
        {#if filteredImages.length === 0}
            <!-- Empty State -->
            <Card.Root>
                <Card.Header class="text-center py-16">
                    <ContainerIcon class="h-16 w-16 mx-auto text-muted-foreground mb-4" />
                    <Card.Title class="text-xl">
                        {#if searchTerm}
                            No images match your search
                        {:else}
                            No container images found
                        {/if}
                    </Card.Title>
                    <Card.Description class="text-base">
                        {#if searchTerm}
                            Try adjusting your search terms or clear the search to see all images.
                        {:else}
                            Get started by creating your first container image configuration.
                        {/if}
                    </Card.Description>
                    {#if !searchTerm}
                        <div class="mt-6">
                            <Button onclick={handleCreateClick}>
                                <PlusIcon class="h-4 w-4" />
                                Create First Image
                            </Button>
                        </div>
                    {/if}
                </Card.Header>
            </Card.Root>
        {:else}
            <!-- Images Table -->
            <div class="rounded-lg border bg-background">
                <Table.Root>
                    <Table.Header>
                        <Table.Row class="hover:bg-transparent">
                            <Table.Head class="font-semibold">Image ID</Table.Head>
                            <Table.Head class="font-semibold">Docker Image</Table.Head>
                            <Table.Head class="font-semibold">Entrypoint</Table.Head>
                            <Table.Head class="w-16"></Table.Head>
                        </Table.Row>
                    </Table.Header>
                    <Table.Body>
                        {#each filteredImages as image (image.imageId)}
                            <Table.Row class="hover:bg-muted/50">
                                <Table.Cell class="font-mono font-medium py-2 cursor-pointer" onclick={() => navigateToImage(image.imageId)}>{image.imageId}</Table.Cell>
                                <Table.Cell class="font-mono text-sm text-muted-foreground max-w-96 truncate py-2 cursor-pointer" onclick={() => navigateToImage(image.imageId)}>
                                    {image.dockerImage}
                                </Table.Cell>
                                <Table.Cell class="font-mono text-sm py-2 cursor-pointer" onclick={() => navigateToImage(image.imageId)}>
                                    {#if image.dockerEntrypoint}
                                        {image.dockerEntrypoint}
                                    {:else}
                                        <span class="text-muted-foreground">—</span>
                                    {/if}
                                </Table.Cell>
                                <Table.Cell class="py-2">
                                    <Button
                                        variant="ghost"
                                        size="sm"
                                        onclick={() => navigateToImage(image.imageId)}
                                        class="h-6 px-3 text-xs"
                                    >
                                        Edit
                                    </Button>
                                </Table.Cell>
                            </Table.Row>
                        {/each}
                    </Table.Body>
                </Table.Root>
            </div>
        {/if}
    </div>
</div>

<!-- Create Image Sheet -->
<Sheet.Root bind:open={createSheetOpen}>
    <Sheet.Content class="w-full sm:max-w-2xl overflow-hidden">
        <form on:submit|preventDefault={handleSubmit} class="flex flex-col h-full">
            <div class="flex-1 overflow-y-auto px-6 pt-6 pb-32 space-y-8">
                <!-- Header Section -->
                <div class="space-y-4">
                    <div>
                        <h1 class="text-2xl font-bold">{isEditMode ? 'Edit' : 'Create'} Image</h1>
                        <p class="text-base text-muted-foreground">
                            {isEditMode
                                ? 'Modify the configuration and recreate the image with new settings.'
                                : 'Configure a new image for observations.'
                            }
                        </p>
                    </div>
                </div>
                <!-- Basic Configuration Card -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 pb-2">
                        <div class="p-2 rounded bg-primary/10 flex items-center justify-center">
                            <SettingsIcon class="h-5 w-5 text-primary" />
                        </div>
                        <h3 class="font-semibold text-base">Basic Configuration</h3>
                    </div>

                    <div class="grid gap-6 sm:grid-cols-2">
                        <div class="space-y-2 sm:col-span-2">
                            <Label for="imageId" class="text-sm font-medium flex items-center gap-1">
                                Image ID <span class="text-destructive">*</span>
                            </Label>
                            <Input
                                id="imageId"
                                bind:value={formData.imageId}
                                placeholder="my-astronomy-pipeline"
                                required
                                class="font-mono"
                                autocomplete="off"
                                disabled={isEditMode}
                            />
                            <p class="text-xs text-muted-foreground leading-relaxed">
                                {isEditMode
                                    ? 'Image ID cannot be changed.'
                                    : 'A unique identifier for this image. Use lowercase letters, numbers, and hyphens only.'
                                }
                            </p>
                        </div>

                        <div class="space-y-2 sm:col-span-2">
                            <Label for="dockerImage" class="text-sm font-medium flex items-center gap-1">
                                Docker Image <span class="text-destructive">*</span>
                            </Label>
                            <Input
                                id="dockerImage"
                                bind:value={formData.dockerImage}
                                placeholder="ghcr.io/owner/repo:latest"
                                required
                                class="font-mono"
                                autocomplete="off"
                            />
                            <p class="text-xs text-muted-foreground leading-relaxed">
                                Full Docker image reference including registry, repository name, and tag.
                            </p>
                        </div>

                        <div class="space-y-2 sm:col-span-2">
                            <Label for="dockerEntrypoint" class="text-sm font-medium">
                                Custom Entrypoint
                            </Label>
                            <Input
                                id="dockerEntrypoint"
                                bind:value={formData.dockerEntrypoint}
                                placeholder="/usr/local/bin/pipeline-runner"
                                class="font-mono"
                                autocomplete="off"
                            />
                            <p class="text-xs text-muted-foreground leading-relaxed">
                                Override the default container entrypoint. Leave empty to use the image's default.
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Docker Arguments Card -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 pb-2">
                        <div class="p-2 rounded bg-blue-100 dark:bg-blue-900/30 flex items-center justify-center">
                            <CodeIcon class="h-5 w-5 text-blue-600 dark:text-blue-400" />
                        </div>
                        <div class="flex-1">
                            <h3 class="font-semibold text-base">Command Arguments</h3>
                            <p class="text-xs text-muted-foreground">Arguments to pass to the container command</p>
                        </div>
                        <Badge variant="secondary" class="text-xs">
                            {formData.dockerArgs.length} arg{formData.dockerArgs.length !== 1 ? 's' : ''}
                        </Badge>
                    </div>

                    {#if formData.dockerArgs.length > 0}
                        <div class="space-y-2 max-h-32 overflow-y-auto border rounded-lg p-3 bg-muted/20">
                            {#each formData.dockerArgs as arg, index}
                                <div class="flex items-center gap-3 p-2 bg-background rounded border group hover:shadow-sm transition-shadow">
                                    <div class="flex-1 min-w-0">
                                        <code class="font-mono text-sm block truncate">{arg}</code>
                                    </div>
                                    <Button
                                        type="button"
                                        variant="ghost"
                                        size="sm"
                                        onclick={() => removeArg(index)}
                                        class="h-7 w-7 p-0 opacity-60 hover:opacity-100 hover:bg-destructive/10 hover:text-destructive transition-all"
                                    >
                                        <XIcon class="h-3 w-3" />
                                    </Button>
                                </div>
                            {/each}
                        </div>
                    {/if}

                    <div class="flex gap-2">
                        <div class="flex-1">
                            <Input
                                bind:value={newArg}
                                placeholder="--config /app/config.yaml"
                                class="font-mono"
                                autocomplete="off"
                                onkeydown={(e) => {
                                    if (e.key === 'Enter') {
                                        e.preventDefault();
                                        if (newArg.trim()) addArg();
                                    }
                                }}
                            />
                        </div>
                        <Button
                            type="button"
                            variant="outline"
                            onclick={addArg}
                            disabled={!newArg.trim()}
                            class="px-4"
                        >
                            <PlusIcon class="h-3 w-3 mr-1" />
                            Add
                        </Button>
                    </div>
                </div>

                <!-- Environment Variables Card -->
                <div class="space-y-4">
                    <div class="flex items-center gap-2 pb-2">
                        <div class="p-2 rounded bg-green-100 dark:bg-green-900/30 flex items-center justify-center">
                            <DatabaseIcon class="h-5 w-5 text-green-600 dark:text-green-400" />
                        </div>
                        <div class="flex-1">
                            <h3 class="font-semibold text-base">Environment Variables</h3>
                            <p class="text-xs text-muted-foreground">Environment variables for the container runtime</p>
                        </div>
                        <Badge variant="secondary" class="text-xs">
                            {formData.dockerEnv.length} var{formData.dockerEnv.length !== 1 ? 's' : ''}
                        </Badge>
                    </div>

                    {#if formData.dockerEnv.length > 0}
                        <div class="space-y-2 max-h-32 overflow-y-auto border rounded-lg p-3 bg-muted/20">
                            {#each formData.dockerEnv as env, index}
                                <div class="flex items-center gap-3 p-2 bg-background rounded border group hover:shadow-sm transition-shadow">
                                    <div class="flex-1 min-w-0">
                                        <code class="font-mono text-sm block truncate">{env}</code>
                                    </div>
                                    <Button
                                        type="button"
                                        variant="ghost"
                                        size="sm"
                                        onclick={() => removeEnv(index)}
                                        class="h-7 w-7 p-0 opacity-60 hover:opacity-100 hover:bg-destructive/10 hover:text-destructive transition-all"
                                    >
                                        <XIcon class="h-3 w-3" />
                                    </Button>
                                </div>
                            {/each}
                        </div>
                    {/if}

                    <div class="flex gap-2">
                        <div class="flex-1">
                            <Input
                                bind:value={newEnv}
                                placeholder="LOG_LEVEL=debug"
                                class="font-mono"
                                autocomplete="off"
                                onkeydown={(e) => {
                                    if (e.key === 'Enter') {
                                        e.preventDefault();
                                        if (newEnv.trim()) addEnv();
                                    }
                                }}
                            />
                        </div>
                        <Button
                            type="button"
                            variant="outline"
                            onclick={addEnv}
                            disabled={!newEnv.trim()}
                            class="px-4"
                        >
                            <PlusIcon class="h-3 w-3 mr-1" />
                            Add
                        </Button>
                    </div>
                </div>

                <!-- Danger Zone -->
                {#if isEditMode}
                    <div class="p-4 bg-destructive/5 border border-destructive/20 rounded-lg">
                        <div class="flex items-center gap-2 pb-3">
                            <div class="p-2 rounded bg-destructive/20 flex items-center justify-center">
                                <AlertTriangleIcon class="h-5 w-5 text-destructive" />
                            </div>
                            <h3 class="font-semibold text-base text-destructive">Danger Zone</h3>
                        </div>

                        <p class="text-sm text-muted-foreground mb-4">
                            Delete this image configuration permanently. This action cannot be undone.
                        </p>

                        {#if !showDeleteConfirm}
                            <Button
                                type="button"
                                variant="destructive"
                                size="sm"
                                onclick={() => { showDeleteConfirm = true; }}
                                disabled={isSubmitting}
                            >
                                <TrashIcon class="h-4 w-4 mr-2" />
                                Delete Image
                            </Button>
                        {:else}
                            <div class="space-y-3 p-3 bg-destructive/10 rounded border border-destructive/30">
                                <p class="text-sm font-medium text-destructive">
                                    Type <code class="bg-muted px-1 py-0.5 rounded text-xs">{editingImageId}</code> to confirm deletion:
                                </p>
                                <Input
                                    bind:value={deleteConfirmInput}
                                    placeholder={editingImageId}
                                    class="font-mono"
                                    autocomplete="off"
                                />
                                <div class="flex gap-2">
                                    <Button
                                        type="button"
                                        variant="outline"
                                        size="sm"
                                        onclick={() => {
                                            showDeleteConfirm = false;
                                            deleteConfirmInput = "";
                                        }}
                                        disabled={isSubmitting}
                                    >
                                        Cancel
                                    </Button>
                                    <Button
                                        type="button"
                                        variant="destructive"
                                        size="sm"
                                        onclick={async () => {
                                            selectedImageId = editingImageId;
                                            await handleDelete();
                                            createSheetOpen = false;
                                            resetForm();
                                        }}
                                        disabled={isSubmitting || deleteConfirmInput !== editingImageId}
                                    >
                                        {#if isSubmitting}
                                            Deleting...
                                        {:else}
                                            Delete Image
                                        {/if}
                                    </Button>
                                </div>
                            </div>
                        {/if}
                    </div>
                {/if}

            </div>

            <!-- Fixed Footer -->
            <div class="absolute bottom-0 left-0 right-0 border-t bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 p-6">
                <div class="flex justify-end gap-3">
                    <Button
                        type="button"
                        variant="outline"
                        onclick={() => {
                            createSheetOpen = false;
                            resetForm();
                        }}
                        disabled={isSubmitting}
                        class="min-w-20"
                    >
                        Cancel
                    </Button>
                    <Button
                        type="submit"
                        disabled={isSubmitting || !formData.imageId?.trim() || !formData.dockerImage?.trim()}
                        class="min-w-32"
                    >
                        {#if isSubmitting}
                            <RefreshIcon class="h-4 w-4 mr-2 animate-spin" />
                            {isEditMode ? 'Updating...' : 'Creating...'}
                        {:else}
                            <ContainerIcon class="h-4 w-4 mr-2" />
                            {isEditMode ? 'Update Image' : 'Create Image'}
                        {/if}
                    </Button>
                </div>
            </div>
        </form>
    </Sheet.Content>
</Sheet.Root>
