/* tslint:disable */
/* eslint-disable */

export function commit_fonts(): void;

export function register_canvas_font(data: Uint8Array): void;

export function register_math_font(data: Uint8Array): void;

export function run_wasm(): Promise<void>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly commit_fonts: () => void;
    readonly register_canvas_font: (a: number, b: number) => void;
    readonly register_math_font: (a: number, b: number) => void;
    readonly run_wasm: () => void;
    readonly __wasm_bindgen_func_elem_20949: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_18249: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_20959: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_13108: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_4: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_5620: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_6: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_6739: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_8: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_6216: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_10: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_11: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_12: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18248_13: (a: number, b: number, c: number) => void;
    readonly __wasm_bindgen_func_elem_18247: (a: number, b: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export5: (a: number, b: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
