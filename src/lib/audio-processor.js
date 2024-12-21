// note: IDE will have issues with this file ignore lsp errors
// IDE does not understand context of AudioWorkletProcessor
// This is loaded in app/+page.svelte via audioWorklet.addModule()
class NoiseProcessor extends AudioWorkletProcessor {
	process(inputs, outputs, parameters) {
		const output = outputs[0];
		output.forEach((channel) => {
			for (let i = 0; i < channel.length; i++) {
				channel[i] = Math.random() * 2 - 1;
			}
		});
		return true;
	}
}

registerProcessor("NoiseProcessor", NoiseProcessor);

export default NoiseProcessor;
