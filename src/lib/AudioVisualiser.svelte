<script>
import { onMount } from "svelte";

let canvas, canvasCtx;
let analyser, dataArray, bufferLength;

let { width, height, fft, audioCtx, noiseNode } = $props();

onMount(() => {
	analyser = audioCtx.createAnalyser();
	analyser.fftSize = fft;
	bufferLength = analyser.frequencyBinCount;
	dataArray = new Uint8Array(bufferLength);

	noiseNode.connect(analyser);

	canvasCtx = canvas.getContext('2d');

	draw();
});

function draw() {
	requestAnimationFrame(draw);
	analyser.getByteFrequencyData(dataArray);

	canvasCtx.fillStyle = 'rgb(0, 0, 0)';
	canvasCtx.clearRect(0, 0, width, height);

	const barWidth = (width / bufferLength) - 1;
	let barHeight;
	let x = 0;

	for (let i = 0; i < bufferLength; i++) {
		barHeight = dataArray[i] / 1.7;

		canvasCtx.fillStyle = 'rgb(' + (barHeight + 100) + ',50,50)';
		canvasCtx.fillRect(x, height - barHeight / 1.7, barWidth, barHeight);

		x += barWidth + 1;
	}
}
</script>

<style>
</style>

<canvas {width} {height} bind:this={canvas}></canvas>
