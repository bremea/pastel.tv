<script lang="ts">
	export let placeholder = 'Password';
	let bindValue: string = "";
	export let autocomplete: string = "";
	export { bindValue as value };
</script>

<input
	class="w-full bg-night border-white border-opacity-25 border-2 rounded-full px-4 py-1 transition-all hover:border-carnation-pink focus:border-carnation-pink"
	{placeholder}
	type="password"
	{autocomplete}
	bind:value={bindValue}
/>
