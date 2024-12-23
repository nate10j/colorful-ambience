#!/bin/bash

OUT_DIR="../src/lib/pkg"

wasm-pack build --target web --out-dir "${OUT_DIR}" "$@"
patch "${OUT_DIR}/noise_generator.js" wasm.patch
