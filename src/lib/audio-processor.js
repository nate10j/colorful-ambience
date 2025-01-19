import { initSync, ColorNoise, NoiseGenerator } from "$lib/pkg/noise_generator"

class NoiseProcessor extends AudioWorkletProcessor {
	constructor(options) {
		super();
		this.generator = null;

		// Get the Wasm binary buffer from processor options
		const { wasmModule } = options?.processorOptions;
		initSync(wasmModule);

		this.generator = NoiseGenerator.new();
		
		this.port.onmessage = (e) => {
			const msg = e.data;
			switch (msg.type) {
				case "updateColorNoise":
					if (this.generator) {
						this.generator.change_color(msg.data);
					}
					break;
			}
		};
	}

	process(inputs, outputs, parameters) {
		if (this.generator) {
			this.generator.process(outputs[0][0]);
		}
		return true;
	}
}

registerProcessor("NoiseProcessor", NoiseProcessor);

export default NoiseProcessor;
