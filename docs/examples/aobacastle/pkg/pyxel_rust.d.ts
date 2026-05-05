/* tslint:disable */
/* eslint-disable */

export function get_bg_idx(): number;

export function get_enemy_flash(): number;

export function get_enemy_hp(): number;

export function get_enemy_idx(): number;

export function get_enemy_max_hp(): number;

export function get_game_state(): number;

export function get_player_flash(): number;

export function get_player_hp(): number;

export function get_player_level(): number;

export function get_player_max_hp(): number;

export function get_player_max_mp(): number;

export function get_player_mp(): number;

/**
 * キーインデックスの定数マップを返す (JS から参照用)
 */
export function key_index(code: string): number;

/**
 * WASM entry point — JS calls `main()` after `await init()` to start the Arisa Quest game.
 */
export function main(): void;

/**
 * WASM entry point for Aoba Castle ninja climbing game.
 */
export function main_aobacastle(): void;

/**
 * WASM entry point for NanoTerras light-ring game.
 * Separate from main() — arisa state is completely unaffected.
 */
export function main_nanoteras(): void;

/**
 * タッチ UI から直接呼べるキー状態セット関数
 */
export function set_key(idx: number, down: boolean): void;

/**
 * Called from JS to start a battle against enemy at given index.
 * The game loop picks this up next frame and transitions to Battle state.
 */
export function start_battle_from_js(idx: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly get_bg_idx: () => number;
    readonly get_enemy_flash: () => number;
    readonly get_enemy_hp: () => number;
    readonly get_enemy_idx: () => number;
    readonly get_enemy_max_hp: () => number;
    readonly get_game_state: () => number;
    readonly get_player_flash: () => number;
    readonly get_player_hp: () => number;
    readonly get_player_level: () => number;
    readonly get_player_max_hp: () => number;
    readonly get_player_max_mp: () => number;
    readonly get_player_mp: () => number;
    readonly key_index: (a: number, b: number) => number;
    readonly set_key: (a: number, b: number) => void;
    readonly start_battle_from_js: (a: number) => void;
    readonly main: () => void;
    readonly main_aobacastle: () => void;
    readonly main_nanoteras: () => void;
    readonly wasm_bindgen_7409b725ef28e7cd___convert__closures_____invoke___web_sys_e2dca692cdd4c806___features__gen_MouseEvent__MouseEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_7409b725ef28e7cd___convert__closures_____invoke___web_sys_e2dca692cdd4c806___features__gen_MouseEvent__MouseEvent______true__1: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_7409b725ef28e7cd___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
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
