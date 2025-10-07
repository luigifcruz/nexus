<script lang="ts">
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import { Badge } from "$lib/components/ui/badge/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu/index.js";
    import CameraVideo from "$lib/components/camera-video.svelte";

    import SunIcon from "@tabler/icons-svelte/icons/sun";
    import CloudIcon from "@tabler/icons-svelte/icons/cloud";
    import SnowflakeIcon from "@tabler/icons-svelte/icons/snowflake";
    import DropletIcon from "@tabler/icons-svelte/icons/droplet";
    import WindIcon from "@tabler/icons-svelte/icons/wind";
    import ThermometerIcon from "@tabler/icons-svelte/icons/thermometer";
    import EyeIcon from "@tabler/icons-svelte/icons/eye";
    import RefreshIcon from "@tabler/icons-svelte/icons/refresh";
    import AlertTriangleIcon from "@tabler/icons-svelte/icons/alert-triangle";
    import ChevronDownIcon from "@tabler/icons-svelte/icons/chevron-down";
    import XIcon from "@tabler/icons-svelte/icons/x";
    import { pipStore } from "$lib/stores/pip.svelte";

    let mounted = $state(false);
    let refreshing = $state(false);
    let maximizedCamera = $state(null);

    // Available cameras
    let cameras = $state([
        { id: "main", name: "Main Site View", url: "http://10.3.0.30/mjpg/video.mjpg", selected: true },
        { id: "lab1", name: "Lab 1 View", url: "http://10.3.0.28/mjpg/video.mjpg", selected: false },
        { id: "lab2", name: "Lab 2 View", url: "http://10.3.0.29/mjpg/video.mjpg", selected: false },
    ]);

    let selectedCameras = $derived(cameras.filter(camera => camera.selected));

    // Local weather data state
    let weatherData = $state({
        temperature: 72.3,
        humidity: 45,
        windSpeed: 8.2,
        windDirection: "NW",
        pressure: 30.15,
        condition: "Clear",
        lastUpdated: new Date().toISOString(),
        observingConditions: "Good",
        advisory: "Winter Storm Watch in effect from Thursday evening through Friday morning. Heavy snow possible with accumulations of 6-12 inches.",
    });

    let weatherForecast = $state([
        { day: "Today", condition: "Clear", high: 75, low: 52, precipitation: 0, icon: "clear" },
        { day: "Tomorrow", condition: "Partly Cloudy", high: 73, low: 48, precipitation: 10, icon: "cloudy" },
        { day: "Wednesday", condition: "Rain", high: 68, low: 45, precipitation: 85, icon: "rain" },
        { day: "Thursday", condition: "Heavy Rain", high: 65, low: 42, precipitation: 95, icon: "rain" },
        { day: "Friday", condition: "Clear", high: 71, low: 47, precipitation: 5, icon: "clear" },
    ]);

    onMount(() => {
        mounted = true;
        // Here you could fetch real weather data
        fetchWeatherData();
    });

    async function fetchWeatherData() {
        // Mock function - replace with actual weather station API call
        // try {
        //     const response = await fetch('/api/weather');
        //     weatherData = await response.json();
        // } catch (error) {
        //     console.error('Failed to fetch weather data:', error);
        // }
    }

    function refreshWeather() {
        refreshing = true;
        setTimeout(() => {
            fetchWeatherData();
            refreshing = false;
        }, 1000);
    }

    function toggleCamera(cameraId: string) {
        cameras = cameras.map(camera =>
            camera.id === cameraId
                ? { ...camera, selected: !camera.selected }
                : camera
        );
    }

    function maximizeCamera(camera: any) {
        maximizedCamera = camera;
        resetVideoState(); // Reset state when maximizing camera
    }

    function closeMaximized() {
        maximizedCamera = null;
    }

    function formatTimestamp(timestamp: string): string {
        return new Date(timestamp).toLocaleTimeString("en-US", {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
        });
    }

    function getConditionIcon(condition: string) {
        switch (condition.toLowerCase()) {
            case "clear":
            case "sunny":
                return SunIcon;
            case "cloudy":
            case "overcast":
                return CloudIcon;
            default:
                return SunIcon;
        }
    }


</script>

