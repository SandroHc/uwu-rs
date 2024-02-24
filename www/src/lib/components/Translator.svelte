<script lang="ts">
	import {onMount} from "svelte";
	//import init, {uwuify} from "$wasm/uwu";
	import init, {uwuify} from "uwu-rs";

	let input = "Please DO NOT announce to the server when you are going to go masturbate. This has been a reoccurring issue, and I'm not sure why some people have such under developed social skills that they think that a server full of mostly male strangers would need to know that. No one is going to be impressed and give you a high five (especially considering where that hand has been). I don't want to add this to the rules, since it would be embarrassing for new users to see that we have a problem with this, but it is going to be enforced as a rule from now on. If it occurs, you will be warned, then additional occurrences will be dealt with at the discretion of mod staff. Thanks.";
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
