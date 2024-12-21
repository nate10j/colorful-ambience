<script>
import init, { greet } from "noise-generator"
import audioProcessorUrl from "$lib/audio-processor.js?url";
import { onMount } from "svelte";

onMount(() => {
	setup();
});

async function setup() {
	const audioCtx = new AudioContext();
	await audioCtx.audioWorklet.addModule(audioProcessorUrl);
}

function playWhiteNoise() {
	alert("playing white noise...");
}
</script>

<div class="container">
	<button type="button" on:click={playWhiteNoise}>white noise</button> 
</div>
