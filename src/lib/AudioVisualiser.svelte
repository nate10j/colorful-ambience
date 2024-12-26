<script>
import { onMount } from "svelte";

let canvas, canvasCtx;
let analyser, dataArray, bufferLength;

let { width, height, audioCtx, noiseNode } = $props();

onMount(() => {
	analyser = audioCtx.createAnalyser();
	analyser.fftSize = 2048;
	bufferLength = analyser.frequencyBinCount;
	dataArray = new Uint8Array(bufferLength);

	noiseNode.connect(analyser);
	analyser.connect(audioCtx.destination);

	canvasCtx = canvas.getContext('2d');

	draw();
});

function draw() {
	analyser.getByteTimeDomainData(dataArray);

	canvasCtx.fillStyle = 'rgb(200, 200, 200)';
	canvasCtx.fillRect(0, 0, canvas.width, canvas.height);

	canvasCtx.lineWidth = 2;
	canvasCtx.strokeStyle = 'rgb(0, 0, 0)';
	canvasCtx.beginPath();

	const sliceWidth = canvas.width * 1.0 / bufferLength;
	let x = 0;

	for (let i = 0; i < bufferLength; i++) {
		const v = dataArray[i] / 128.0;
		const y = v * canvas.height / 2;

		if (i === 0) {
			canvasCtx.moveTo(x, y);
		} else {
			canvasCtx.lineTo(x, y);
		}

		x += sliceWidth;
	}

	canvasCtx.lineTo(canvas.width, canvas.height / 2);
	canvasCtx.stroke();

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
