<script>
	import { RiErrorWarningFill } from 'svelte-remixicon';
</script>

<div class="bg-carnation-pink rounded-full font-bold text-black px-4 py-1 flex">
	<RiErrorWarningFill class="h-6 w-6 fill-black mr-1.5" />
	<slot />
</div>
