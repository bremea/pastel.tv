<script lang="ts">
	export let placeholder = 'Email';
	let bindValue: string = "";
	export { bindValue as value };
</script>

<input
	class="w-full bg-night border-white border-opacity-25 border-2 rounded-full px-4 py-1 transition-all focus:border-carnation-pink"
	{placeholder}
	type="email"
	bind:value={bindValue}
/>
