<script lang="ts">
	import Loader from '../misc/Loader.svelte';

	export let onClick = () => {};
	export let loading = false;
	export let disabled = false;
	export let style: 'DEFAULT' | 'GLASS' = 'DEFAULT';
	if (loading) disabled = true;
	let styleClassAddon = '';
	if (style == 'GLASS') {
		styleClassAddon += ''
	}
	let classAddon = `${styleClassAddon} ` + '';
	export { classAddon as class };
</script>

<button
	class={`relative w-full flex justify-center items-center bg-transparent border-white text-carnation-pink font-bold border-opacity-25 border-2 rounded-full px-4 py-1 transition-all disabled:cursor-not-allowed disabled:opacity-75 enabled:hover:border-carnation-pink enabled:active:border-carnation-pink enabled:focus:border-carnation-pink enabled:active:scale-90 ${classAddon}`}
	on:click={onClick}
	{disabled}
>
	{#if !loading}
		<slot />
	{:else}
		<div class="flex items-center justify-center">
			<Loader />
		</div>
	{/if}
</button>