{#if mounted}
    <div class="flex flex-col gap-6 py-6">
        <!-- Page Header -->
        <div class="px-4 lg:px-6">
            <div class="flex items-center justify-between">
                <div>
                    <h1 class="text-3xl font-bold tracking-tight">Site Overview</h1>
                    <p class="text-muted-foreground">
                        Live view of the Allen Telescope Array site and current weather
                        conditions
                    </p>
                </div>
            </div>
        </div>

        <!-- Main Content: Live View and Weather -->
        <div class="px-4 lg:px-6">
            <div class="grid gap-6 grid-cols-3">
                <!-- Live Site View - 2/3 width -->
                <div class="col-span-2">
                    <Card.Root>
                        <Card.Header>
                            <div class="flex items-center justify-between">
                                <Card.Title class="flex items-center gap-2">
                                    <EyeIcon class="h-5 w-5" />
                                    Live Site View
                                </Card.Title>
                                <DropdownMenu.Root>
                                    <DropdownMenu.Trigger>
                                        <Button variant="outline" size="sm">
                                            <EyeIcon class="h-4 w-4" />
                                            Cameras ({selectedCameras.length})
                                            <ChevronDownIcon class="h-4 w-4" />
                                        </Button>
                                    </DropdownMenu.Trigger>
                                    <DropdownMenu.Content>
                                        <DropdownMenu.Label>Select Cameras</DropdownMenu.Label>
                                        <DropdownMenu.Separator />
                                        {#each cameras as camera}
                                            <DropdownMenu.CheckboxItem
                                                checked={camera.selected}
                                                onCheckedChange={() => toggleCamera(camera.id)}
                                            >
                                                {camera.name}
                                            </DropdownMenu.CheckboxItem>
                                        {/each}
                                    </DropdownMenu.Content>
                                </DropdownMenu.Root>
                            </div>

                        </Card.Header>
                        <Card.Content>
                            {#if selectedCameras.length === 0}
                                <div class="relative aspect-video bg-muted rounded-lg overflow-hidden">
                                    <div class="absolute inset-0 flex flex-col items-center justify-center text-muted-foreground">
                                        <EyeIcon class="h-12 w-12 mb-4 opacity-50" />
                                        <p class="text-lg font-medium mb-2">No cameras selected</p>
                                        <p class="text-sm text-center">
                                            Please select one or more cameras from the dropdown above.
                                        </p>
                                    </div>
                                </div>
                            {:else}
                                <div class="grid gap-4 {selectedCameras.length === 1 ? '' : selectedCameras.length === 2 ? 'grid-cols-2' : selectedCameras.length <= 4 ? 'grid-cols-2' : 'grid-cols-3'}">
                                    {#each selectedCameras as camera (camera.id)}
                                        <div class="{selectedCameras.length === 1 ? 'aspect-video' : 'aspect-[4/3]'} bg-muted rounded-lg overflow-hidden">
                                            <CameraVideo
                                                {camera}
                                                onMaximize={maximizeCamera}
                                                onPiP={(cam) => pipStore.toggle(cam)}
                                                variant="small"
                                            />
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </Card.Content>
                    </Card.Root>
                </div>

                <!-- Weather Station & Forecast - 1/3 width -->
                <div class="col-span-1">
                    <Card.Root class="h-full">
                        <Card.Header class="pb-3">
                            {@const ConditionIcon = getConditionIcon(weatherData.condition)}
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    <ConditionIcon class="h-5 w-5" />
                                    <Card.Title>Weather & Forecast</Card.Title>
                                </div>
                                <Button
                                    variant="ghost"
                                    size="sm"
                                    onclick={refreshWeather}
                                    disabled={refreshing}
                                    class="size-8 p-0"
                                >
                                    <RefreshIcon class="h-4 w-4 {refreshing ? 'animate-spin' : ''}" />
                                </Button>
                            </div>
                        </Card.Header>
                        <Card.Content class="space-y-4">
                            <!-- Temperature Display -->
                            <div class="text-center py-1">
                                <div class="text-4xl font-bold font-mono">{weatherData.temperature}°F</div>
                                <div class="text-sm text-muted-foreground mt-1">{weatherData.condition}</div>
                                <Badge
                                    variant={weatherData.observingConditions === "Excellent" ? "default" :
                                            weatherData.observingConditions === "Good" ? "secondary" : "destructive"}
                                    class="mt-2"
                                >
                                    {weatherData.observingConditions}
                                </Badge>
                            </div>

                            <!-- Weather Advisory -->
                            {#if weatherData.advisory}
                                <div class="space-y-2">
                                    <div class="flex items-center gap-2 p-3 rounded bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800">
                                        <AlertTriangleIcon class="h-4 w-4 text-yellow-600 dark:text-yellow-500 shrink-0" />
                                        <div class="text-sm text-yellow-800 dark:text-yellow-200">
                                            <div class="font-medium mb-1">Weather Advisory</div>
                                            <div class="text-xs leading-relaxed">{weatherData.advisory}</div>
                                        </div>
                                    </div>
                                </div>
                            {/if}

                            <!-- Weather Details Grid -->
                            <div class="grid grid-cols-2 gap-2 text-sm">
                                <div class="flex items-center gap-2 p-2 rounded bg-muted/30">
                                    <DropletIcon class="h-4 w-4 text-blue-500" />
                                    <div>
                                        <div class="font-medium">{weatherData.humidity}%</div>
                                        <div class="text-xs text-muted-foreground">Humidity</div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2 p-2 rounded bg-muted/30">
                                    <WindIcon class="h-4 w-4 text-green-500" />
                                    <div>
                                        <div class="font-medium">{weatherData.windSpeed} mph</div>
                                        <div class="text-xs text-muted-foreground">Wind</div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2 p-2 rounded bg-muted/30">
                                    <ThermometerIcon class="h-4 w-4 text-red-500" />
                                    <div>
                                        <div class="font-medium">{weatherData.pressure}</div>
                                        <div class="text-xs text-muted-foreground">Pressure</div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2 p-2 rounded bg-muted/30">
                                    <EyeIcon class="h-4 w-4 text-purple-500" />
                                    <div>
                                        <div class="font-medium">Clear</div>
                                        <div class="text-xs text-muted-foreground">Visibility</div>
                                    </div>
                                </div>
                            </div>

                            <!-- 5-Day Forecast -->
                            <div class="space-y-2">
                                <h4 class="text-sm font-medium text-muted-foreground">5-Day Forecast</h4>
                                <div class="space-y-2">
                                    {#each weatherForecast.slice(0, 5) as day}
                                        <div class="flex items-center gap-2 p-2 rounded bg-muted/30 {day.condition === 'Heavy Rain' || day.condition === 'Snow' ? 'ring-2 ring-red-400 dark:ring-red-500' : ''}">
                                            <div class="flex items-center gap-2 flex-1">
                                                {#if day.icon === "clear"}
                                                    <SunIcon class="h-4 w-4 text-yellow-500" />
                                                {:else if day.icon === "cloudy"}
                                                    <CloudIcon class="h-4 w-4 text-gray-500 dark:text-gray-400" />
                                                {:else if day.icon === "rain"}
                                                    <DropletIcon class="h-4 w-4 text-blue-500" />
                                                {:else if day.icon === "snow"}
                                                    <SnowflakeIcon class="h-4 w-4 text-blue-300 dark:text-blue-400" />
                                                {/if}
                                                <span class="text-sm font-medium">{day.day}</span>
                                            </div>
                                            <div class="flex items-center gap-2">
                                                <span class="text-xs text-muted-foreground">{day.precipitation}%</span>
                                                <span class="text-sm font-medium text-right">{day.high}°/{day.low}°</span>
                                            </div>
                                        </div>
                                    {/each}
                                </div>
                            </div>
                        </Card.Content>
                    </Card.Root>
                </div>
            </div>
        </div>

    </div>

    <!-- Maximized Camera Modal -->
    {#if maximizedCamera}
        <div class="fixed inset-0 z-50 bg-black/80 flex items-center justify-center p-4" onclick={closeMaximized}>
            <div class="relative w-full h-full max-w-6xl max-h-[90vh] bg-background rounded-lg overflow-hidden" onclick={(e) => e.stopPropagation()}>
                <CameraVideo
                    camera={maximizedCamera}
                    onPiP={(cam) => pipStore.toggle(cam)}
                    onClose={closeMaximized}
                    variant="maximized"
                />
            </div>
        </div>
    {/if}


{:else}
    <!-- Loading State -->
    <div class="flex flex-col gap-6 py-6 px-4 lg:px-6 opacity-50">
        <div class="h-20 bg-muted/20 rounded animate-pulse"></div>
        <div class="grid gap-6">
            <div>
                <div
                    class="aspect-video bg-muted/20 rounded animate-pulse"
                ></div>
            </div>
            <div class="grid gap-6 @xl/main:grid-cols-2">
                <div class="h-80 bg-muted/20 rounded animate-pulse"></div>
                <div class="h-80 bg-muted/20 rounded animate-pulse"></div>
            </div>
            <div class="h-96 bg-muted/20 rounded animate-pulse"></div>
        </div>
    </div>
{/if}
