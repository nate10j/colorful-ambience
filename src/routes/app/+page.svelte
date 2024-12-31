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
let gainNode;

onMount(() => {
	wasmModulePromise = fetch(wasmUrl).then((res) => res.arrayBuffer());
	setup();
});

async function setup() {
	audioCtx = new AudioContext();
	audioCtx.suspend();

	gainNode = new GainNode(audioCtx);
	gainNode.gain.value = 0.8;
	await audioCtx.audioWorklet.addModule(audioProcessorUrl);

	const wasmModule = await wasmModulePromise;

	worklet = new AudioWorkletNode(audioCtx, "NoiseProcessor", {
		processorOptions: { wasmModule },
	});

	worklet.connect(gainNode);
	gainNode.connect(audioCtx.destination)
}

function selectWhiteNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.White})
}

function selectPinkNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.Pink})
}

function selectBrownNoise() {
	worklet.port.postMessage({type: "updateColorNoise", data: ColorNoise.Brown})
}

let toggleText = "Play";
let playing = false;

function toggleButtonClick() {
	if (playing) {
		pause();
		playing = false;
	} else {
		play();
		playing = true;
	}
}

function play() {
	audioCtx.resume();
	toggleText = "Pause"
}

function pause() {
	audioCtx.suspend();
	toggleText = "Play"
}

function onVolumeChange(event) {
	gainNode.gain.value = event.target.value / 100, audioCtx.currentTime;
}
</script>

<div class="container">
	{#if audioCtx != null && worklet != null}
		<div class="visualiser-container">
			<AudioVisualiser width={350} height={120} fft={64} audioCtx={audioCtx} noiseNode={gainNode}/>
		</div>
	{/if}
	<div class="controls">
		<ul>
			<li><button class="white" on:click={selectWhiteNoise}>White</button></li>
			<li><button class="pink" on:click={selectPinkNoise}>Pink</button></li>
			<li><button class="brown" on:click={selectBrownNoise}>Brown</button></li>
		</ul>
		<div>
		<input class="volume" type="range" value="80" min="0" max="100" on:change={onVolumeChange}>
		</div>
		<button class="toggle" on:click={toggleButtonClick}>{toggleText}</button>
	</div>
</div>

<style>
.container {
	display: flex;
	justify-content: center;
	align-items: center;
	box-sizing: border-box;
	flex-wrap: wrap;
}

.visualiser-container {
	width: 100%;
	max-width: 350px;
	margin: 0 1rem;
	margin-bottom: 0.5rem;
}

.controls {
	display: flex;
	flex-direction: column;
	align-items: center;
	width: 350px;
}

ul {
	list-style: none;
	display: flex;
	width: 100%;
	justify-content: space-between;
}

li button {
	font-weight: 700;
	font-size: 1rem;
	width: 100px;
	height: 100px;
	border: none;
	cursor: pointer;
	border-radius: 6px;
	transition: transform 0.2s, opacity 0.2s;
}

li button:hover {
	opacity: 80%;
	transform: scale(1.05);
}

li .white {
	color: #552D2D;
	background-color: #FFF4EA;
}

li .pink {
	color: #552D2D;
	background-color: #F59D92;
}

li .brown {
	color: #FFF4EA;
	background-color: #874747;
}

.volume {
	width: 100%;
	margin: 5px 0;
}

.toggle {
	font-size: 1rem;
	padding: 0.5rem 1.5rem;
	width: 100%;
	border: none;
	cursor: pointer;
	border-radius: 6px;
	background-color: #333;
	color: #fff;
	transition: background-color 0.2s, transform 0.2s;
}

.toggle:hover {
	background-color: #555;
	transform: scale(1.05);
}

/* Media query to adjust layout for larger screens */
@media (min-width: 768px) {
	.container {
		flex-direction: row-reverse;
		align-items: flex-start;
	}

	.controls {
		align-items: flex-start;
	}
}
</style>
