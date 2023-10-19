<script lang="ts">
	import { checkEmail } from '$lib/caller/account';
	import Button from '$lib/components/input/Button.svelte';
	import EmailInput from '$lib/components/input/EmailInput.svelte';
	import PasswordInput from '$lib/components/input/PasswordInput.svelte';
	import TextInput from '$lib/components/input/TextInput.svelte';
	import Error from '$lib/components/misc/Error.svelte';
	import { fade, scale, slide } from 'svelte/transition';

	const helperText = {
		enterEmail: 'Enter your email to continue',
		enterPassExisting: 'Enter your password',
		enterPassNew: 'Create a password',
		captcha: 'Let\'s make sure you\'re a human'
	};

	let error: undefined | string;
	let loading = false;
	let email: string = "";
	let password: string = "";
	let confirmPassword: string = "";

	let loginStage = 0;
	let loginInfo = {
		email,
		password
	};

	async function runButtonClick() {
		error = undefined;
		loading = true;
		switch (loginStage) {
			case 0: {
				await getEmailInfo();
				break;
			}
			case 1: {
				await login();
				break;
			}
			case 2: {
				await register();
				break;
			}
		}
		loading = false;
	}

	async function getEmailInfo() {
		if (email == undefined) {
			error = 'Enter an email';
			return;
		}
		const res = await checkEmail(email);
		if (res.error) {
			error = res.message;
		} else {
			if (res.new_account) loginStage = 2;
			else loginStage = 1;
		}
	}

	async function register() {
		if (password == undefined || password == "") {
			error = 'Enter a password';
			return;
		}
		if (password != confirmPassword) {
			error = 'Passwords don\'t match';
			return;
		}

		return;
	}

	async function login() {
		return;
	}
</script>

<div class="w-full h-screen bg-gradient-to-r from-uranian-blue to-celadon pb-1.5">
	<div class="w-full h-full flex items-center justify-center bg-night flex-col">
		<div class="space-y-4 flex flex-col items-center">
			<h1 class="text-carnation-pink">pastel.tv</h1>

			<div class="layered-transition text-center">
				{#if loginStage == 0}
					<p out:fade>{helperText.enterEmail}</p>
				{:else if loginStage == 1}
					<p transition:fade={{ delay: 500 }}>{helperText.enterPassExisting}</p>
				{:else if loginStage == 2}
					<p transition:fade={{ delay: 500 }}>{helperText.enterPassNew}</p>
				{/if}
			</div>

			{#if error}
				<Error>{error}</Error>
			{/if}
			<div class="w-64 space-y-4 pt-2">
				<EmailInput bind:value={email} />
				{#if loginStage == 1 || loginStage == 2}
					<div transition:slide class="space-y-4">
						<PasswordInput bind:value={password} placeholder="Password" />
						{#if loginStage == 2}
							<PasswordInput bind:value={confirmPassword} placeholder="Confirm password" />
						{/if}
					</div>
				{/if}
				<Button onClick={runButtonClick} {loading} disabled={loading}>Continue</Button>
			</div>
		</div>
	</div>
</div>
