/* tslint:disable */
/* eslint-disable */

/**
 * A loaded Gemma 4 model with all GPU resources allocated. One `Model` corresponds to
 * one conversation: it owns the KV cache and tracks the current position.
 *
 * Internally a `Model` is a tokenizer + a [`Forward`] + a [`Sampler`]. `Forward` runs
 * one wgpu CommandEncoder per token (M7 work) — significantly faster than the original
 * per-kernel-readback path, which is now retained only as a parity oracle.
 */
export class Model {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * `[<|audio> token id, <audio|> token id]` if both sentinels exist; else `null`.
     */
    audioSentinelIds(): Uint32Array | undefined;
    /**
     * Cooperatively cancel an in-flight `encodeImage` / `encodeAudio`. The
     * in-flight `Promise` rejects with a "cancelled" error on the next
     * transformer-layer boundary (≤500 ms in practice). Safe to call when
     * no encode is running — the flag is cleared at the start of the
     * next encode regardless.
     */
    cancelMultimodalEncode(): void;
    /**
     * Drop the active adapter.
     */
    clearAdapter(): void;
    /**
     * Decode WAV file bytes into 16 kHz mono Float32Array. Convenience for JS
     * callers that have a WAV file but don't want to plumb Web Audio.
     */
    static decodeWav(bytes: Uint8Array): Float32Array;
    encode(text: string): Uint32Array;
    /**
     * Encode raw 16 kHz mono PCM (Float32Array in `[-1, 1]`) into a
     * Float32Array of soft-token embeddings. Caller is responsible for
     * resampling to 16 kHz if the source is at a different rate.
     */
    encodeAudio(pcm: Float32Array): Promise<Float32Array>;
    /**
     * Encode an RGB image into a `Float32Array` of soft-token embeddings, flat
     * `[n_pooled_patches × d_text]`. JS pass-in: `pixels` is the image in
     * channel-first `[R..., G..., B...]` order normalised to `[-1, 1]`; `h`,
     * `w` are integer pixel dims aligned to `patch_size * n_merge` (= 48).
     */
    encodeImage(pixels: Float32Array, h: number, w: number, progress_cb?: Function | null): Promise<Float32Array>;
    /**
     * `[<|image> token id, <image|> token id]` if both sentinels exist in the
     * vocab, else `null`. Used by the JS chat handler to splice soft-token
     * embeddings between the markers in the encoded prompt.
     */
    imageSentinelIds(): Uint32Array | undefined;
    /**
     * Number of soft tokens an `h × w` image will produce, or `null` if either
     * dimension is misaligned.
     */
    imageSoftTokenCount(h: number, w: number): number | undefined;
    isEos(id: number): boolean;
    /**
     * JS entry point: build a Model from raw GGUF bytes (e.g. a `Uint8Array` from
     * `fetch().then(r => r.arrayBuffer())`). Holds the entire GGUF in wasm linear
     * memory; only suitable for files that fit under the 4 GB wasm32 cap.
     */
    static load(bytes: Uint8Array): Promise<Model>;
    /**
     * Load a safetensors LoRA adapter from raw bytes (e.g. the
     * `Uint8Array` returned by `TrainingSession.saveAdapter`).
     * Returns the number of LoRA slots loaded.
     */
    loadAdapter(bytes: Uint8Array): number;
    /**
     * JS entry point: stream the GGUF from a file the host has already saved to
     * OPFS (Origin Private File System). `read_fn` is a JS callback with signature
     * `(offset_f64, len_f64) -> Promise<Uint8Array> | Uint8Array`. `total_bytes`
     * is the file's full size (caller knows this from the OPFS file handle).
     *
     * This is the path that bypasses iOS Safari's ~5.6 GiB single-Blob cap and
     * ~2 GiB live-JS-heap cap — bytes are read directly from the disk-backed
     * OPFS file in slices and never aggregate in JS memory.
     * JS entry point: stream the GGUF from an OPFS-resident file with
     * vision + audio towers built. Optional `max_context` caps the KV
     * pre-allocation; pass 0 to use the compile-time `MAX_CONTEXT`
     * (4096). On iPhone, supplying 2048 saves ~600 MB of KV buffer
     * against the multimodal weight budget.
     */
    static loadFromOpfs(read_fn: Function, total_bytes: number, max_context: number): Promise<Model>;
    /**
     * JS entry point: text-only variant of [`loadFromOpfs`]. Skips vision and
     * audio tower construction AND caps the KV cache at `max_context` tokens
     * (default 512 if `max_context` is 0 or absent) so the wasm-load
     * footprint stays small enough to fit a Q4_K_M `gemma4:e2b` in
     * iPhone-class shared RAM (8 GB). `encode_image` / `encode_audio` will
     * fail with "this checkpoint has no vision/audio tower" — text
     * inference and chat work as normal.
     */
    static loadFromOpfsTextOnly(read_fn: Function, total_bytes: number, max_context: number): Promise<Model>;
    /**
     * JS entry point: stream the GGUF over HTTP via byte-range requests. The full
     * file never lands in wasm memory — tensors are fetched on demand and dropped
     * after each GPU upload. This is the path that lets `gemma4:e2b` (~7 GB) load
     * in the browser despite wasm32's 4 GB linear-memory cap.
     *
     * Requires the server to support `Range: bytes=N-M` and to expose either
     * `Content-Range` or `X-Total-Size` so the client can discover the file length.
     */
    static loadFromUrl(url: string): Promise<Model>;
    /**
     * Evict cached audio-tower weights from GPU memory.
     */
    releaseAudioWeights(): number;
    /**
     * Evict cached vision-tower weights from GPU memory. Returns the number
     * of cache entries freed. Call between turns on iPhone when the next
     * message won't include an image to free ~3 GB.
     */
    releaseVisionWeights(): number;
    /**
     * Render a single user message (and optional system message) into the Gemma 4
     * chat-template prompt. JS callers pass `[{role, content}, ...]` as JSON.
     */
    renderChat(messages_json: any, with_bos: boolean): string;
    /**
     * Like [`renderChat`] but leaves a trailing assistant turn OPEN if
     * the last message has `role: "model"`. Used by suspend/resume to
     * rebuild KV cache from a conversation that includes a partial
     * assistant response.
     */
    renderChatForContinuation(messages_json: any, with_bos: boolean): string;
    reset(): void;
    /**
     * Inverse of [`saveKvState`]. Validates the snapshot against the
     * currently-loaded model (layout hash) and refuses to apply if it's
     * from a different model architecture — caller should fall back to
     * token-replay rebuild in that case.
     */
    restoreKvState(bytes: Uint8Array): void;
    /**
     * Snapshot KV cache + sampler state into a single Uint8Array. Caller
     * writes the result to OPFS / IndexedDB for suspend/resume.
     */
    saveKvState(): Promise<Uint8Array>;
    /**
     * Configure sampling from a JSON-shape `{temperature, top_k, top_p, repetition_penalty, seed}`.
     * JS callers pass an object; serde decodes it.
     */
    setSampling(opts_json: any): void;
    /**
     * Re-allocate the per-layer KV cache at a new capacity (tokens).
     * Returns the previous max_context so JS can restore later. See
     * `shrink_kv_native` for the full rationale.
     */
    shrinkKv(new_max_context: number): number;
    /**
     * Feed one token, advance pos, return sampled next token id.
     */
    step(token_id: number): Promise<number>;
    /**
     * Feed one pre-computed embedding (e.g. one soft-token row from
     * `encodeImage`), advance pos, return sampled next token id. JS pass-in is a
     * `Float32Array` of length `d_model` (1536 for gemma4:e2b).
     */
    stepWithEmbedding(embedding: Float32Array): Promise<number>;
    tokenStr(id: number): string | undefined;
    /**
     * Total bytes currently held in the shared GPU weight cache.
     */
    readonly cachedWeightBytes: bigint;
    /**
     * True iff a LoRA adapter is currently active.
     */
    readonly hasAdapter: boolean;
    /**
     * True iff this checkpoint carries an audio tower.
     */
    readonly hasAudio: boolean;
    /**
     * True iff this checkpoint carries a vision tower (gemma4:e2b/e4b).
     */
    readonly hasVision: boolean;
    /**
     * Current KV cache capacity (tokens). Snapshot this before
     * `shrinkKv()` and pass it back on `trainingFinish` to restore.
     */
    readonly maxContext: number;
    readonly position: number;
    readonly vocabSize: number;
}

