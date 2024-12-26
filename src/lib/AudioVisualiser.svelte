<script>
import { onMount } from "svelte";

let canvas, canvasCtx;
let analyser, dataArray, bufferLength;

let { width, height, audioCtx, noiseNode } = $props();

onMount(() => {
	analyser = audioCtx.createAnalyser();
	analyser.fftSize = 1024;
	bufferLength = analyser.frequencyBinCount;
	dataArray = new Uint8Array(bufferLength);

	noiseNode.connect(analyser);
	analyser.connect(audioCtx.destination);

	canvasCtx = canvas.getContext('2d');

	draw();
});

function draw() {
	analyser.getByteFrequencyData(dataArray);

	canvasCtx.fillStyle = 'rgb(0, 0, 0)';
	canvasCtx.clearRect(0, 0, width, height);

	const barWidth = (width / bufferLength) * 2.5;
	let barHeight;
	let x = 0;

	for (let i = 0; i < bufferLength; i++) {
		barHeight = dataArray[i] / 2;

		canvasCtx.fillStyle = 'rgb(' + (barHeight + 100) + ',50,50)';
		canvasCtx.fillRect(x, height - barHeight / 2, barWidth, barHeight);

		x += barWidth + 1;
	}

	requestAnimationFrame(draw);
}
</script>

<style>
canvas {
	display: block;
	margin: 0 auto;
	border: 1px solid black;
}
</style>

<canvas {width} {height} bind:this={canvas}></canvas>
