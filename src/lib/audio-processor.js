import { initSync, ColorNoise, NoiseGenerator } from "noise_generator";

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
