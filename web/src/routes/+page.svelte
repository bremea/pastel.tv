<script>
	import Button from '$lib/components/input/Button.svelte';
	import TextInput from '$lib/components/input/TextInput.svelte';
	import Error from '$lib/components/misc/Error.svelte';

	function checkEmail() {}
</script>

<div class="w-full h-screen bg-gradient-to-r from-uranian-blue to-celadon pb-1.5">
	<div class="w-full h-full flex items-center justify-center bg-night flex-col">
		<div class="space-y-4 flex flex-col items-center">
			<h1 class="text-carnation-pink">pastel.tv</h1>
			<p>Enter your email to continue</p>
			<Error>err</Error>
			<div class="w-64 space-y-4 pt-2">
				<TextInput type="email" />
				<Button onClick={checkEmail}>Continue</Button>
			</div>
		</div>
	</div>
</div>
