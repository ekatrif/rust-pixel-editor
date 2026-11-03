import init, { canvas_area } from "pixel-core";
import wasmUrl from "pixel-core/pixel_core_bg.wasm" with { type: "file" };

await init({ module_or_path: wasmUrl });

const status = document.createElement("p");
status.textContent = `Rust result: ${canvas_area(32, 32)}`;
document.body.append(status);