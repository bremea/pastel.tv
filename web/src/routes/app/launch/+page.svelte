<script lang="ts">
	import { goto } from '$app/navigation';
	import AppIcon from '$lib/components/apps/AppIcon.svelte';
	import Friend from '$lib/components/apps/Friend.svelte';
	import BigSearch from '$lib/components/input/BigSearch.svelte';
	import Tabs from '$lib/components/input/Tabs.svelte';
	import { currentUser } from '$lib/context/context';
	import { onMount } from 'svelte';
	import {
		RiApps2Line,
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

<div transition:fade class="w-full h-full absolute flex flex-col items-center space-y-12">
	<div class="text-center flex flex-col justify-center space-y-2">
		<h1>Hi, <span class="text-carnation-pink">{$currentUser?.name}</span>!</h1>
		<p class="text-xl">What are we watching today?</p>
	</div>

	<BigSearch />

	<div class="relative flex justify-center h-full">
		<div class="space-y-2 px-8">
			<div
				class="grid grid-col-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 justify-center gap-8"
				transition:fly={{ x: -256 }}
			>
				{#each apps as app, i}
					{#if hasApps}
						<div class="flex items-center justify-center" in:scale={{ delay: 300 + 100 * i }}>
							<AppIcon
								appName={app.name}
								iconSrc={`/assets/${app.icon}`}
								onSelect={onAppSelect}
								id={i}
							/>
						</div>
					{/if}
				{/each}
			</div>
		</div>
	</div>
</div>
