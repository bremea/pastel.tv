<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import { accessToken, currentUser } from '$lib/context/context';
	import { getAccessToken, getMe } from '$lib/caller/account';
	import { get } from 'svelte/store';
	import { onMount } from 'svelte';

	let loading = true;

	onMount(async () => {
		if (!browser || !loading) return;
		if (!get(accessToken)) {
			try {
				let accessTokenRequest = await getAccessToken();
				if (accessTokenRequest.error) {
					throw new Error(accessTokenRequest.message);
				} else {
					accessToken.set(accessTokenRequest?.access_token);
				}
			} catch (_) {
				await goto('/app/login');
				loading = false;
				return;
			}
		}

		const me = await getMe();
		if (me.error) {
			await goto('/app/login');
			loading = false;
			return;
		}
		console.log(me);
		currentUser.set(me);
		loading = false;
	});
</script>

<div class="w-full h-full min-h-screen bg-night relative flex flex-col items-center justify-center">
	{#if loading}
		<p>Loading</p>
	{:else}
		<slot />
	{/if}
</div>
