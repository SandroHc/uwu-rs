<script lang="ts">
	import type {FormEventHandler} from "svelte/elements";
	import {onMount} from "svelte";
	import init, {uwuify} from "uwu-rs";

	let input = "";
	let loading = true;
	let copied = false;
	$: translated = loading ? input : uwuify(input);

	onMount(async () => {
		await init();
		loading = false;
	});

	function fitInput(event: FormEventHandler<HTMLTextAreaElement>) {
		const elem = event.currentTarget;
		elem.style.height = "";
		elem.style.height = elem.scrollHeight + "px";
	}

	async function copyToClipboard() {
		try {
			await navigator.clipboard.writeText(translated);

			// Highlight text for 1 second to signal success
			copied = true;
			setTimeout(() => (copied = false), 1000);
		} catch (error) {
			console.error(error);
		}
	}

	async function handleKeyPress(event: KeyboardEvent) {
		if (event.key === "Enter") {
			await copyToClipboard();
		}
	}
</script>

<div class="card">
	<!-- svelte-ignore a11y-autofocus -->
	<textarea autofocus class="input" placeholder="write here :3" bind:value={input} on:input={fitInput}/>

	{#if input}
		{#if loading}
			<div class="loading">
				...
			</div>
		{:else}
			<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
			<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
			<div class="translated" class:copied={copied} tabindex="0" role="main" on:click={copyToClipboard}
				 on:keyup={handleKeyPress}>
				{translated}
			</div>
		{/if}
	{/if}
</div>

<style lang="scss">
	.card {
		$radius: 0.5rem;
		$border-width: 1px;

		background-color: var(--background-color-darker-1);
		border: $border-width solid var(--background-color-darker-1);
		border-radius: calc($radius - $border-width * 2);
		min-width: 600px;
		max-width: 600px;

		> * {
			padding: 0.5rem 0.75rem;
		}

		.input {
			background-color: var(--background-color-lighter-1);
			border-radius: calc($radius - $border-width * 2);
			border: none;
			color: var(--text-color);
			font-family: inherit; // Override default monospaced font
			margin: 0;
			resize: vertical;
			width: 100%;

			&:focus-visible {
				outline: 2px solid var(--background-color-darker-2);
			}
		}

		.translated {
			color: var(--text-color-lighter-1);
			cursor: pointer;
			white-space: break-spaces;

			&:hover,
			&:focus {
				color: var(--text-color);
			}

			&.copied {
				color: #ebffd1;
			}

			@media (prefers-reduced-motion: no-preference) {
				transition: color 100ms ease-in;
			}
		}
	}

	.loading {
		@media (prefers-reduced-motion: no-preference) {
			--bg-size: 200%;
			--color-one: var(--text-color);
			--color-two: var(--text-color-lighter-2);
			animation: breathe 2s infinite linear;
			background: linear-gradient(90deg, var(--color-one), var(--color-two), var(--color-one)) 0 0 / var(--bg-size) 100%;
			background-clip: text;
			color: transparent;

			@keyframes breathe {
				to {
					background-position: var(--bg-size) 0;
				}
			}
		}
	}
</style>
