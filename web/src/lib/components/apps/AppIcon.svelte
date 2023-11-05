<script lang="ts">
	import { onMount } from 'svelte';

	export let appName = '';
	export let iconSrc = '';
	export let onSelect: (id: number) => void;
	export let id: number;
	let icon: HTMLButtonElement;

	onMount(() => {
		icon.style.backgroundImage = `url(${iconSrc})`;
	});
</script>

<button
	class="rounded-full w-app-icon h-app-icon bg-black flex items-center justify-center bg-cover bg-no-repeat bg-center outline outline-none hover:outline-4 hover:outline-carnation-pink focus:outline-carnation-pink focus:outline-4 border-4 border-night transition-all active:scale-90 active:outline-0"
	bind:this={icon}
	on:click={() => onSelect(id)}
/>
