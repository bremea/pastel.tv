<script lang="ts">
	import { goto } from '$app/navigation';
	import AppIcon from '$lib/components/apps/AppIcon.svelte';
	import Friend from '$lib/components/apps/Friend.svelte';
	import BigSearch from '$lib/components/input/BigSearch.svelte';
	import Button from '$lib/components/input/Button.svelte';
	import Tabs from '$lib/components/input/Tabs.svelte';
	import { currentUser } from '$lib/context/context';
	import { onMount } from 'svelte';
	import {
		RiApps2Line,
		RiArrowRightSLine,
		RiChat2Line,
		RiGroupLine,
		RiMessage3Line,
		RiTvLine
	} from 'svelte-remixicon';
	import { fade, fly, scale, slide } from 'svelte/transition';

	let currentTab = 1;
	let hasApps = false;

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

	onMount(() => {
		hasApps = true;
	});

	const onAppSelect = async (id: number) => {
		await goto(`/app/launch/${id}`);
	};
</script>

<div in:fade={{ delay: 500 }} class="absolute w-full h-full pt-24 flex flex-col items-center">
	<div class="text-center flex flex-col justify-center space-y-2 mb-12" out:fade>
		<h1>Hi {$currentUser?.name}</h1>
		<p class="text-xl">What are we watching today?</p>
	</div>
	<div class="flex justify-between w-full px-12">
		<p class="text-xl">Pinned Apps</p>
		<Button class="w-auto pr-6">
			All apps
			<RiArrowRightSLine class="fill-carnation-pink absolute right-1" />
		</Button>
	</div>
	{#each apps as app, i}
		{#if hasApps}
			<div class="flex items-center justify-center" in:scale={{ delay: 300 + 100 * i }}>
				<AppIcon appName={app.name} iconSrc={`/assets/${app.icon}`} onSelect={onAppSelect} id={i} />
			</div>
		{/if}
	{/each}
</div>
