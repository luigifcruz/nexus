<script lang="ts">
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import MaximizeIcon from "@tabler/icons-svelte/icons/maximize";
    import PictureInPictureIcon from "@tabler/icons-svelte/icons/picture-in-picture";
    import XIcon from "@tabler/icons-svelte/icons/x";
    import CircleDottedIcon from "@tabler/icons-svelte/icons/circle-dotted";

    interface Camera {
        id: string;
        name: string;
        url: string;
    }

    interface Props {
        camera: Camera;
        showControls?: boolean;
        showName?: boolean;
        onMaximize?: (camera: Camera) => void;
        onPiP?: (camera: Camera) => void;
        onClose?: () => void;
        variant?: 'small' | 'maximized' | 'pip';
    }

    let {
        camera,
        showControls = true,
        showName = true,
        onMaximize,
        onPiP,
        onClose,
        variant = 'small'
    }: Props = $props();

    let videoError = $state(false);
    let videoLoaded = $state(false);
    let videoLoading = $state(true);

    function handleVideoError() {
        videoError = true;
        videoLoaded = false;
        videoLoading = false;
    }

    function handleVideoLoad() {
        videoError = false;
        videoLoaded = true;
        videoLoading = false;
    }

    function resetVideoState() {
        videoError = false;
        videoLoaded = false;
        videoLoading = true;
    }

    // Reset video state when camera changes
    $effect(() => {
        if (camera) {
            resetVideoState();
        }
    });

    // Get classes based on variant
    function getBadgeSize() {
        switch (variant) {
            case 'maximized':
                return 'text-sm h-8';
            case 'pip':
                return 'text-xs h-6';
            default:
                return 'text-xs h-7';
        }
    }

    function getButtonSize() {
        switch (variant) {
            case 'maximized':
                return 'h-8 w-8';
            case 'pip':
                return 'h-6 w-6';
            default:
                return 'h-7 w-7';
        }
    }

    function getIconSize() {
        return 'h-3 w-3';
    }
</script>

<div class="relative w-full h-full">
    <!-- Camera Name Badge -->
    {#if showName}
        <div class="absolute top-2 left-2 z-10">
            <Badge
                variant="secondary"
                class="{getBadgeSize()} flex items-center opacity-70"
                style="backdrop-filter: blur(4px);"
            >
                {camera.name}
            </Badge>
        </div>
    {/if}

    <!-- Status Badge for maximized/pip -->
    {#if showControls && (variant === 'maximized' || variant === 'pip')}
        <div class="absolute top-2 right-2 z-10 flex items-center gap-2">
            {#if videoLoaded && !videoError}
                <Badge
                    variant="destructive"
                    class="{getBadgeSize()} flex items-center gap-1 opacity-80"
                    style="backdrop-filter: blur(4px);"
                >
                    <div class="h-2 w-2 rounded-full bg-white animate-pulse"></div>
                    LIVE
                </Badge>
            {:else if videoLoading}
                <Badge
                    class="{getBadgeSize()} flex items-center gap-1 opacity-80 bg-blue-500 hover:bg-blue-600 text-white"
                    style="backdrop-filter: blur(4px);"
                >
                    <CircleDottedIcon class="{getIconSize()} text-white animate-pulse" />
                    LOADING
                </Badge>
            {/if}
            {#if onPiP && variant !== 'pip'}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={() => onPiP?.(camera)}
                >
                    <PictureInPictureIcon class="{getIconSize()}" />
                </Button>
            {/if}

            {#if onMaximize && variant !== 'maximized'}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={() => onMaximize?.(camera)}
                >
                    <MaximizeIcon class="{getIconSize()}" />
                </Button>
            {/if}

            {#if onClose}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={onClose}
                >
                    <XIcon class="{getIconSize()}" />
                </Button>
            {/if}
        </div>
    {/if}

    <!-- Control Buttons for small cameras -->
    {#if showControls && variant !== 'maximized' && variant !== 'pip'}
        <div class="absolute bottom-2 right-2 z-10 flex items-center gap-2">
            {#if onPiP && variant !== 'pip'}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={() => onPiP?.(camera)}
                >
                    <PictureInPictureIcon class="{getIconSize()}" />
                </Button>
            {/if}

            {#if onMaximize && variant !== 'maximized'}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={() => onMaximize?.(camera)}
                >
                    <MaximizeIcon class="{getIconSize()}" />
                </Button>
            {/if}

            {#if onClose}
                <Button
                    variant="secondary"
                    size="sm"
                    class="{getButtonSize()} p-0 opacity-70 hover:opacity-100"
                    style="backdrop-filter: blur(4px);"
                    onclick={(e) => {
                        e.stopPropagation();
                        onClose?.();
                    }}
                >
                    <XIcon class="{getIconSize()}" />
                </Button>
            {/if}
        </div>
    {/if}

    <!-- Status Badge for small cameras or when no controls -->
    {#if variant !== 'maximized' && variant !== 'pip'}
        <div class="absolute top-2 right-2 z-10">
            {#if videoLoaded && !videoError}
                <Badge
                    variant="destructive"
                    class="{getBadgeSize()} flex items-center gap-1 opacity-80"
                    style="backdrop-filter: blur(4px);"
                >
                    <div class="h-2 w-2 rounded-full bg-white animate-pulse"></div>
                    LIVE
                </Badge>
            {:else if videoLoading}
                <Badge
                    class="{getBadgeSize()} flex items-center gap-1 opacity-80 bg-blue-500 hover:bg-blue-600 text-white"
                    style="backdrop-filter: blur(4px);"
                >
                    <CircleDottedIcon class="{getIconSize()} text-white animate-pulse" />
                    LOADING
                </Badge>
            {/if}
        </div>
    {/if}

    <!-- Video Content -->
    {#if !videoError}
        <img
            src={camera.url}
            class="w-full h-full object-cover"
            onerror={handleVideoError}
            onload={handleVideoLoad}
        />
    {:else}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground">
            <EyeIcon class="{variant === 'pip' ? 'h-6 w-6' : 'h-8 w-8'} mb-2 opacity-50" />
            <p class="{variant === 'pip' ? 'text-xs' : 'text-sm'} font-medium mb-1">Stream unavailable</p>
            <p class="text-xs text-center">Check camera connection{variant !== 'pip' ? ' or try refreshing' : ''}</p>
        </div>
    {/if}
</div>
