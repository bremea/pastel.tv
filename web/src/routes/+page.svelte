<script lang="ts">
	import * as caller from '$lib/caller/account';
	import Button from '$lib/components/input/Button.svelte';
	import EmailInput from '$lib/components/input/EmailInput.svelte';
	import NumberInput from '$lib/components/input/NumberInput.svelte';
	import PasswordInput from '$lib/components/input/PasswordInput.svelte';
	import TextInput from '$lib/components/input/TextInput.svelte';
	import Error from '$lib/components/misc/Error.svelte';
	import { fade, scale, slide } from 'svelte/transition';

	const helperText = [
		'Enter your email to continue',
		'Enter your password',
		'Enter your name, and create a password',
		'Check your email for a code'
	];

	let error: undefined | string;
	let loading = false;

	let loginStage = 0;
	let loginInfo = {
		email: '',
		password: '',
		confirmPassword: '',
		name: ''
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
				await sendEmailConf();
				break;
			}
			case 3: {
				await register();
				break;
			}
		}
		loading = false;
	}

	async function getEmailInfo() {
		if (loginInfo.email === undefined || loginInfo.email === '') {
			error = 'Enter an email';
			return;
		}
		const res = await caller.checkEmail(loginInfo.email);
		if (res.error) {
			error = res.message;
		} else {
			if (res.new_account) loginStage = 2;
			else loginStage = 1;
		}
	}

	async function sendEmailConf() {
		if (loginInfo.password === undefined || loginInfo.password === '') {
			error = 'Enter a password';
			return;
		}
		if (loginInfo.password != loginInfo.confirmPassword) {
			error = "Passwords don't match";
			return;
		}

		const res = await caller.sendEmailVerify(loginInfo.email, loginInfo.name);
		if (res.error) {
			error = res.message;
		} else {
			loginStage = 3;
		}
	}

	async function login() {
		return;
	}

	async function register() {
		return;
	}
</script>

<div class="w-full h-screen bg-gradient-to-r from-uranian-blue to-celadon pb-1.5">
	<div class="w-full h-full flex items-center justify-center bg-night flex-col">
		<div class="space-y-4 flex flex-col items-center">
			<h1 class="text-carnation-pink">pastel.tv</h1>

			<div class="layered-transition text-center">
				{#each helperText as textItem, i}
					{#if loginStage == i}
						<p out:fade in:fade={{ delay: 500 }}>{textItem}</p>
					{/if}
				{/each}
			</div>

			{#if error}
				<Error>{error}</Error>
			{/if}
			<div class="w-64 space-y-4 pt-2">
				{#if loginStage < 3}
					<div transition:slide class="space-y-4">
						<EmailInput bind:value={loginInfo.email} autocomplete="email" />
						{#if loginStage == 1 || loginStage == 2}
							<div transition:slide class="space-y-4">
								{#if loginStage == 2}
									<TextInput
										bind:value={loginInfo.name}
										placeholder="Your name"
										autocomplete="nickname"
									/>
								{/if}
								<PasswordInput
									bind:value={loginInfo.password}
									placeholder="Password"
									autocomplete={loginStage == 2 ? 'new-password' : 'current-password'}
								/>
								{#if loginStage == 2}
									<PasswordInput
										bind:value={loginInfo.confirmPassword}
										placeholder="Confirm password"
										autocomplete="new-password"
									/>
								{/if}
							</div>
						{/if}
					</div>
				{:else if loginStage == 3}
					<div transition:slide class="space-y-4">
						<NumberInput placeholder="Verification code" autocomplete="one-time-code" />
					</div>
				{/if}
				<Button onClick={runButtonClick} {loading} disabled={loading}>Continue</Button>
			</div>
		</div>
	</div>
</div>
