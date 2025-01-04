<script>
import { onMount } from "svelte";

let canvas, canvasCtx;
let analyser, dataArray, bufferLength;

let { className, width, height, fft, audioCtx, noiseNode } = $props();

onMount(() => {
	analyser = audioCtx.createAnalyser();
	analyser.fftSize = fft;
	bufferLength = analyser.frequencyBinCount;
	dataArray = new Uint8Array(bufferLength);

	noiseNode.connect(analyser);

	canvasCtx = canvas.getContext('2d');

	draw();
});

function interpolateColor(value, maxValue) {
    const startColor = { r: 250, g: 185, b: 164 };
    const endColor = { r: 240, g: 128, b: 128 };

    const t = value / maxValue;

    const r = Math.round((1 - t) * startColor.r + t * endColor.r);
    const g = Math.round((1 - t) * startColor.g + t * endColor.g);
    const b = Math.round((1 - t) * startColor.b + t * endColor.b);

	return `rgb(${r}, ${g}, ${b})`
}

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

		canvasCtx.fillStyle = interpolateColor(barHeight, 100);
		canvasCtx.fillRect(x, height - barHeight / 1.7, barWidth, barHeight);

		x += barWidth + 1;
	}
}
</script>

<canvas class={className} {width} {height} bind:this={canvas}></canvas>
