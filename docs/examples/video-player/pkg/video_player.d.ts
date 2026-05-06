/* tslint:disable */
/* eslint-disable */

/**
 * AES-256-CTR でデータを復号する（先頭から）
 */
export function decrypt_chunk(data: Uint8Array, key: Uint8Array, iv: Uint8Array): Uint8Array;

/**
 * AES-256-CTR でデータをオフセット指定で復号する（ストリーミング用）
 *
 * # Arguments
 * * `data`        - 暗号化されたバイト列
 * * `key`         - 32バイトのAESキー
 * * `iv`          - 16バイトのIV（ノンス）
 * * `byte_offset` - ストリーム上のバイトオフセット（前チャンクの合計バイト数）
 */
export function decrypt_chunk_at(data: Uint8Array, key: Uint8Array, iv: Uint8Array, byte_offset: bigint): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly decrypt_chunk: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
    readonly decrypt_chunk_at: (a: number, b: number, c: number, d: number, e: number, f: number, g: bigint) => [number, number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
