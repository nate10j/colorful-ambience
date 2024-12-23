// note: IDE will have issues with this file ignore lsp errors
// IDE does not understand context of AudioWorkletProcessor
// This is loaded in app/+page.svelte via audioWorklet.addModule()
import { initSync, NoiseGenerator } from "./pkg/noise_generator";

class NoiseProcessor extends AudioWorkletProcessor {
	generator;

	constructor(options) {
		super();

		// this is neccessary or else it wont load the module in
		let { wasmModule } = options?.processorOptions;
		initSync( wasmModule );

		// construct an oscillator (in wasm)
		this.generator = NoiseGenerator.new();
	}
	process(inputs, outputs, parameters) {
		for (let i = 0; i < outputs[0].length; i++) {
			this.generator.process(outputs[0][i])
		}
		return true;
	}
}

registerProcessor("NoiseProcessor", NoiseProcessor);

export default NoiseProcessor;