export class WasmDatabase {
    free(): void;
    [Symbol.dispose](): void;
    close(): void;
    /**
     * Register a JavaScript callback as a SQL scalar function.
     *
     * The callback receives the evaluated arguments as JS values and must
     * return synchronously (async callbacks are deferred to a later
     * release). Pass `n_args = -1` for variadic.
     *
     * User-defined functions cannot shadow built-ins — the engine resolves
     * known names (`UPPER`, `JSON_EXTRACT`, `vec_distance_cosine`, …) before
     * consulting the UDF registry.
     */
    createFunction(name: string, n_args: number, callback: Function): void;
    /**
     * Remove a previously-registered user-defined function. Returns true if
     * a function by that name existed.
     */
    deleteFunction(name: string): boolean;
    exec(sql: string): bigint;
    execMany(sql: string): void;
    execParams(sql: string, params: any): bigint;
    flush(): void;
    static fromBuffer(data: Uint8Array): WasmDatabase;
    constructor();
    static openInMemory(): WasmDatabase;
    static openPersisted(name: string, chunk_size?: bigint | null, max_shards?: number | null): Promise<WasmDatabase>;
    static openWithIdb(name: string, chunk_size?: bigint | null): Promise<WasmDatabase>;
    static openWithOpfs(name: string, chunk_size?: bigint | null, max_shards?: number | null): Promise<WasmDatabase>;
    query(sql: string): any;
    queryOne(sql: string): any;
    queryParams(sql: string, params: any): any;
    toBuffer(): Uint8Array;
}

