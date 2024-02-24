<script lang="ts">
	import "../app.scss";
	import Footer from "$lib/components/Footer.svelte";
</script>

<svelte:head>
	<title>uwuify</title>
	<meta name="description" content="uwuify your life"/>
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
		gap: 0.75rem;
		justify-content: center;
		align-items: center;
	}
</style>
