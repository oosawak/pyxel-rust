/* tslint:disable */
/* eslint-disable */

export function sendai_daikannon_add_attack(power: number): void;

export function sendai_daikannon_get_enemy_hp(): number;

export function sendai_daikannon_get_player_hp(): number;

export function sendai_daikannon_get_state(): number;

export function sendai_daikannon_init(player_hp: number): void;

export function sendai_daikannon_reset(player_hp: number): void;

export function sendai_daikannon_update(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly sendai_daikannon_add_attack: (a: number) => void;
    readonly sendai_daikannon_get_enemy_hp: () => number;
    readonly sendai_daikannon_get_player_hp: () => number;
    readonly sendai_daikannon_get_state: () => number;
    readonly sendai_daikannon_init: (a: number) => void;
    readonly sendai_daikannon_update: () => void;
    readonly sendai_daikannon_reset: (a: number) => void;
    readonly __wbindgen_externrefs: WebAssembly.Table;
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
