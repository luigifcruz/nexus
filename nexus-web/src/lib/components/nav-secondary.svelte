<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import type { WithoutChildren } from "$lib/utils.js";
	import type { ComponentProps } from "svelte";
	import type { ComponentType } from "svelte";

	let {
		items,
		currentPath,
		...restProps
	}: { items: { title: string; url: string; icon: ComponentType }[]; currentPath: string } & WithoutChildren<
		ComponentProps<typeof Sidebar.Group>
	> = $props();

	function isActive(url: string): boolean {
		if (url === "/") {
			return currentPath === "/";
		}
		return currentPath.startsWith(url);
	}
</script>

<Sidebar.Group {...restProps}>
	<Sidebar.GroupContent>
		<Sidebar.Menu>
			{#each items as item (item.title)}
				<Sidebar.MenuItem>
					<Sidebar.MenuButton isActive={isActive(item.url)}>
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
