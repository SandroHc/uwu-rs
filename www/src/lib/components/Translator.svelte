<script lang="ts">
	import {onMount} from "svelte";
	import init, {uwuify} from "../../../../pkg/uwu";

	let input = "";
	let loaded = false;

	onMount(async () => {
		await init();
		loaded = true;
	});

	function translate(input: string) {
		if (!loaded) {
			return input;
		}
		return uwuify(input);
	}
</script>

<div class="card">
	<!-- svelte-ignore a11y-autofocus -->
	<textarea autofocus class="input" placeholder="Enter your text" bind:value={input}/>

	<div class="translated">
		{translate(input)}
	</div>
</div>

<style lang="scss">
	.card {
		$radius: 0.5rem;
		$border-width: 1px;

		background-color: var(--background-color-darker-1);
		border: $border-width solid var(--background-color-darker-1);
		border-radius: calc($radius - $border-width*2);
		min-width: 600px;
		max-width: 600px;

		> * {
			padding: 0.5rem 0.75rem;
		}

		.input {
			background-color: var(--background-color-lighter-1);
			border-radius: calc($radius - $border-width*2);
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
</style>
