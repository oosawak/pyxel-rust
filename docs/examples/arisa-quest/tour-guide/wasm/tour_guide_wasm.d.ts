/* tslint:disable */
/* eslint-disable */

export function add_document(title: string, body: string): void;

export function build_prompt_text(query: string): string;

export function clear_documents(): void;

export function context_limit(): number;

export function document_count(): number;

export function load_documents(json_text: string): void;

export function query_guide(query: string): string;

export function set_context_limit(limit: number): void;

export function set_system_prompt(prompt: string): void;

export function system_prompt_text(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly add_document: (a: number, b: number, c: number, d: number) => void;
    readonly build_prompt_text: (a: number, b: number, c: number) => void;
    readonly clear_documents: () => void;
    readonly context_limit: () => number;
    readonly document_count: () => number;
    readonly load_documents: (a: number, b: number, c: number) => void;
    readonly query_guide: (a: number, b: number, c: number) => void;
    readonly set_context_limit: (a: number) => void;
    readonly set_system_prompt: (a: number, b: number) => void;
    readonly system_prompt_text: (a: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number) => void;
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
