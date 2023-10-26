<script lang="ts">
	import { browser } from '$app/environment';
	import { beforeNavigate, goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { me } from '$lib/caller/account';
	import { currentUser } from '$lib/context/context';
	import type { BeforeNavigate } from '@sveltejs/kit';

	let loading = true;

	const testForValidNavigation = async (navigation?: BeforeNavigate) => {
		if (
			$currentUser == undefined &&
			((navigation && navigation.to?.url.pathname != '/app/login') ||
				$page.url.pathname != '/app/login')
		) {
			navigation?.cancel();
			await goto('/app/login');
			return;
		}

		let fetchUser = await me();
		if (!fetchUser || fetchUser?.error) {
			navigation?.cancel();
			await goto('/app/login');
		} else {
			currentUser.set(fetchUser);
		}
	};

	beforeNavigate(testForValidNavigation);

	if (loading && browser) {
		testForValidNavigation();
		loading = false;
	}
</script>

<div class="w-full h-full min-h-screen bg-night relative flex flex-col items-center justify-center">
	{#if !loading}
		<slot />
	{/if}
</div>
