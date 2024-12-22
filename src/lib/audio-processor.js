// note: IDE will have issues with this file ignore lsp errors
// IDE does not understand context of AudioWorkletProcessor
// This is loaded in app/+page.svelte via audioWorklet.addModule()
class NoiseProcessor extends AudioWorkletProcessor {
	constructor() {
		super();
	}
	process(inputs, outputs, parameters) {
		for (let i = 0; i < outputs[0].length; i++) {
		}
		return true;
	}
}

registerProcessor("NoiseProcessor", NoiseProcessor);

export default NoiseProcessor;
