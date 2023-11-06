<script lang="ts">
	import { onMount, type ComponentType } from 'svelte';

	export let options: Array<TabOptions> = [];
	export let active = 0;
	let highlight: HTMLDivElement;

	const switchTo = (i: number) => {
		active = i;
		highlight.style.left = `${i * 256 + i * 8}px`;
	};

	onMount(() => {
		switchTo(active);
	})

	interface TabOptions {
		element: ComponentType;
		name: string;
	}
</script>

<div class="bg-night-550 rounded-full relative">
	<div
		bind:this={highlight}
		class="bg-uranian-blue bg-opacity-50 z-10 rounded-full absolute w-app-icon h-10 transition-all m-2 left-0"
	/>
	<div class="space-x-2 w-full flex m-2">
		{#each options as option, i}
			<button
				title={option.name}
				on:click={() => switchTo(i)}
				class="rounded-full p-2 w-app-icon flex justify-center z-20 bg-white bg-opacity-0 hocus:bg-opacity-25 outline-none focus-visible:outline-uranian-blue focus-visible:outline-2 transition-all active:scale-90"
			>
				<svelte:component this={option.element} size="24px" />
			</button>
		{/each}
	</div>
</div>
