<script lang="ts">
	import { onMount, type ComponentType } from 'svelte';
	import { RiSearchLine } from 'svelte-remixicon';

	export let value = '';
</script>

<div class="flex bg-night-550 rounded-full p-2 items-center w-full space-x-2">
	<RiSearchLine size="24px" class="stroke-2" />
	<input type="search" class="bg-transparent text-lg w-full font-bold placeholder:font-normal" placeholder="Search..." bind:value />
</div>
