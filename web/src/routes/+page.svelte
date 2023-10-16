<script>
	import Button from "$lib/components/input/Button.svelte";
	import TextInput from "$lib/components/input/TextInput.svelte";

</script>
<div class="w-full h-screen bg-gradient-to-r from-uranian-blue to-celadon pb-1.5">
	<div class="w-full h-full flex items-center justify-center bg-night">
		<div class="space-y-4 flex flex-col items-center">
			<h1 class="text-carnation-pink">pastel.tv</h1>
			<p>Enter your email to continue</p>
			<TextInput type="email" />
			<Button>Continue</Button>
		</div>
	</div>
</div>