export function __wasm_start(): void;

/**
 * M0 smoke export: doubles every f32 on the GPU. Useful from JS to confirm WebGPU
 * is wired up before loading the full model.
 */
export function computeSpike(input: Float32Array): Promise<Float32Array>;

export function init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_model_free: (a: number, b: number) => void;
    readonly computeSpike: (a: number, b: number) => any;
    readonly model_audioSentinelIds: (a: number) => [number, number];
    readonly model_cachedWeightBytes: (a: number) => bigint;
    readonly model_cancelMultimodalEncode: (a: number) => void;
    readonly model_clearAdapter: (a: number) => void;
    readonly model_decodeWav: (a: number, b: number) => [number, number, number, number];
    readonly model_encode: (a: number, b: number, c: number) => [number, number];
    readonly model_encodeAudio: (a: number, b: number, c: number) => any;
    readonly model_encodeImage: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly model_hasAdapter: (a: number) => number;
    readonly model_hasAudio: (a: number) => number;
    readonly model_hasVision: (a: number) => number;
    readonly model_imageSentinelIds: (a: number) => [number, number];
    readonly model_imageSoftTokenCount: (a: number, b: number, c: number) => number;
    readonly model_isEos: (a: number, b: number) => number;
    readonly model_load: (a: number, b: number) => any;
    readonly model_loadAdapter: (a: number, b: number, c: number) => [number, number, number];
    readonly model_loadFromOpfs: (a: any, b: number, c: number) => any;
    readonly model_loadFromOpfsTextOnly: (a: any, b: number, c: number) => any;
    readonly model_loadFromUrl: (a: number, b: number) => any;
    readonly model_maxContext: (a: number) => number;
    readonly model_position: (a: number) => number;
    readonly model_releaseAudioWeights: (a: number) => number;
    readonly model_releaseVisionWeights: (a: number) => number;
    readonly model_renderChat: (a: number, b: any, c: number) => [number, number, number, number];
    readonly model_renderChatForContinuation: (a: number, b: any, c: number) => [number, number, number, number];
    readonly model_reset: (a: number) => void;
    readonly model_restoreKvState: (a: number, b: number, c: number) => [number, number];
    readonly model_saveKvState: (a: number) => any;
    readonly model_setSampling: (a: number, b: any) => [number, number];
    readonly model_shrinkKv: (a: number, b: number) => [number, number, number];
    readonly model_step: (a: number, b: number) => any;
    readonly model_stepWithEmbedding: (a: number, b: number, c: number) => any;
    readonly model_tokenStr: (a: number, b: number) => [number, number];
    readonly model_vocabSize: (a: number) => number;
    readonly __wasm_start: () => void;
    readonly __wbg_wasmdatabase_free: (a: number, b: number) => void;
    readonly wasmdatabase_close: (a: number) => void;
    readonly wasmdatabase_createFunction: (a: number, b: number, c: number, d: number, e: any) => void;
    readonly wasmdatabase_deleteFunction: (a: number, b: number, c: number) => number;
    readonly wasmdatabase_exec: (a: number, b: number, c: number) => [bigint, number, number];
    readonly wasmdatabase_execMany: (a: number, b: number, c: number) => [number, number];
    readonly wasmdatabase_execParams: (a: number, b: number, c: number, d: any) => [bigint, number, number];
    readonly wasmdatabase_flush: (a: number) => [number, number];
    readonly wasmdatabase_fromBuffer: (a: number, b: number) => [number, number, number];
    readonly wasmdatabase_new: () => [number, number, number];
    readonly wasmdatabase_openPersisted: (a: number, b: number, c: number, d: bigint, e: number) => any;
    readonly wasmdatabase_openWithIdb: (a: number, b: number, c: number, d: bigint) => any;
    readonly wasmdatabase_openWithOpfs: (a: number, b: number, c: number, d: bigint, e: number) => any;
    readonly wasmdatabase_query: (a: number, b: number, c: number) => [number, number, number];
    readonly wasmdatabase_queryOne: (a: number, b: number, c: number) => [number, number, number];
    readonly wasmdatabase_queryParams: (a: number, b: number, c: number, d: any) => [number, number, number];
    readonly wasmdatabase_toBuffer: (a: number) => [number, number, number, number];
    readonly init: () => void;
    readonly wasmdatabase_openInMemory: () => [number, number, number];
    readonly wasm_bindgen__convert__closures_____invoke__hd0c2dfa3660cef4f: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h78b8fad9cd9306ea: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h14ad95f599c68105: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h31efb7a7fb02db26: (a: number, b: number, c: any) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
