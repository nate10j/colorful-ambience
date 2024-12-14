<script>
	// font
	import "../styles/inter.css"

	import "../styles/app.css";

	// components
	import NavigationBar from "$lib/NavigationBar.svelte";

	let { children } = $props();
</script>

<NavigationBar/>

{@render children()}
