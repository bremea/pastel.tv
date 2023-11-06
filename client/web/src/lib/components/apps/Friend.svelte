<script>
	import { RiTvLine } from 'svelte-remixicon';

	export let id = 0;
</script>

<div
	class="rounded-full bg-night-400 p-1 space-x-4 w-1.5-app-icon flex items-center justify-start hover:bg-night-550 transition-all active:scale-90"
>
	<img
		src="https://cdn.discordapp.com/avatars/710164635524923575/9c105248c06139ec32bac7d4dd2a5440.webp"
		alt="Jade's pfp"
		class="rounded-full h-12"
	/>
	<div class="flex-col">
		<p class="font-bold text-lg">jade{id}</p>
		<div class="flex items-center space-x-1">
			<RiTvLine size="16px" />
			<p class="text-sm">YouTube</p>
		</div>
	</div>
</div>
