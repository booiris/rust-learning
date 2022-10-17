import { initSync } from "./rust-screeps";
import wasm_bytes from "./rust-screeps_bg.wasm.bin";
initSync(wasm_bytes);
export * from "./rust-screeps";
