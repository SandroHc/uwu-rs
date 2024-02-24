<script lang="ts">
	// import svelteLogo from "../assets/svelte.svg";
	// import viteLogo from "../assets/vite.svg";
	import Translator from "$lib/components/Translator.svelte";
</script>

<!--<div>-->
<!--	<a href="https://vitejs.dev" target="_blank" rel="noreferrer">-->
<!--		<img src={viteLogo} class="logo" alt="Vite Logo"/>-->
<!--	</a>-->
<!--	<a href="https://svelte.dev" target="_blank" rel="noreferrer">-->
<!--		<img src={svelteLogo} class="logo svelte" alt="Svelte Logo"/>-->
<!--	</a>-->
<!--</div>-->

<div class="header">
	uwuify
</div>

<Translator/>

<style lang="scss">
	.header {
		font-size: 3.5em;
		font-weight: bold;
		letter-spacing: -0.05em;
		margin: 0 auto;
	}
</style>
