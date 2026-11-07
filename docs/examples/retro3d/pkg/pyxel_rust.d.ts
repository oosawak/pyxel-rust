/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

export function api_coverage_cursor(visible: boolean): void;

export function api_coverage_info(): string;

export function browser_audio_status(): string;

export function clear_retro3d_model(): void;

export function cube_example_info(): string;

export function cube_example_input(json: string): void;

export function cube_example_sounds(): string;

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
 * Import a self-contained GLB from browser-provided bytes. Failed imports retain
 * the previous model so a malformed selection cannot blank the current scene.
 */
export function load_retro3d_glb(bytes: Uint8Array | number[]): number;

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

export function pause_cube_example(paused: boolean): void;

export function reset_retro3d(): void;

export function resources_audio_edit(): void;

export function resources_audio_info(): string;

export function resources_audio_load_png(bytes: Uint8Array | number[]): void;

export function resources_audio_play(kind: number, loop_: boolean, start: number): void;

export function resources_audio_png(): Uint8Array;

export function resources_audio_stop(channel?: number | null): void;

export function resources_audio_wav(): Uint8Array;

/**
 * Call from a user gesture to satisfy browser audio autoplay policy.
 */
export function resume_browser_audio(): Promise<void>;

export function retro3d_angle(): number;

export function retro3d_animation_duration(): number;

export function retro3d_animation_index(): number;

export function retro3d_animation_names(): string;

export function retro3d_animation_playing(): boolean;

export function retro3d_animation_time(): number;

export function retro3d_auto_rotate(): boolean;

export function retro3d_distance(): number;

export function retro3d_has_model(): boolean;

export function retro3d_paused(): boolean;

export function rotate_retro3d(degrees: number): void;

export function seek_retro3d_animation(seconds: number): void;

export function select_retro3d_animation(index: number): void;

/**
 * タッチ UI から直接呼べるキー状態セット関数
 */
export function set_key(idx: number, down: boolean): void;

export function set_retro3d_animation_loop(looping: boolean): void;

export function set_retro3d_animation_playing(playing: boolean): void;

export function set_retro3d_animation_speed(speed: number): void;

export function set_retro3d_auto_rotate(enabled: boolean): void;

export function set_retro3d_distance(distance: number): void;

export function set_retro3d_paused(paused: boolean): void;

export function start_api_coverage(): void;

/**
 * Called from JS to start a battle against enemy at given index.
 * The game loop picks this up next frame and transitions to Battle state.
 */
export function start_battle_from_js(idx: number): void;

export function start_cube_example(id: number): void;

export function start_cube_example_resolution(id: number, scale: number): void;

export function start_resources_audio(): void;

export function start_retro3d(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly api_coverage_cursor: (a: number) => void;
    readonly api_coverage_info: () => [number, number];
    readonly browser_audio_status: () => [number, number];
    readonly clear_retro3d_model: () => void;
    readonly cube_example_info: () => [number, number];
    readonly cube_example_input: (a: number, b: number) => [number, number];
    readonly cube_example_sounds: () => [number, number];
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
    readonly load_retro3d_glb: (a: number, b: number) => [number, number, number];
    readonly main: () => void;
    readonly main_aobacastle: () => void;
    readonly main_nanoteras: () => void;
    readonly pause_cube_example: (a: number) => void;
    readonly reset_retro3d: () => void;
    readonly resources_audio_edit: () => void;
    readonly resources_audio_info: () => [number, number];
    readonly resources_audio_load_png: (a: number, b: number) => [number, number];
    readonly resources_audio_play: (a: number, b: number, c: number) => [number, number];
    readonly resources_audio_png: () => [number, number, number, number];
    readonly resources_audio_stop: (a: number) => void;
    readonly resources_audio_wav: () => [number, number, number, number];
    readonly resume_browser_audio: () => any;
    readonly retro3d_angle: () => number;
    readonly retro3d_animation_duration: () => number;
    readonly retro3d_animation_index: () => number;
    readonly retro3d_animation_names: () => [number, number];
    readonly retro3d_animation_playing: () => number;
    readonly retro3d_animation_time: () => number;
    readonly retro3d_auto_rotate: () => number;
    readonly retro3d_distance: () => number;
    readonly retro3d_has_model: () => number;
    readonly retro3d_paused: () => number;
    readonly rotate_retro3d: (a: number) => void;
    readonly seek_retro3d_animation: (a: number) => [number, number];
    readonly select_retro3d_animation: (a: number) => [number, number];
    readonly set_key: (a: number, b: number) => void;
    readonly set_retro3d_animation_loop: (a: number) => void;
    readonly set_retro3d_animation_playing: (a: number) => void;
    readonly set_retro3d_animation_speed: (a: number) => [number, number];
    readonly set_retro3d_auto_rotate: (a: number) => void;
    readonly set_retro3d_distance: (a: number) => void;
    readonly set_retro3d_paused: (a: number) => void;
    readonly start_api_coverage: () => void;
    readonly start_battle_from_js: (a: number) => void;
    readonly start_cube_example: (a: number) => [number, number];
    readonly start_cube_example_resolution: (a: number, b: number) => [number, number];
    readonly start_resources_audio: () => [number, number];
    readonly start_retro3d: () => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___wasm_bindgen_cf0989641d10091a___JsValue__core_8896a31dc0048bac___result__Result_____wasm_bindgen_cf0989641d10091a___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_bad5fcda91e6a9fc___features__gen_MouseEvent__MouseEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_bad5fcda91e6a9fc___features__gen_MouseEvent__MouseEvent______true__63: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_bad5fcda91e6a9fc___features__gen_MouseEvent__MouseEvent______true__64: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_bad5fcda91e6a9fc___features__gen_MouseEvent__MouseEvent______true__65: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_bad5fcda91e6a9fc___features__gen_MouseEvent__MouseEvent______true__66: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
