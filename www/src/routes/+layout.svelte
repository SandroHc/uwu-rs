<script lang="ts">
	import "../app.scss";
	import Footer from "$lib/components/Footer.svelte";
</script>

<svelte:head>
	<title>uwuify</title>
	<meta name="description" content="uwuify your life! :3"/>
</svelte:head>

<main>
	<slot/>
</main>

<Footer/>

<style lang="scss">
	main {
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		gap: 1rem;
		justify-content: center;
		align-items: center;

		@media (width >= 1000px) {
			gap: 2rem;
		}
	}
</style>
