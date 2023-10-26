<script lang="ts">
	import { goto } from '$app/navigation';
	import * as caller from '$lib/caller/account';
	import AppIcon from '$lib/components/apps/AppIcon.svelte';
	import { onMount } from 'svelte';
	import { fade, scale } from 'svelte/transition';

	let name: string | undefined;

	const apps = [
		{
			name: 'Netflix',
			icon: 'NetflixLogo.png'
		},
		{
			name: 'YouTube',
			icon: 'YoutubeLogo.png'
		},
		{
			name: 'Plex',
			icon: 'PlexLogo.png'
		},
		{
			name: 'Twitch',
			icon: 'TwitchLogo.png'
		}
	];

	onMount(async () => {
		const res = await caller.me();
		if (!res?.error) {
			name = res?.name;
		}
	});

	const onAppSelect = async (id: number) => {
		await goto(`/app/launch/${id}`);
	};
</script>

<div in:fade={{ delay: 500 }} class="absolute w-full h-full pt-24 flex flex-col items-center">
	<div class="text-center flex flex-col justify-center space-y-2" out:fade>
		<h1>Hi, {name}!</h1>
		<p class="text-xl">What are we watching today?</p>
	</div>
	<div
		class="py-24 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-8 items-center w-full"
	>
		{#each apps as app, i}
			<div
				class="w-full h-full flex items-center justify-center"
				in:scale|global={{ delay: 600 + 100 * i }}
				out:scale|global={{ delay: 100 * i }}
			>
				<AppIcon appName={app.name} iconSrc={`/assets/${app.icon}`} onSelect={onAppSelect} id={i} />
			</div>
		{/each}
	</div>
</div>
