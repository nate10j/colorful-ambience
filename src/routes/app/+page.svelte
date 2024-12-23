<script>
import audioProcessorUrl from "$lib/audio-processor.js?url";
import wasmUrl from "$lib/pkg/noise_generator_bg.wasm?url";

import { onMount } from "svelte";

let wasmModulePromise;
let audioCtx;

onMount(() => {
	wasmModulePromise = fetch(wasmUrl).then((res) => res.arrayBuffer());
	setup();
});

async function setup() {
	audioCtx = new AudioContext();
	await audioCtx.audioWorklet.addModule(audioProcessorUrl);

	const wasmModule = await wasmModulePromise;

	const worklet = new AudioWorkletNode(audioCtx, "NoiseProcessor", {
		processorOptions: { wasmModule },
	});
	worklet.connect(audioCtx.destination);
}

function playWhiteNoise() {
	if (audioCtx.state === "suspended") {
		audioCtx.resume();
	}
}
</script>

<div class="container">
	<button type="button" on:click={playWhiteNoise}>white noise</button> 
</div>
