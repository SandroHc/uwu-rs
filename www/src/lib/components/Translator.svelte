<script lang="ts">
	import {onMount} from "svelte";
	import init, {uwuify} from "$wasm/uwu";

	let input = "";
	let loading = true;

	onMount(async () => {
		await init();
		loading = false;
	});
</script>

<div class="card">
	<!-- svelte-ignore a11y-autofocus -->
	<textarea autofocus class="input" placeholder="Enter your text" bind:value={input}/>

	{#if input}
		<div class="translated">
			{#if loading}
				<div class="loading">
					...
				</div>
			{:else}
				{uwuify(input)}
			{/if}
		</div>
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
		}
	}

	.loading {
		@media (prefers-reduced-motion: no-preference) {
			--bg-size: 200%;
			--color-one: var(--text-color);
			--color-two: var(--text-color-lighter-1);
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
