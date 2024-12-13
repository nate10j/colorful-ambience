<script>
	import "../app.css";
	import NavigationBar from "$lib/NavigationBar.svelte";

	let { children } = $props();
</script>

<NavigationBar/>

{@render children()}
