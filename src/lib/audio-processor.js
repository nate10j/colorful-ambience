// note: IDE will have issues with this file ignore lsp errors
// IDE does not understand context of AudioWorkletProcessor
// This is loaded in app/+page.svelte via audioWorklet.addModule()
import { initSync, ColorNoise, NoiseGenerator } from "./pkg/noise_generator";

class NoiseProcessor extends AudioWorkletProcessor {
	generator;

	constructor(options) {
		super();

		// this is neccessary or else it wont load the module in
		let { wasmModule } = options?.processorOptions;

		// deprecated paremeter, fix later (technical debt)
		initSync( wasmModule );

		this.generator = NoiseGenerator.new(ColorNoise.White);

		this.port.onmessage = (e) => {
			const msg = e.data;
			switch (msg.type) {
				case "updateColorNoise":
					this.generator.change_color(msg.data);
			}
		};
	}

	process(inputs, outputs, parameters) {
		this.generator.process(outputs[0][0])
		return true;
	}
}

registerProcessor("NoiseProcessor", NoiseProcessor);

export default NoiseProcessor;
