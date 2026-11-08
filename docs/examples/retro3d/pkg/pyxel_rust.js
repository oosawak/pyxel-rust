/* @ts-self-types="./pyxel_rust.d.ts" */

/**
 * Chroma subsampling format
 * @enum {0 | 1 | 2 | 3}
 */
export const ChromaSampling = Object.freeze({
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420: 0, "0": "Cs420",
    /**
     * Horizontally subsampled.
     */
    Cs422: 1, "1": "Cs422",
    /**
     * Not subsampled.
     */
    Cs444: 2, "2": "Cs444",
    /**
     * Monochrome.
     */
    Cs400: 3, "3": "Cs400",
});

/**
 * @param {boolean} visible
 */
export function api_coverage_cursor(visible) {
    wasm.api_coverage_cursor(visible);
}

/**
 * @returns {string}
 */
export function api_coverage_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.api_coverage_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {string}
 */
export function browser_audio_status() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.browser_audio_status();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {number | null} [scale]
 * @returns {Uint8Array | undefined}
 */
export function browser_capture_gif(scale) {
    const ret = wasm.browser_capture_gif(isLikeNone(scale) ? Number.MAX_SAFE_INTEGER : (scale) >>> 0);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v1;
    if (ret[0] !== 0) {
        v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
}

/**
 * Use the capture scale configured by init_with_options unless overridden.
 * @param {number | null} [scale]
 * @returns {Promise<Uint8Array>}
 */
export function browser_capture_png(scale) {
    const ret = wasm.browser_capture_png(isLikeNone(scale) ? Number.MAX_SAFE_INTEGER : (scale) >>> 0);
    return ret;
}

/**
 * @returns {string}
 */
export function browser_demo_bgm() {
    let deferred2_0;
    let deferred2_1;
    try {
        const ret = wasm.browser_demo_bgm();
        var ptr1 = ret[0];
        var len1 = ret[1];
        if (ret[3]) {
            ptr1 = 0; len1 = 0;
            throw takeFromExternrefTable0(ret[2]);
        }
        deferred2_0 = ptr1;
        deferred2_1 = len1;
        return getStringFromWasm0(ptr1, len1);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

export function browser_demo_edit() {
    const ret = wasm.browser_demo_edit();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {boolean} integer
 * @param {boolean} monitor
 */
export function browser_demo_flags(integer, monitor) {
    wasm.browser_demo_flags(integer, monitor);
}

export function browser_demo_icon() {
    const ret = wasm.browser_demo_icon();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * Exercise the official filled/outline names in a small exportable image.
 * @param {number} scale
 * @returns {Uint8Array}
 */
export function browser_demo_image_png(scale) {
    const ret = wasm.browser_demo_image_png(scale);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @returns {string}
 */
export function browser_demo_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.browser_demo_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {Uint8Array | number[]} bytes
 */
export function browser_demo_load(bytes) {
    const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.browser_demo_load(ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {string} text
 */
export function browser_demo_palette(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.browser_demo_palette(ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {string}
 */
export function browser_demo_palette_text() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.browser_demo_palette_text();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export function browser_demo_reset_palette() {
    wasm.browser_demo_reset_palette();
}

/**
 * @param {number} w
 * @param {number} h
 */
export function browser_demo_resize(w, h) {
    const ret = wasm.browser_demo_resize(w, h);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {Uint8Array}
 */
export function browser_demo_save() {
    const ret = wasm.browser_demo_save();
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

export function browser_demo_shapes() {
    wasm.browser_demo_shapes();
}

export function browser_demo_stop() {
    wasm.browser_demo_stop();
}

/**
 * @param {string} value
 */
export function browser_demo_title(value) {
    const ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.browser_demo_title(ptr0, len0);
}

/**
 * @param {string} text
 */
export function browser_demo_tmx(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.browser_demo_tmx(ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * Manual async frame stepping is available when a callback game loop is not running.
 * @returns {Promise<void>}
 */
export function browser_flip() {
    const ret = wasm.browser_flip();
    return ret;
}

/**
 * @param {boolean} enabled
 * @returns {Promise<void>}
 */
export function browser_fullscreen(enabled) {
    const ret = wasm.browser_fullscreen(enabled);
    return ret;
}

/**
 * @returns {number}
 */
export function browser_recorded_frames() {
    const ret = wasm.browser_recorded_frames();
    return ret >>> 0;
}

/**
 * @param {number} seconds
 */
export function browser_recording(seconds) {
    const ret = wasm.browser_recording(seconds);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function browser_reset() {
    const ret = wasm.browser_reset();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function browser_reset_screencast() {
    wasm.browser_reset_screencast();
}

/**
 * @param {number} mode
 */
export function browser_screen_mode(mode) {
    const ret = wasm.browser_screen_mode(mode);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} scale
 * @returns {Uint8Array | undefined}
 */
export function browser_screencast(scale) {
    const ret = wasm.browser_screencast(scale);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v1;
    if (ret[0] !== 0) {
        v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
}

/**
 * @param {number} scale
 * @returns {Promise<Uint8Array>}
 */
export function browser_screenshot(scale) {
    const ret = wasm.browser_screenshot(scale);
    return ret;
}

export function browser_show() {
    wasm.browser_show();
}

/**
 * @param {boolean} enabled
 */
export function browser_text_input(enabled) {
    const ret = wasm.browser_text_input(enabled);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {string | undefined}
 */
export function browser_window_error() {
    const ret = wasm.browser_window_error();
    let v1;
    if (ret[0] !== 0) {
        v1 = getStringFromWasm0(ret[0], ret[1]);
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
}

export function clear_retro3d_model() {
    wasm.clear_retro3d_model();
}

/**
 * @returns {string}
 */
export function cube_example_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.cube_example_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {string} json
 */
export function cube_example_input(json) {
    const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.cube_example_input(ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {string}
 */
export function cube_example_sounds() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.cube_example_sounds();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {number}
 */
export function get_bg_idx() {
    const ret = wasm.get_bg_idx();
    return ret;
}

/**
 * @returns {number}
 */
export function get_enemy_flash() {
    const ret = wasm.get_enemy_flash();
    return ret;
}

/**
 * @returns {number}
 */
export function get_enemy_hp() {
    const ret = wasm.get_enemy_hp();
    return ret;
}

/**
 * @returns {number}
 */
export function get_enemy_idx() {
    const ret = wasm.get_enemy_idx();
    return ret;
}

/**
 * @returns {number}
 */
export function get_enemy_max_hp() {
    const ret = wasm.get_enemy_max_hp();
    return ret;
}

/**
 * @returns {number}
 */
export function get_game_state() {
    const ret = wasm.get_game_state();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_flash() {
    const ret = wasm.get_player_flash();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_hp() {
    const ret = wasm.get_player_hp();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_level() {
    const ret = wasm.get_player_level();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_max_hp() {
    const ret = wasm.get_player_max_hp();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_max_mp() {
    const ret = wasm.get_player_max_mp();
    return ret;
}

/**
 * @returns {number}
 */
export function get_player_mp() {
    const ret = wasm.get_player_mp();
    return ret;
}

/**
 * キーインデックスの定数マップを返す (JS から参照用)
 * @param {string} code
 * @returns {number}
 */
export function key_index(code) {
    const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.key_index(ptr0, len0);
    return ret >>> 0;
}

/**
 * Import a self-contained GLB from browser-provided bytes. Failed imports retain
 * the previous model so a malformed selection cannot blank the current scene.
 * @param {Uint8Array | number[]} bytes
 * @returns {number}
 */
export function load_retro3d_glb(bytes) {
    const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.load_retro3d_glb(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0] >>> 0;
}

/**
 * WASM entry point — JS calls `main()` after `await init()` to start the Arisa Quest game.
 */
export function main() {
    wasm.main();
}

/**
 * WASM entry point for Aoba Castle ninja climbing game.
 */
export function main_aobacastle() {
    wasm.main_aobacastle();
}

/**
 * WASM entry point for NanoTerras light-ring game.
 * Separate from main() — arisa state is completely unaffected.
 */
export function main_nanoteras() {
    wasm.main_nanoteras();
}

/**
 * @param {number} actor
 * @param {boolean} active
 */
export function node_motion_demo_active(actor, active) {
    const ret = wasm.node_motion_demo_active(actor, active);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {string}
 */
export function node_motion_demo_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.node_motion_demo_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {number} actor
 * @param {number} clip
 * @param {boolean} looping
 * @param {number} speed
 * @param {number} frame
 */
export function node_motion_demo_play(actor, clip, looping, speed, frame) {
    const ret = wasm.node_motion_demo_play(actor, clip, looping, speed, frame);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} actor
 * @param {number} clip
 * @param {number} frame
 */
export function node_motion_demo_sample(actor, clip, frame) {
    const ret = wasm.node_motion_demo_sample(actor, clip, frame);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} actor
 */
export function node_motion_demo_stop(actor) {
    const ret = wasm.node_motion_demo_stop(actor);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {boolean} paused
 */
export function pause_cube_example(paused) {
    wasm.pause_cube_example(paused);
}

export function reset_retro3d() {
    wasm.reset_retro3d();
}

export function resources_audio_edit() {
    wasm.resources_audio_edit();
}

/**
 * @returns {string}
 */
export function resources_audio_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.resources_audio_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {Uint8Array | number[]} bytes
 */
export function resources_audio_load_png(bytes) {
    const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.resources_audio_load_png(ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} kind
 * @param {boolean} loop_
 * @param {number} start
 */
export function resources_audio_play(kind, loop_, start) {
    const ret = wasm.resources_audio_play(kind, loop_, start);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @returns {Uint8Array}
 */
export function resources_audio_png() {
    const ret = wasm.resources_audio_png();
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * @param {number | null} [channel]
 */
export function resources_audio_stop(channel) {
    wasm.resources_audio_stop(isLikeNone(channel) ? Number.MAX_SAFE_INTEGER : (channel) >>> 0);
}

/**
 * @returns {Uint8Array}
 */
export function resources_audio_wav() {
    const ret = wasm.resources_audio_wav();
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
}

/**
 * Call from a user gesture to satisfy browser audio autoplay policy.
 * @returns {Promise<void>}
 */
export function resume_browser_audio() {
    const ret = wasm.resume_browser_audio();
    return ret;
}

/**
 * @returns {number}
 */
export function retro3d_angle() {
    const ret = wasm.retro3d_angle();
    return ret;
}

/**
 * @returns {number}
 */
export function retro3d_animation_duration() {
    const ret = wasm.retro3d_animation_duration();
    return ret;
}

/**
 * @returns {number}
 */
export function retro3d_animation_index() {
    const ret = wasm.retro3d_animation_index();
    return ret;
}

/**
 * @returns {string}
 */
export function retro3d_animation_names() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.retro3d_animation_names();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @returns {boolean}
 */
export function retro3d_animation_playing() {
    const ret = wasm.retro3d_animation_playing();
    return ret !== 0;
}

/**
 * @returns {number}
 */
export function retro3d_animation_time() {
    const ret = wasm.retro3d_animation_time();
    return ret;
}

/**
 * @returns {boolean}
 */
export function retro3d_auto_rotate() {
    const ret = wasm.retro3d_auto_rotate();
    return ret !== 0;
}

/**
 * @returns {number}
 */
export function retro3d_distance() {
    const ret = wasm.retro3d_distance();
    return ret;
}

/**
 * @returns {boolean}
 */
export function retro3d_has_model() {
    const ret = wasm.retro3d_has_model();
    return ret !== 0;
}

/**
 * @returns {boolean}
 */
export function retro3d_paused() {
    const ret = wasm.retro3d_paused();
    return ret !== 0;
}

/**
 * @param {number} degrees
 */
export function rotate_retro3d(degrees) {
    wasm.rotate_retro3d(degrees);
}

/**
 * @param {boolean} visible
 */
export function screen_demo_cursor(visible) {
    wasm.screen_demo_cursor(visible);
}

export function screen_demo_destroy_node() {
    wasm.screen_demo_destroy_node();
}

export function screen_demo_draw() {
    wasm.screen_demo_draw();
}

export function screen_demo_edit() {
    wasm.screen_demo_edit();
}

export function screen_demo_font() {
    wasm.screen_demo_font();
}

/**
 * @returns {string}
 */
export function screen_demo_info() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.screen_demo_info();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {boolean} active
 * @param {boolean} visible
 * @param {number} opacity
 */
export function screen_demo_node_flags(active, visible, opacity) {
    wasm.screen_demo_node_flags(active, visible, opacity);
}

/**
 * @param {boolean} enabled
 */
export function screen_demo_nodes(enabled) {
    wasm.screen_demo_nodes(enabled);
}

/**
 * Eighteen node-local drawing commands through the same callback API as user code.
 */
export function screen_demo_painter() {
    wasm.screen_demo_painter();
}

/**
 * @param {number} x
 * @param {number} y
 * @returns {number}
 */
export function screen_demo_pixel(x, y) {
    const ret = wasm.screen_demo_pixel(x, y);
    return ret;
}

export function screen_demo_remove_store() {
    const ret = wasm.screen_demo_remove_store();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {string | null} [value]
 * @returns {string | undefined}
 */
export function screen_demo_store(value) {
    var ptr0 = isLikeNone(value) ? 0 : passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    const ret = wasm.screen_demo_store(ptr0, len0);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    let v2;
    if (ret[0] !== 0) {
        v2 = getStringFromWasm0(ret[0], ret[1]);
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v2;
}

/**
 * @param {number} seconds
 */
export function seek_retro3d_animation(seconds) {
    const ret = wasm.seek_retro3d_animation(seconds);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} index
 */
export function select_retro3d_animation(index) {
    const ret = wasm.select_retro3d_animation(index);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * タッチ UI から直接呼べるキー状態セット関数
 * @param {number} idx
 * @param {boolean} down
 */
export function set_key(idx, down) {
    wasm.set_key(idx, down);
}

/**
 * @param {boolean} looping
 */
export function set_retro3d_animation_loop(looping) {
    wasm.set_retro3d_animation_loop(looping);
}

/**
 * @param {boolean} playing
 */
export function set_retro3d_animation_playing(playing) {
    wasm.set_retro3d_animation_playing(playing);
}

/**
 * @param {number} speed
 */
export function set_retro3d_animation_speed(speed) {
    const ret = wasm.set_retro3d_animation_speed(speed);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {boolean} enabled
 */
export function set_retro3d_auto_rotate(enabled) {
    wasm.set_retro3d_auto_rotate(enabled);
}

/**
 * @param {number} distance
 */
export function set_retro3d_distance(distance) {
    wasm.set_retro3d_distance(distance);
}

/**
 * @param {boolean} paused
 */
export function set_retro3d_paused(paused) {
    wasm.set_retro3d_paused(paused);
}

export function start_api_coverage() {
    wasm.start_api_coverage();
}

/**
 * Called from JS to start a battle against enemy at given index.
 * The game loop picks this up next frame and transitions to Battle state.
 * @param {number} idx
 */
export function start_battle_from_js(idx) {
    wasm.start_battle_from_js(idx);
}

export function start_browser_system() {
    const ret = wasm.start_browser_system();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} display_scale
 * @param {number} capture_scale
 * @param {number} capture_sec
 * @param {boolean} headless
 * @param {number} quit_key
 */
export function start_browser_system_with_options(display_scale, capture_scale, capture_sec, headless, quit_key) {
    const ret = wasm.start_browser_system_with_options(display_scale, capture_scale, capture_sec, headless, quit_key);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} id
 */
export function start_cube_example(id) {
    const ret = wasm.start_cube_example(id);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

/**
 * @param {number} id
 * @param {number} scale
 */
export function start_cube_example_resolution(id, scale) {
    const ret = wasm.start_cube_example_resolution(id, scale);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function start_node_motion() {
    const ret = wasm.start_node_motion();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function start_resources_audio() {
    const ret = wasm.start_resources_audio();
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function start_retro3d() {
    wasm.start_retro3d();
}

/**
 * @param {boolean} manual
 */
export function start_screen_tools(manual) {
    wasm.start_screen_tools(manual);
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_208446807d0d1e30: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_debug_string_f249a2a76caa9a55: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_function_741ff2bb612836f5: function(arg0) {
            const ret = typeof(arg0) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_undefined_be5b34249ff1fc15: function(arg0) {
            const ret = arg0 === undefined;
            return ret;
        },
        __wbg___wbindgen_number_get_934b640d0889af78: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64((arg0 >>> 0) + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_throw_4c88d8bf264fd984: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_09736879a3f67559: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_addEventListener_847fd86b8c33ec43: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_appendChild_dbf7ad2f48413560: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.appendChild(arg1);
            return ret;
        }, arguments); },
        __wbg_apply_bfe2b07b146427e2: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.apply(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_arrayBuffer_5abbcb61bd48f427: function(arg0) {
            const ret = arg0.arrayBuffer();
            return ret;
        },
        __wbg_axes_27b88b4eb3f1c9e8: function(arg0) {
            const ret = arg0.axes;
            return ret;
        },
        __wbg_body_5f971cbc6950ca4f: function(arg0) {
            const ret = arg0.body;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_button_653210370515dd71: function(arg0) {
            const ret = arg0.button;
            return ret;
        },
        __wbg_buttons_94a35f2b0c63a35a: function(arg0) {
            const ret = arg0.buttons;
            return ret;
        },
        __wbg_call_40b008ba32f73991: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.call(arg1);
            return ret;
        }, arguments); },
        __wbg_call_a410eb4b59ed5965: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.call(arg1, arg2);
            return ret;
        }, arguments); },
        __wbg_clientWidth_269ea87f17b863bf: function(arg0) {
            const ret = arg0.clientWidth;
            return ret;
        },
        __wbg_clientX_7207eed549b03753: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientY_3371139730cd6f74: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_closest_39824b16065066bc: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.closest(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_code_f0359f3273037430: function(arg0, arg1) {
            const ret = arg1.code;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_connect_744eb2f8329488e7: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.connect(arg1);
            return ret;
        }, arguments); },
        __wbg_connected_26ebb89ef63ebae6: function(arg0) {
            const ret = arg0.connected;
            return ret;
        },
        __wbg_copyToChannel_0544d629fa356aea: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.copyToChannel(getArrayF32FromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_createElement_de4a31ed2fb252be: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
            return ret;
        }, arguments); },
        __wbg_createScriptProcessor_639fec4f841a316c: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg0.createScriptProcessor(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0);
            return ret;
        }, arguments); },
        __wbg_ctrlKey_b7978e4cf78dc62b: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_dataTransfer_3f478a3e477952f6: function(arg0) {
            const ret = arg0.dataTransfer;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_data_fb9a3ebd8ff4b3fd: function(arg0, arg1) {
            const ret = arg1.data;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_deltaX_6ad2483f3f4c0349: function(arg0) {
            const ret = arg0.deltaX;
            return ret;
        },
        __wbg_deltaY_37dc0a54748a91f4: function(arg0) {
            const ret = arg0.deltaY;
            return ret;
        },
        __wbg_destination_9a4d8f898e5c0a0b: function(arg0) {
            const ret = arg0.destination;
            return ret;
        },
        __wbg_document_3b5c3071b43e97ee: function(arg0) {
            const ret = arg0.document;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_drawImage_47077eeae8f1a73d: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            arg0.drawImage(arg1, arg2, arg3, arg4, arg5);
        }, arguments); },
        __wbg_drawImage_f6dcb1e2c24e0cd2: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
            arg0.drawImage(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
        }, arguments); },
        __wbg_files_2d8b9138ca028e2c: function(arg0) {
            const ret = arg0.files;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_fillRect_e1f0b9915220348b: function(arg0, arg1, arg2, arg3, arg4) {
            arg0.fillRect(arg1, arg2, arg3, arg4);
        },
        __wbg_fillText_89d353a9ad9f4a1a: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments); },
        __wbg_focus_d5e2fdbdb56416f0: function() { return handleError(function (arg0) {
            arg0.focus();
        }, arguments); },
        __wbg_fullscreenElement_ecd845ac09ed38dc: function(arg0) {
            const ret = arg0.fullscreenElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getBoundingClientRect_329ca748ec918b0c: function(arg0) {
            const ret = arg0.getBoundingClientRect();
            return ret;
        },
        __wbg_getContext_d93f904b714852ae: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_getElementById_7ee213db584f37b0: function(arg0, arg1, arg2) {
            const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getGamepads_5be880521b3ecfaa: function() { return handleError(function (arg0) {
            const ret = arg0.getGamepads();
            return ret;
        }, arguments); },
        __wbg_getItem_49556a3e59cfb688: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getItem(getStringFromWasm0(arg2, arg3));
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getPropertyValue_c0aae8b8341401f1: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getPropertyValue(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getRandomValues_8b6f1f663e6f188d: function() { return handleError(function (arg0, arg1) {
            globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
        }, arguments); },
        __wbg_get_2080c49015e35254: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(arg0, arg1);
            return ret;
        }, arguments); },
        __wbg_get_955bde2b2fe2defa: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_get_d7176cfb76d71114: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_height_ce5f2b5901ba05ab: function(arg0) {
            const ret = arg0.height;
            return ret;
        },
        __wbg_id_2aec05cae0b5abeb: function(arg0, arg1) {
            const ret = arg1.id;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_instanceof_Blob_56431c8f5387c12d: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Blob;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_CanvasRenderingContext2d_72a60be8aaadecab: function(arg0) {
            let result;
            try {
                result = arg0 instanceof CanvasRenderingContext2D;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Element_05dbb22ded594343: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Element;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_GamepadButton_db22ff43074e1ad4: function(arg0) {
            let result;
            try {
                result = arg0 instanceof GamepadButton;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Gamepad_7900b439501ad012: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Gamepad;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlCanvasElement_89f280683e987b9e: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLCanvasElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_HtmlTextAreaElement_37c6efee878106d5: function(arg0) {
            let result;
            try {
                result = arg0 instanceof HTMLTextAreaElement;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Window_5cf453f5639aa1a2: function(arg0) {
            let result;
            try {
                result = arg0 instanceof Window;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isComposing_4b9450ae90c4e806: function(arg0) {
            const ret = arg0.isComposing;
            return ret;
        },
        __wbg_isComposing_b1e74068b6ff27e2: function(arg0) {
            const ret = arg0.isComposing;
            return ret;
        },
        __wbg_key_76125a528069feff: function(arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_left_9df47701ff317a35: function(arg0) {
            const ret = arg0.left;
            return ret;
        },
        __wbg_length_186163212ffe79c3: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_length_694310723930d77e: function(arg0) {
            const ret = arg0.length;
            return ret;
        },
        __wbg_localStorage_6827cfe6db14a9f6: function() { return handleError(function (arg0) {
            const ret = arg0.localStorage;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_location_7320782274fc691b: function(arg0) {
            const ret = arg0.location;
            return ret;
        },
        __wbg_mapping_6c3a7915decb8fdd: function(arg0) {
            const ret = arg0.mapping;
            return (__wbindgen_enum_GamepadMappingType.indexOf(ret) + 1 || 3) - 1;
        },
        __wbg_metaKey_33c588e07fd74724: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_name_f721590be23206c9: function(arg0, arg1) {
            const ret = arg1.name;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_navigator_c17f443a6fe5d38d: function(arg0) {
            const ret = arg0.navigator;
            return ret;
        },
        __wbg_new_13de428334042d26: function(arg0) {
            const ret = new Uint8Array(arg0);
            return ret;
        },
        __wbg_new_37eb45aaf5eeb820: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_ba40181b3f24ac80: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_new_cd4348290afae586: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_typed_e09da9dae6ee5935: function(arg0, arg1) {
            try {
                var state0 = {a: arg0, b: arg1};
                var cb0 = (arg0, arg1) => {
                    const a = state0.a;
                    state0.a = 0;
                    try {
                        return wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_(a, state0.b, arg0, arg1);
                    } finally {
                        state0.a = a;
                    }
                };
                const ret = new Promise(cb0);
                return ret;
            } finally {
                state0.a = 0;
            }
        },
        __wbg_new_with_context_options_0af40546f3ba1d87: function() { return handleError(function (arg0) {
            const ret = new lAudioContext(arg0);
            return ret;
        }, arguments); },
        __wbg_new_with_u8_clamped_array_and_sh_e5d757b60792d996: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0, arg3 >>> 0);
            return ret;
        }, arguments); },
        __wbg_now_043d245551b22f9c: function(arg0) {
            const ret = arg0.now();
            return ret;
        },
        __wbg_outputBuffer_4e55504e23bc679c: function() { return handleError(function (arg0) {
            const ret = arg0.outputBuffer;
            return ret;
        }, arguments); },
        __wbg_parentElement_300cdd39ce1d95f8: function(arg0) {
            const ret = arg0.parentElement;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_performance_50e6a3f8dc80bbca: function(arg0) {
            const ret = arg0.performance;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_pressed_5508f4e84425b2ee: function(arg0) {
            const ret = arg0.pressed;
            return ret;
        },
        __wbg_preventDefault_cfb06b02e7406899: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_prototypesetcall_13162965d1fe7aa3: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
        },
        __wbg_push_c9f4893fc209f768: function(arg0, arg1) {
            const ret = arg0.push(arg1);
            return ret;
        },
        __wbg_putImageData_4c8f5c533216d0a0: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.putImageData(arg1, arg2, arg3);
        }, arguments); },
        __wbg_querySelector_72201b7c62dc196b: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        }, arguments); },
        __wbg_queueMicrotask_1e8503a8a690f0a3: function(arg0) {
            const ret = arg0.queueMicrotask;
            return ret;
        },
        __wbg_queueMicrotask_4a1a4d8dd332373b: function(arg0) {
            queueMicrotask(arg0);
        },
        __wbg_reload_a46804ad57f539bf: function() { return handleError(function (arg0) {
            arg0.reload();
        }, arguments); },
        __wbg_removeItem_6db621e167d16a1c: function() { return handleError(function (arg0, arg1, arg2) {
            arg0.removeItem(getStringFromWasm0(arg1, arg2));
        }, arguments); },
        __wbg_remove_8228e91d7ddbc755: function(arg0) {
            arg0.remove();
        },
        __wbg_requestAnimationFrame_9ba49d36a19ca83c: function() { return handleError(function (arg0, arg1) {
            const ret = arg0.requestAnimationFrame(arg1);
            return ret;
        }, arguments); },
        __wbg_resolve_dbf78ab4942478b0: function(arg0) {
            const ret = Promise.resolve(arg0);
            return ret;
        },
        __wbg_resume_eb9b8678e3c9d27b: function() { return handleError(function (arg0) {
            const ret = arg0.resume();
            return ret;
        }, arguments); },
        __wbg_sampleRate_239ec0f08f69ef0b: function(arg0) {
            const ret = arg0.sampleRate;
            return ret;
        },
        __wbg_setAttribute_f75317b2b5cabfb4: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setItem_cfa3cb31f271776a: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setProperty_3fe74e09d53acb0b: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_set_fillStyle_95fde67582a9c695: function(arg0, arg1, arg2) {
            arg0.fillStyle = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_fillStyle_f2819d67645a21ba: function(arg0, arg1) {
            arg0.fillStyle = arg1;
        },
        __wbg_set_font_aa0c0c514c73a5fa: function(arg0, arg1, arg2) {
            arg0.font = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_height_ae0610d318c3428c: function(arg0, arg1) {
            arg0.height = arg1 >>> 0;
        },
        __wbg_set_id_861bcd7afe9f86ad: function(arg0, arg1, arg2) {
            arg0.id = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_imageSmoothingEnabled_80eb6ec9c323bf88: function(arg0, arg1) {
            arg0.imageSmoothingEnabled = arg1 !== 0;
        },
        __wbg_set_onaudioprocess_7d17887bca7c32fa: function(arg0, arg1) {
            arg0.onaudioprocess = arg1;
        },
        __wbg_set_sample_rate_a688c2e32f20133b: function(arg0, arg1) {
            arg0.sampleRate = arg1;
        },
        __wbg_set_tabIndex_38e2c0c2e76c66c5: function(arg0, arg1) {
            arg0.tabIndex = arg1;
        },
        __wbg_set_textBaseline_ff2e3c5c84684b23: function(arg0, arg1, arg2) {
            arg0.textBaseline = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_textContent_0177b2a6d9293c2c: function(arg0, arg1, arg2) {
            arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_title_7c9f7c08b10ea16a: function(arg0, arg1, arg2) {
            arg0.title = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_value_651c5a714f0a7b56: function(arg0, arg1, arg2) {
            arg0.value = getStringFromWasm0(arg1, arg2);
        },
        __wbg_set_width_5f89a551b5472887: function(arg0, arg1) {
            arg0.width = arg1 >>> 0;
        },
        __wbg_state_db98deb9dd250b27: function(arg0) {
            const ret = arg0.state;
            return (__wbindgen_enum_AudioContextState.indexOf(ret) + 1 || 4) - 1;
        },
        __wbg_static_accessor_GLOBAL_THIS_003ee833af9699fc: function() {
            const ret = typeof globalThis === 'undefined' ? undefined : globalThis;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_GLOBAL_e4293fb22cbe77da: function() {
            const ret = typeof global === 'undefined' ? undefined : global;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_SELF_53bb20457401453d: function() {
            const ret = typeof self === 'undefined' ? undefined : self;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_static_accessor_WINDOW_b5e0810bfecb7feb: function() {
            const ret = typeof window === 'undefined' ? undefined : window;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_style_f499e599ac70277f: function(arg0) {
            const ret = arg0.style;
            return ret;
        },
        __wbg_target_e5cd9c7aa87dd8df: function(arg0) {
            const ret = arg0.target;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_then_656f253a5dd7d8ea: function(arg0, arg1, arg2) {
            const ret = arg0.then(arg1, arg2);
            return ret;
        },
        __wbg_then_e3f7f2d365708be8: function(arg0, arg1) {
            const ret = arg0.then(arg1);
            return ret;
        },
        __wbg_toBlob_1e5caf520e53b922: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.toBlob(arg1, getStringFromWasm0(arg2, arg3));
        }, arguments); },
        __wbg_toDataURL_e02e097933c73a95: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.toDataURL(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_top_2d45cda4e060d8bf: function(arg0) {
            const ret = arg0.top;
            return ret;
        },
        __wbg_value_6c3df9cc1c6b6185: function(arg0) {
            const ret = arg0.value;
            return ret;
        },
        __wbg_value_f61036dc298faa36: function(arg0, arg1) {
            const ret = arg1.value;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 1, len1, true);
            getDataViewMemory0().setInt32((arg0 >>> 0) + 4 * 0, ptr1, true);
        },
        __wbg_width_d3c8d417d821d2fd: function(arg0) {
            const ret = arg0.width;
            return ret;
        },
        __wbindgen_generic_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Externref], shim_idx: 1477, ret: Result(Unit), inner_ret: Some(Result(Unit)) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___wasm_bindgen_cf0989641d10091a___JsValue__core_8896a31dc0048bac___result__Result_____wasm_bindgen_cf0989641d10091a___JsError___true_);
            return ret;
        },
        __wbindgen_generic_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [F64], shim_idx: 108, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___f64______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("AudioProcessingEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true_);
            return ret;
        },
        __wbindgen_generic_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("CompositionEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__115);
            return ret;
        },
        __wbindgen_generic_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("DragEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__116);
            return ret;
        },
        __wbindgen_generic_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__117);
            return ret;
        },
        __wbindgen_generic_0000000000000007: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("InputEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__118);
            return ret;
        },
        __wbindgen_generic_0000000000000008: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__119);
            return ret;
        },
        __wbindgen_generic_0000000000000009: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("MouseEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__120);
            return ret;
        },
        __wbindgen_generic_000000000000000a: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("WheelEvent")], shim_idx: 107, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__121);
            return ret;
        },
        __wbindgen_generic_000000000000000b: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [Option(NamedExternref("Blob"))], shim_idx: 106, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___core_8896a31dc0048bac___option__Option_web_sys_c32f62750fef9b4___features__gen_Blob__Blob_______true_);
            return ret;
        },
        __wbindgen_generic_000000000000000c: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [], shim_idx: 105, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen_cf0989641d10091a___convert__closures_____invoke_______true_);
            return ret;
        },
        __wbindgen_generic_000000000000000d: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_generic_000000000000000e: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_generic_000000000000000f: function(arg0, arg1) {
            var v0 = getArrayU8FromWasm0(arg0, arg1).slice();
            wasm.__wbindgen_free(arg0, arg1 * 1, 1);
            // Cast intrinsic for `Vector(U8) -> Externref`.
            const ret = v0;
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            table.set(0, undefined);
            table.set(1024, undefined);
            table.set(1025, null);
            table.set(1026, true);
            table.set(1027, false);
        },
    };
    return {
        __proto__: null,
        "./pyxel_rust_bg.js": import0,
    };
}

const lAudioContext = (typeof AudioContext !== 'undefined' ? AudioContext : (typeof webkitAudioContext !== 'undefined' ? webkitAudioContext : undefined));
function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke_______true_(arg0, arg1) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke_______true_(arg0, arg1);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true_(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__115(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__115(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__116(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__116(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__117(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__117(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__118(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__118(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__119(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__119(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__120(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__120(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__121(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___web_sys_c32f62750fef9b4___features__gen_DragEvent__DragEvent______true__121(arg0, arg1, arg2);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___wasm_bindgen_cf0989641d10091a___JsValue__core_8896a31dc0048bac___result__Result_____wasm_bindgen_cf0989641d10091a___JsError___true_(arg0, arg1, arg2) {
    const ret = wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___wasm_bindgen_cf0989641d10091a___JsValue__core_8896a31dc0048bac___result__Result_____wasm_bindgen_cf0989641d10091a___JsError___true_(arg0, arg1, arg2);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined___js_sys_e23d1d6365e9eae___Function_fn_wasm_bindgen_cf0989641d10091a___JsValue_____wasm_bindgen_cf0989641d10091a___sys__Undefined_______true_(arg0, arg1, arg2, arg3);
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___core_8896a31dc0048bac___option__Option_web_sys_c32f62750fef9b4___features__gen_Blob__Blob_______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___core_8896a31dc0048bac___option__Option_web_sys_c32f62750fef9b4___features__gen_Blob__Blob_______true_(arg0, arg1, isLikeNone(arg2) ? 0 : addToExternrefTable0(arg2));
}

function wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___f64______true_(arg0, arg1, arg2) {
    wasm.wasm_bindgen_cf0989641d10091a___convert__closures_____invoke___f64______true_(arg0, arg1, arg2);
}


const __wbindgen_enum_AudioContextState = ["suspended", "running", "closed"];


const __wbindgen_enum_GamepadMappingType = ["", "standard"];

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

function debugString(val) {
    switch (typeof val) {
        // primitive types
        case 'number':
        case 'boolean':
        case 'undefined':
        return `${val}`;
        case 'string':
        return JSON.stringify(val);
        case 'bigint':
        return `${val}n`;
        case 'symbol':
        return val.description !== undefined
            ? `Symbol(${val.description})`
            : 'Symbol';
        case 'function':
        return typeof val.name === 'string' && val.name !== ''
            ? `Function(${val.name})`
            : 'Function';
        case 'object': {
            if (val === null) {
                return 'null';
            }
            // objects
            if (Array.isArray(val)) {
                return '[' + Array.from(val, debugString).join(', ') + ']';
            }
            // errors
            if (val instanceof Error) {
                return `${val.name}: ${val.message}\n${val.stack}`;
            }

            const toStringOutput = toString.call(val);
            // Test for built-in
            const className = /\[object ([^\]]+)\]/.exec(toStringOutput)?.[1];

            // Failed to match the standard '[object ClassName]'
            if (className === undefined) {
                return toStringOutput;
            }
            if (className === 'Object') {
                // we're a user defined class or Object
                // JSON.stringify avoids problems with cycles, and is generally much
                // easier than looping through ownProperties of `val`.
                try {
                    return 'Object(' + JSON.stringify(val) + ')';
                } catch (_) {
                    return 'Object';
                }
            }
            // TODO we could test for more things here, like `Set`s and `Map`s.
            return className;
        }
        // Per MDN: 'This list of values is exhaustive.'
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/typeof#description
    }
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getClampedArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ClampedArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedUint8ClampedArrayMemory0 = null;
function getUint8ClampedArrayMemory0() {
    if (cachedUint8ClampedArrayMemory0 === null || cachedUint8ClampedArrayMemory0.byteLength === 0) {
        cachedUint8ClampedArrayMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
    }
    return cachedUint8ClampedArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(bytes) {
    numBytesDecoded += bytes.length;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true });
        cachedTextDecoder.decode();
        numBytesDecoded = bytes.length;
    }
    return cachedTextDecoder.decode(bytes);
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    cachedUint8ClampedArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

let __wbg_init_promise;
async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;
    if (__wbg_init_promise === undefined) {
        __wbg_init_promise = __wbg_init_once(module_or_path);
        __wbg_init_promise.catch(() => { __wbg_init_promise = undefined; });
    }
    return __wbg_init_promise;
}

async function __wbg_init_once(module_or_path) {

    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('pyxel_rust_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
