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

export function browser_demo_bgm(): string;

export function browser_demo_edit(): void;

export function browser_demo_flags(integer: boolean, monitor: boolean): void;

export function browser_demo_icon(): void;

export function browser_demo_info(): string;

export function browser_demo_load(bytes: Uint8Array | number[]): void;

export function browser_demo_palette(text: string): void;

export function browser_demo_palette_text(): string;

export function browser_demo_reset_palette(): void;

export function browser_demo_resize(w: number, h: number): void;

export function browser_demo_save(): Uint8Array;

export function browser_demo_stop(): void;

export function browser_demo_title(value: string): void;

export function browser_demo_tmx(text: string): void;

/**
 * Manual async frame stepping is available when a callback game loop is not running.
 */
export function browser_flip(): Promise<void>;

export function browser_fullscreen(enabled: boolean): Promise<void>;

export function browser_recorded_frames(): number;

export function browser_recording(seconds: number): void;

export function browser_reset(): void;

export function browser_reset_screencast(): void;

export function browser_screen_mode(mode: number): void;

export function browser_screencast(scale: number): Uint8Array | undefined;

export function browser_screenshot(scale: number): Promise<Uint8Array>;

export function browser_show(): void;

export function browser_text_input(enabled: boolean): void;

export function browser_window_error(): string | undefined;

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

export function screen_demo_cursor(visible: boolean): void;

export function screen_demo_destroy_node(): void;

export function screen_demo_draw(): void;

export function screen_demo_edit(): void;

export function screen_demo_font(): void;

export function screen_demo_info(): string;

export function screen_demo_node_flags(active: boolean, visible: boolean, opacity: number): void;

export function screen_demo_nodes(enabled: boolean): void;

export function screen_demo_pixel(x: number, y: number): number;

export function screen_demo_remove_store(): void;

export function screen_demo_store(value?: string | null): string | undefined;

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

export function start_browser_system(): void;

export function start_cube_example(id: number): void;

export function start_cube_example_resolution(id: number, scale: number): void;

export function start_resources_audio(): void;

export function start_retro3d(): void;

export function start_screen_tools(manual: boolean): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly api_coverage_cursor: (a: number) => void;
    readonly api_coverage_info: () => [number, number];
    readonly browser_audio_status: () => [number, number];
    readonly browser_demo_bgm: () => [number, number, number, number];
    readonly browser_demo_edit: () => [number, number];
    readonly browser_demo_flags: (a: number, b: number) => void;
    readonly browser_demo_icon: () => [number, number];
    readonly browser_demo_info: () => [number, number];
    readonly browser_demo_load: (a: number, b: number) => [number, number];
    readonly browser_demo_palette: (a: number, b: number) => [number, number];
    readonly browser_demo_palette_text: () => [number, number];
    readonly browser_demo_reset_palette: () => void;
    readonly browser_demo_resize: (a: number, b: number) => [number, number];
    readonly browser_demo_save: () => [number, number, number, number];
    readonly browser_demo_stop: () => void;
    readonly browser_demo_title: (a: number, b: number) => void;
    readonly browser_demo_tmx: (a: number, b: number) => [number, number];
    readonly browser_flip: () => any;
    readonly browser_fullscreen: (a: number) => any;
    readonly browser_recorded_frames: () => number;
    readonly browser_recording: (a: number) => [number, number];
    readonly browser_reset: () => [number, number];
    readonly browser_reset_screencast: () => void;
    readonly browser_screen_mode: (a: number) => [number, number];
    readonly browser_screencast: (a: number) => [number, number, number, number];
    readonly browser_screenshot: (a: number) => any;
    readonly browser_show: () => void;
    readonly browser_text_input: (a: number) => [number, number];
    readonly browser_window_error: () => [number, number];
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
    readonly screen_demo_cursor: (a: number) => void;
    readonly screen_demo_destroy_node: () => void;
    readonly screen_demo_draw: () => void;
    readonly screen_demo_edit: () => void;
    readonly screen_demo_font: () => void;
    readonly screen_demo_info: () => [number, number];
    readonly screen_demo_node_flags: (a: number, b: number, c: number) => void;
    readonly screen_demo_nodes: (a: number) => void;
    readonly screen_demo_pixel: (a: number, b: number) => number;
    readonly screen_demo_remove_store: () => [number, number];
    readonly screen_demo_store: (a: number, b: number) => [number, number, number, number];
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
    readonly start_browser_system: () => [number, number];
    readonly start_cube_example: (a: number) => [number, number];
    readonly start_cube_example_resolution: (a: number, b: number) => [number, number];
    readonly start_resources_audio: () => [number, number];
    readonly start_retro3d: () => void;
    readonly start_screen_tools: (a: number) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___f64______true_: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___wasm_bindgen_cf0989641d10091a___JsValue__core_8896a31dc0048bac___result__Result_____wasm_bindgen_cf0989641d10091a___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___core_8896a31dc0048bac___option__Option_web_sys_c32f62750fef9b4___features__gen_Blob__Blob_______true_: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__103: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__104: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__105: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__106: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__107: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__108: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__109: (a: number, b: number, c: any) => void;
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
