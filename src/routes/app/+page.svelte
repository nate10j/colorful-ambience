<script>
import audioProcessorUrl from "$lib/audio-processor.js?url";
import { onMount } from "svelte";

let audioCtx;

onMount(() => {
	setup();
});

async function setup() {
	audioCtx = new AudioContext();
	await audioCtx.audioWorklet.addModule(audioProcessorUrl);
	const worklet = new AudioWorkletNode(audioCtx, "NoiseProcessor");
	worklet.connect(audioCtx.destination);
}

function playWhiteNoise() {
	audioCtx.resume();
}
</script>

<div class="container">
	<button type="button" on:click={playWhiteNoise}>white noise</button> 
</div>
