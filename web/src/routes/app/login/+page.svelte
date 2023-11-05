<script lang="ts">
	import { goto } from '$app/navigation';
	import * as caller from '$lib/caller/account';
	import Button from '$lib/components/input/Button.svelte';
	import EmailInput from '$lib/components/input/EmailInput.svelte';
	import PasswordInput from '$lib/components/input/PasswordInput.svelte';
	import TextInput from '$lib/components/input/TextInput.svelte';
	import Error from '$lib/components/misc/Error.svelte';
	import { accessToken, currentUser } from '$lib/context/context';
	import { fade, scale, slide } from 'svelte/transition';

	const helperText = [
		'Enter your email to continue',
		'Enter your password',
		'Enter your name, and create a password',
		'Check your email for a verification code'
	];

	let error: undefined | string;
	let loading = false;
	let navigating = false;

	let loginStage = 0;
	let loginInfo = {
		email: '',
		password: '',
		confirmPassword: '',
		name: '',
		otp: ''
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
		if (!navigating) loading = false;
	}

	async function getEmailInfo() {
		if (loginInfo.email === undefined || loginInfo.email === '') {
			error = 'Enter an email';
			return;
		}
		const res = await caller.checkEmail(loginInfo.email);
		if (!res || res.error) {
			error = res?.message;
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
		if (res?.error) {
			error = res.message;
		} else {
			loginStage = 3;
		}
	}

	async function login() {
		if (loginInfo.password === undefined || loginInfo.password === '') {
			error = 'Enter a password';
			return;
		}
		const res = await caller.login(loginInfo.email, loginInfo.password);
		if (!res || res.error) {
			error = res?.message;
		} else {
			accessToken.set(res.access_token);
			const me = await caller.getMe();
			if (me.error) {
				error = 'Internal error';
				return;
			}
			currentUser.set(me);
			navigating = true;
			await goto('/app/launch');
		}
	}

	async function register() {
		if (loginInfo.otp === undefined || loginInfo.otp === '') {
			error = 'Enter a code';
			return;
		}
		const res = await caller.register(
			loginInfo.email,
			loginInfo.name,
			loginInfo.password,
			loginInfo.otp
		);
		if (!res || res.error) {
			error = res?.message;
		} else {
			accessToken.set(res.access_token);
			const me = await caller.getMe();
			if (me.error) {
				error = 'Internal error';
				return;
			}
			currentUser.set(me);
			navigating = true;
			await goto('/app/launch');
		}
	}
</script>

<div class="space-y-4 flex flex-col items-center absolute" transition:fade>
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
	<form class="w-64 space-y-4 pt-2">
		{#if loginStage < 3}
			<div transition:slide class="space-y-4">
				<EmailInput bind:value={loginInfo.email} autocomplete="username" />
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
				<TextInput
					bind:value={loginInfo.otp}
					placeholder="Verification code"
					autocomplete="one-time-code"
				/>
			</div>
		{/if}
		<Button onClick={runButtonClick} {loading} disabled={loading}>Continue</Button>
	</form>
</div>
