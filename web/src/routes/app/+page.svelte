<script lang="ts">
	import * as caller from '$lib/caller/account';
	import { onMount } from 'svelte';

	let name: string | undefined;

	onMount(async () => {
		const res = await caller.me();
		if (!res?.error) {
			name = res?.email
		}
	});
</script>

<p>{name}</p>