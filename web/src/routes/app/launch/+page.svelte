<script lang="ts">
	import { goto } from '$app/navigation';
	import AppIcon from '$lib/components/apps/AppIcon.svelte';
	import { currentUser } from '$lib/context/context';
	import { fade, scale } from 'svelte/transition';

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
		},
	];

	const onAppSelect = async (id: number) => {
		await goto(`/app/launch/${id}`);
	};
</script>

<div in:fade={{ delay: 500 }} class="absolute w-full h-full pt-24 flex flex-col items-center">
	<div class="text-center flex flex-col justify-center space-y-2 mb-12" out:fade>
		<h1>Hi, {$currentUser?.name}!</h1>
		<p class="text-xl">What are we watching today?</p>
	</div>
	<div class="p-8 flex flex-wrap justify-center gap-8">
		{#each apps as app, i}
			<div
				class="flex items-center justify-center"
				in:scale|global={{ delay: 600 + 100 * i }}
				out:scale|global={{ delay: 100 * i }}
			>
				<AppIcon appName={app.name} iconSrc={`/assets/${app.icon}`} onSelect={onAppSelect} id={i} />
			</div>
		{/each}
	</div>
</div>
