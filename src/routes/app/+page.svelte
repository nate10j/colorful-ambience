<script>
import audioProcessorUrl from "$lib/audio-processor.js?url";
import wasmUrl from "$lib/pkg/noise_generator_bg.wasm?url";

import { ColorNoise } from "$lib/pkg/noise_generator";

import { onMount } from "svelte";
import AudioVisualiser from "$lib/AudioVisualiser.svelte";
import startVisualiser from "$lib/AudioVisualiser.svelte";

let wasmModulePromise;
let audioCtx;
let worklet;

onMount(() => {
	wasmModulePromise = fetch(wasmUrl).then((res) => res.arrayBuffer());
	setup();
});

async function setup() {
	audioCtx = new AudioContext();
	audioCtx.suspend();

	await audioCtx.audioWorklet.addModule(audioProcessorUrl);

	const wasmModule = await wasmModulePromise;

	worklet = new AudioWorkletNode(audioCtx, "NoiseProcessor", {
		processorOptions: { wasmModule },
	});

	worklet.connect(audioCtx.destination);
}

function playWhiteNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.White})
	if (audioCtx.state === "suspended") {
		audioCtx.resume();
	}
}

function playPinkNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.Pink})
	if (audioCtx.state === "suspended") {
		audioCtx.resume();
	}
}

function playBrownNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.Brown})

	if (audioCtx.state === "suspended") {
		audioCtx.resume();
	}
}
</script>

<div class="container">
	<form>
		<div>
			<button class="white" on:click={playWhiteNoise}>White noise</button>
			<button class="pink" on:click={playPinkNoise}>Pink noise</button>
			<button class="brown" on:click={playBrownNoise}>Brown noise</button>
		</div>
	</form>

	{#if audioCtx != null && worklet != null}
		<AudioVisualiser width={400} height={100} audioCtx={audioCtx} noiseNode={worklet}/>
	{/if}
</div>

<style>
</style>
