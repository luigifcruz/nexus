<script lang="ts">
    import SettingsIcon from "@tabler/icons-svelte/icons/settings";
</script>

<svelte:head>
    <title>Settings - Nexus</title>
</svelte:head>

<div class="flex flex-col gap-6 py-6">
    <!-- Header -->
    <div class="px-4 lg:px-6">
        <div class="flex items-center justify-between">
            <div>
                <h1 class="text-3xl font-bold tracking-tight">Settings</h1>
            </div>
        </div>
    </div>

    <!-- Content -->
    <div class="flex-1 flex flex-col items-center justify-center px-4 lg:px-6 min-h-[75vh]">
        <div class="text-center space-y-8">
            <div class="space-y-4">
                <SettingsIcon class="h-16 w-16 mx-auto text-muted-foreground/50" />
                <div class="space-y-2">
                    <h2 class="text-2xl font-semibold">Coming Soon</h2>
                    <p class="text-muted-foreground max-w-md">
                        Configuration options will be available in a future release.
                    </p>
                </div>
            </div>


        </div>

        <!-- ASCII Art -->
        <div class="fixed bottom-4 right-4 text-xs font-mono text-muted-foreground/30 leading-tight pointer-events-none select-none">
            <pre>                           .-.
          .-""`""-.      |(@ @)
       _/`oOoOoOoOo`\_   \ \-/
      '.-=-=-=-=-=-=-.'   \/ \
        `-=.=-.-=.=-'      \ /\
           ^  ^  ^         _H_ \ art by jgs</pre>
        </div>
    </div>
</div>
