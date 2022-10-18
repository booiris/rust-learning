import { initSync } from "./screeps-arena";
import wasm_bytes from "./screeps-arena_bg.wasm.bin";
initSync(wasm_bytes);
export * from "./screeps-arena";
