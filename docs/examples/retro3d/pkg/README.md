# pyxel-rust

> ⚠️ **このリポジトリは現在開発中です。**  
> API・仕様は予告なく変更される可能性があります。

**pyxel-rust** は、[Pyxel](https://github.com/kitao/pyxel) にインスパイアされた Rust 製のゲームライブラリです。
`wasm-bindgen` を使用して WebAssembly ターゲットをサポートし、ブラウザ上でゲームを動作させることができます。  
（Emscripten は使用していません。元の Python 版 Pyxel が Emscripten を使用しています。）

---

## 特徴

- 🧊 **Pyxel Cube 3D（デスクトップ・ブラウザ）** — 公式 Pyxel 3.0.0 のソフトウェア3D描画・シーングラフ・衝突判定を利用
- 🦀 **Rust 製コア** — 型安全で高速なゲームエンジン
- 🌐 **WASM 対応** — `wasm-bindgen` / `wasm-pack` でブラウザ向けにビルド（`wasm32-unknown-unknown`）
- 🖼️ **Canvas 2D 統合** — ブラウザでは `web-sys` 経由で Canvas 2D API を使用
- 🖥️ **デスクトップ対応** — pyxel-core (SDL2) または wgpu バックエンドで実行
- 🎮 **Pyxel 互換 API** — `init` / `run` / `cls` / `blt` / `text` など主要 API を実装
- 🐍 **Python CLI ツール** — `pyxel_rust_cli.py`（Python 製）で Rust ゲームのビルド・実行を管理

---

## バックエンド構成

| バックエンド | 対象 | 使用ライブラリ | feature フラグ |
|---|---|---|---|
| `pyxel-core-backend` | デスクトップ（デフォルト） | pyxel-core (SDL2) | `pyxel-core-backend` |
| `wgpu-backend` | デスクトップ（代替） | pixels + winit | `wgpu-backend` |
| `wasm-backend` | ブラウザ | wasm-bindgen + web-sys | `wasm-backend` |

---

## リポジトリ構成

```
pyxel-rust/
├── src/                    ← pyxel-rust エンジン (Rust)
│   ├── lib.rs
│   ├── api/                ← graphics, input, audio, system, math ...
│   └── backend/
│       ├── wasm_backend/   ← Canvas 2D API (ブラウザ / wasm-bindgen)
│       └── wgpu_backend/   ← wgpu バックエンド (デスクトップ代替)
├── examples/               ← Rust サンプルソース
│   ├── cubeboy.rs          ← プラットフォーマー
│   ├── lineboy.rs          ← ラインアクション
│   └── rpg.rs              ← RPG
├── docs/
│   └── examples/           ← ブラウザで動作するデモ (GitHub Pages)
│       ├── arisa-quest/    ← GPS連動RPGデモ
│       ├── cubeboy/        ← プラットフォーマーデモ
│       └── lineboy/        ← ラインアクションデモ
├── scripts/
│   └── ci.sh               ← CI スクリプト (fmt / clippy / build / test)
├── pyxel_rust_cli.py       ← Python CLI (Pyxel CLI 拡張)
├── pyproject.toml          ← Python パッケージ設定
└── install-deps.sh         ← ビルド依存ライブラリインストーラー
```

---

## セットアップ

### 依存ライブラリのインストール（デスクトップビルド用）

[rustup](https://rustup.rs/) を使用してください。`rust-toolchain.toml` は公式Pyxel 3.0.0と同じ
`nightly-2026-11-05`（WASMターゲット・rustfmt・Clippy付き）を指定します。

デフォルトの静的SDL2ビルドにはCMake、C/C++コンパイラ、libclangが必要です。
macOSではXcode Command Line ToolsとCMakeを準備します：

```bash
xcode-select --install  # 未導入の場合
brew install cmake
```

Debian / Ubuntuでは以下のスクリプトで依存ライブラリを導入できます：

```bash
sudo bash install-deps.sh
```

### Python CLI のインストール

```bash
pip install -e .
# → pyxel-rust コマンドが使用可能になります
```

---

## ビルド方法

### WASM ビルド（ブラウザ向け）

```bash
# wasm-pack のインストール（未インストールの場合）
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# ビルド（wasm-backend feature を指定）
wasm-pack build --target web --out-dir docs/examples/arisa-quest/pkg \
  -- --no-default-features --features wasm-backend
```

### デスクトップビルド

```bash
# デフォルト（pyxel-core / SDL2 バックエンド）
cargo build
cargo run --example cubeboy

# wgpu バックエンドを使う場合
cargo build --no-default-features --features wgpu-backend
```

### Pyxel 3.0.0 / レトロ3D

デフォルトのSDL2バックエンドは、公式Pyxel **3.0.0** のリリースコミット
`e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd` に固定しています。
CargoがGitHubから直接取得するため、`pyxel_fork` の手動配置は不要です。
初回は依存関係とSDL2ソースのダウンロードにネットワーク接続が必要です。

```bash
# 回転する立方体（SDL2静的リンク）
cargo run --example retro3d

# システムのSDL2を使用する場合
cargo run --no-default-features --features sdl2_dynamic --example retro3d

# macOS + Homebrew SDL2（brew install sdl2）の場合
BINDGEN_EXTRA_CLANG_ARGS="-I$(brew --prefix sdl2)/include/SDL2" \
LIBRARY_PATH="$(brew --prefix sdl2)/lib" \
cargo run --no-default-features --features sdl2_dynamic --example retro3d

# ウィンドウ不要の3D描画テスト
cargo test --locked --test retro3d

# wgpuの画像・タイルマップ互換性テスト
cargo test --locked --no-default-features --features wgpu-backend --test wgpu_compat
```

`pyxel_rust::cube` は公式の `Vec3` / `Mat4` / `Quat` / `Camera` /
`Mesh` / `Node` / `Scene` / 衝突判定と描画関数を公開します。
`prelude::render_3d(&target, &camera, |ctx| { ... })` で画面やオフスクリーン画像に描画できます。
SDL2の画面は `pyxel_rust::screen()`、各バックエンドのオフスクリーン画像は `Image::new(width, height)` で取得します。
使用例は [`examples/retro3d.rs`](examples/retro3d.rs) を参照してください。

`render_3d` はフレームごとに深度をリセットし、カメラのバッファを再利用します。
`camera.clear_color = None` なら描画先画像の既存ピクセルを保持します。
SDL2の画面に直接描く場合は既存の2D背景を保持し、描画後に `text` などでHUDを重ねられます。
公式Python版のNodeコールバックはこのRustラッパーでは自動実行しません。
公式Python API全体との互換性を保証するものではありません。
`wasm-backend` でも同じ `cube` / `Image` / `render_3d` を利用できます。
描画した画像を `blit_3d(&target, x, y)` でCanvas用フレームバッファへ転送し、
転送は画像全体を上書きするため、その後に2Dの文字やUIを重ねます。`wgpu-backend` のCube接続は未対応です。

### ブラウザ3Dデモ

```bash
wasm-pack build --target web --out-dir docs/examples/retro3d/pkg \
  -- --locked --no-default-features --features wasm-backend
python3 -m http.server 8765 --bind 127.0.0.1 --directory docs
# http://127.0.0.1:8765/examples/retro3d/
```

3つの立方体を回転し、一時停止・距離変更できます。
左右キー / Space / R、または画面のタッチボタンで操作します。
実装は [`src/games/retro3d.rs`](src/games/retro3d.rs)、
公開ページは [`docs/examples/retro3d/`](docs/examples/retro3d/) です。
ビルド済みWASM/JSを同梱しているため、プレビューだけならHTTPサーバーの起動で確認できます。

ブラウザ版は公式3.0.0の固定ソースを
[`vendor/pyxel-cube-core`](vendor/pyxel-cube-core/UPSTREAM.md) に同梱しています。
Cubeの描画・数学・衝突判定を維持し、SDL2との接続を切り離して既存のCanvasバックエンドへ接続します。
ブラウザでは `cube::Mesh::from_glb_bytes(&bytes, colkey, fps)` でGLBのバイト列を読み込み、
`draw_mesh_3d(ctx, &mesh.borrow(), &transform, state)` で階層変換とマテリアルを反映して描画できます。
ファイルシステムやPythonのNodeコールバックは使用しません。

デモ下部の「ファイルを選ぶ」で自分のGLBを表示できます。「サンプルを表示」では
テクスチャ付きピラミッドを読み込みます。「動くサンプル」には上下移動と回転の2クリップを用意しています。
選択したファイルはブラウザ内で処理します。
表示位置と倍率は自動調整し、読み込みに失敗しても現在のモデルを保持します。

対応範囲：
- デモのファイル上限は16 MiB。画像・バッファをGLBのBINチャンクに埋め込んでください。
- 埋め込みテクスチャ、ノードの親子変換、パートごとのマテリアルに対応します。
- ノードの移動・回転・拡大縮小アニメーションを、クリップ選択・再生/停止・シーク・ループ・速度変更で確認できます。
- スキニング、モーフターゲット、外部画像/バッファは未対応です。表示位置と倍率は初期姿勢を基準に調整します。
- サンプルは外部アセットを使わず生成しています。`python3 scripts/generate-retro3d-model.py` で再生成できます。

`MeshAnimationPlayer` はSDL2とWASMの共通APIです。GLB読込時と同じFPSを指定し、
ゲームの `update` から `advance(経過秒)` を呼び出します。選択時に元の姿勢を復元するため、
異なるノードを動かすクリップに切り替えても前の姿勢が残りません。

```rust
let mut player = MeshAnimationPlayer::new(mesh.clone(), 30.0)?;
player.select_clip(Some(0))?;
player.advance(1.0 / 30.0)?; // ゲーム更新ごとのシミュレーション時間
player.set_playing(false);
player.seek(1.0)?;          // 秒単位でシーク
player.select_clip(None)?;  // 元の姿勢へ戻す
```

デモの「一時停止」は自動回転とアニメーションを両方停止します。
アニメーション欄の再生ボタンはモデルの動きを再開し、シークはアニメーションを停止して指定位置を表示します。
アニメーション付きモデルの読込時は、動きを見やすくするため自動回転をオフにします。

```bash
# ブラウザ用のプラットフォーム非依存レンダラーも同じ描画テストを実行
cargo test --locked --no-default-features --features wasm-backend --test retro3d --test glb_browser --test animation_browser

# ブラウザで表示・操作・モバイルレイアウトを検証（HTTPサーバーを起動した状態）
npm install --no-save playwright@1.62.1
npx playwright install chromium
node scripts/playtest-retro3d.cjs
```

デモ画面の「Pyxel全機能の対応状況を確認」を開くと、Pyxel 3.0.0の2D/共通・Cubeの
公式APIと定数738項目、およびRust拡張3項目の状況をバックエンド別に確認できます。
検索・分類・実装状況による絞り込みに対応します。CLI・エディタUI・非公開APIは一覧の対象外です。
「実装あり」はRustの相当機能があるというソース確認の結果で、完全互換や全条件の検証済みを意味しません。
関数名があっても空処理・固定値のスタブは未対応と表示し、関連テストと未検証を区別します。
「ブラウザ確認を実行」は起動・Canvas・デモ状態・GLB/アニメーションAPI接続の4項目を確認し、モデルや再生状態を変更しません。
音声や全APIの動作確認にはなりません。

対応表の確認基準はエンジンのコミットに固定しています。APIを変更した際は判定規則を見直し、再生成してください。
生成後の古い判定を検出するため、ソースのハッシュも記録しています。

```bash
python3 scripts/generate-compatibility.py --check
python3 scripts/test-compatibility.py
# 判定の見直し後に再生成
python3 scripts/generate-compatibility.py
# 公式API一覧は固定コミットのcheckoutから抽出
python3 scripts/extract-pyxel-api.py --upstream /path/to/pyxel --check
# 対応パネルのブラウザテスト（HTTPサーバーが必要）
node scripts/playtest-compatibility.cjs
```

スクリーンショットは一時ディレクトリの `pyxel-retro3d-smoke` に保存します。
`PYXEL_TEST_URL` と `PYXEL_TEST_OUTPUT` でテスト先と出力先を指定できます。

公式3.0.0では旧MML構文と2.0以前のリソース読込が削除されています。
古いデスクトップ用アセットは移行が必要です。
詳細: [公式変更履歴](https://github.com/kitao/pyxel/blob/v3.0.0/CHANGELOG.md)

### CLI コマンド（Python CLI 経由）

```bash
pyxel-rust rust_run <PROJECT_NAME>       # cargo run で実行
pyxel-rust rust_package <PROJECT_NAME>  # .pyxapp にパッケージング
pyxel-rust rust_app2wasm <PROJECT_NAME> # WASM ビルド＆ローカルサーバー起動
```

### 公式CubeデモのRust移植と比較

[`docs/examples/cube/`](docs/examples/cube/) で公式の6デモをRust版／公式Python版で並べて比較できます。
PCでは横並び、狭い画面では縦並びになり、単独表示にも切り替えられます。左右の再生時刻と入力は同期せず、それぞれ操作できます。
Hello Cube、Basic Shapes、Custom Shapes、Mesh and Motion、3D Collision、3D Physicsを公式3.0.0のコードから移植しています。
Rust実装は [`src/games/cube_examples/`](src/games/cube_examples/) です。基本図形・レーザー・GLBモーション・庭園のコイン/ゴール判定・2段ジャンプ・物理応答と投射体に対応します。
タッチ操作・ゲームパッド入力・一時停止を追加し、Qは画面を閉じる代わりに停止します。
比較画面を切り替えると各デモを再起動します。両ランタイムのフレーム時刻は同期しません。

モデルは公式配布GLBをそのまま使用します。Gazelle by monkus（CC BY 4.0）のクレジットを画面と
[`assets/gazelle.md`](src/games/cube_examples/assets/gazelle.md) に表示しています。
効果音は公式MMLから公式シンセサイザーで生成したWAVを、音声オン操作後にWeb Audioで再生します。
汎用のNodeコールバックAPIを追加したものではなく、サンプル内で更新・描画・衝突通知の順序を実装しています。

```bash
# SDL2でも同じ6シーンを実行（1〜6を指定）
cargo run --locked --example cube_examples -- 1
cargo test --locked --test cube_examples
# ブラウザ向けの同じシーンを検証
cargo test --locked --no-default-features --features wasm-backend --test cube_examples
cargo test --locked --no-default-features --features wasm-backend --lib games::cube_examples
# 公式MMLの効果音を再生成（SDL2ビルド環境が必要）
cargo run --locked --example cube_audio
# 公式PNGをPyxel色インデックスへ変換
python3 scripts/prepare-cube-textures.py --upstream /path/to/pyxel
# ブラウザ比較テスト（ローカルHTTPサーバーと外部の公式ページへ接続）
node scripts/playtest-cube-examples.cjs
node scripts/playtest-cube-comparison.cjs
```

Rust版の公式Cubeデモでは、描画解像度を標準・2倍・4倍に変更できます。3D形状を実際に高解像度で描画し、HUDと入力は元の座標系を保ちます。解像度変更時はRust版だけを再起動します。ネイティブ版は `cargo run --example cube_examples -- 1 2` の第2引数で倍率を指定できます。

### 共通APIの追加対応

[基本APIの動作確認](https://oosawak.github.io/pyxel-rust/examples/api-coverage/) で楕円・三角形・塗りつぶし、キーボード、マウス、標準ゲームパッド4台の状態を確認できます。

- 入力：`btnp_hold`のhold/repeat、`btnv`、`set_btn`、`set_btnv`、`input_keys`、マウスボタン・ホイール・カーソル表示。ブラウザのマウス座標はCSSの表示サイズを考慮します。フォーカスを失った場合は押下を解除します。
- 描画：`elli`、`ellib`、`tri`、`trib`、`fill`をSDL2とCanvas/WASMで公開し、公式ラスタライザーを使用します。`rectb` / `circb`も追加。既存Rustゲーム用に`rect` / `circ`の輪郭描画の意味は維持しています。
- 数学：`ceil`、度数の`atan2`、`rndi`、`nseed`、`noise`。SDL2とCanvasの乱数・ノイズは公式コアを使用します。従来の`rnd_int`が常に0になる不具合も修正しました。
- 定数：不足していた260件を公開しました。定数の存在と機器の入力対応は別です。キー番号はバックエンド固有なので、数値を直書きせず定数名を使用してください。ブラウザのゲームパッドは標準マッピングに対応し、wgpuのゲームパッドイベントは引き続き未対応です。

`set_mouse_pos`はブラウザ／wgpuでは論理座標の変更のみで、OSのポインターは移動しません。ブラウザのホイールは方向を整数ステップに正規化します。ゲームパッドの自動テストは模擬Web Gamepadオブジェクトを使い、実機の全モデルの確認を意味しません。

```bash
cargo test --locked --no-default-features --features wasm-backend --lib --test math_compatibility
node scripts/playtest-api-coverage.cjs
```

### リソースとブラウザ音声

[リソースと音声の確認ページ](https://oosawak.github.io/pyxel-rust/examples/resources-audio/) で共有バンクの編集、PNG読込・保存、MML・PCM・曲の再生、チャンネル別停止とWAV保存を試せます。

`images()` / `tilemaps()` / `sounds()` / `musics()` / `channels()` / `tones()` は、描画・音声が参照する共有ハンドルのVecを返します。ハンドルの内容を編集すると即時に反映します。返り値Vec自体の要素差し替えやバンク数変更は反映しません。これらの変更には`with_images` / `with_tilemaps` / `with_sounds` / `with_musics` / `with_channels` / `with_tones`を使用できます。Canvasの`bltm`はタイル(0,0)と`ImageSource::Index`/`Image`の両方を公式コアで描画します。

Canvas版では`Image::from_image_bytes` / `load_bytes` / `png_bytes`、`Font::from_bytes`（BDF/TTF/OTF）、`Sound::load_pcm_bytes`、`Sound::wav_bytes` / `Music::wav_bytes`を提供します。ページ側でFile/Fetch/Blobを扱うことで、仮想ファイルシステムなしに読込・ダウンロードできます。ファイルパス指定、画像保存時scale、FFmpegによる動画保存は未対応です。表示パレットは最大256色で、`colors()`から直接編集できます。

`blt_transform` / `bltm_transform`で回転・拡大縮小、`text_with_font`で任意フォントを使えます。SDL2のパレット・image_*・tilemap_*補助関数も実際のリソースに接続しています。

Canvas音声は公式シンセサイザーをWeb Audioへストリーミングします。初回はユーザー操作から`resume_browser_audio()`を呼んでください。`play` / `playm` / `stop` / `play_pos` / `is_playing`を実装し、エラーを扱う`try_play` / `try_playm`も提供します。内部サンプルレートは22,050Hzで、現時点の出力アダプターはScriptProcessorNodeを使用します。ブラウザが音声出力を許可しない場合は`browser_audio_status()`で状態を確認できます。

従来の`window.pyxelPlaySound` / `pyxelPlayMusic` / `pyxelStopSound`があれば優先して呼び、既存ページとの互換性を維持します。再生位置・状態は内蔵エンジンの情報で、外部JSフックの音声状態は含みません。wgpuの音声出力は引き続き未対応です。

```bash
cargo test --locked --no-default-features --features wasm-backend --test browser_resources
cargo test --locked --test native_resources
node scripts/playtest-resources-audio.cjs
```

### ブラウザの画面・文字・ファイル操作

[ブラウザAPIの確認ページ](https://oosawak.github.io/pyxel-rust/examples/browser-system/) で画面サイズ、タイトル、アイコン、全画面、整数倍表示、性能表示、256色描画を確認できます。SDL2/Canvasの共通APIは`title` / `icon` / `resize` / `fullscreen` / `integer_scale` / `perf_monitor`です。ブラウザの全画面化はユーザー操作とブラウザの許可が必要で、失敗は非同期の`browser_fullscreen`で受け取れます。

`input_text` / `set_input_text`と`dropped_files` / `set_dropped_files`を公開しています。Canvasは`browser_text_input(true)`でIME用の入力欄を有効にできます。文字・ドロップ名はフレームごとにリセットされます。ブラウザは絶対パスを取得できないため、ドロップしたファイルの内容は`dropped_file_bytes(index)`で取得し、次のドロップまで保持します。wgpuも文字・IME・ファイルドロップのイベントに接続しました。IMEの自動検証は合成イベントを使用しており、OSごとのIME実機検証は別です。

Canvasの`load_bytes` / `save_bytes`は`ResourceOptions`で除外バンクを指定でき、公式`.pyxres`のZIP/TOMLを直接読み書きします。保存対象は画像・タイルマップ・従来のSound配列・Music配列で、MML・PCM・Tone・Font・Nodeは含みません。パレットは`load_pal_text` / `save_pal_text`で別に保存します。破損データは変更前に検証し、選択した全バンクの検証に失敗した場合は既存リソースを維持します。マニフェストは32MiB、読込画像・タイルマップは合計1,600万セルに制限します。

`Tilemap::from_tmx_text` / `load_tmx_text`は公式コアと同じ埋め込みタイルセット・8×8タイル・CSV層を使用します。`browser_screenshot(scale)`（Rustでは`screenshot_png`）はテキストを含む表示CanvasをPNGバイト列にし、1〜16倍で保存できます。ファイル選択・ダウンロードはページ側で行います。SDL2には通常のファイルパスによる`load` / `save` / `load_pal` / `save_pal` / `screenshot`を公開しました。

`gen_bgm(preset, transpose, instrumentation, seed, play)`で公式生成器のMMLを取得し、`play=true`で再生できます。ブラウザの初回再生にはユーザー操作からの音声有効化が必要です。

```bash
cargo test --locked --no-default-features --features wasm-backend --lib --test browser_files
node scripts/playtest-browser-system.cjs
```

### 共有画面・録画・Nodeのライフサイクル

[画面・録画・Nodeの確認ページ](https://oosawak.github.io/pyxel-rust/examples/screen-tools/) で画像／タイルマップの透視投影、共有画面の直接編集、GIF録画、手動フレーム、Nodeの更新・衝突・描画・破棄を試せます。

`screen()` / `cursor()` / `font()`は実際の共有Imageを返します。Canvasの通常描画も同じImageへ統一し、`screen()`の画素・クリップ・カメラ・色変換の編集をそのまま使えます。標準ASCIIテキストは公式の4×6フォント画像を参照します（従来Canvas独自フォントの字幅5から公式の字幅4へ変更）。Canvasでの日本語重ね描画は論理画面とは別です。リサイズ後もscreenの共有ハンドルは維持します。借用中のImageを操作するAPIは、呼出し前にRefCellの借用を解放してください。

`blt3d` / `bltm3d`は公式の画像／タイルマップ透視投影を使用します。`show()`は描いた画面を表示し続けます。Canvasでは直ちに戻る非同期ループです。手動ループにはCanvasの`flip().await`（JSは`browser_flip`）を使い、`run/show`と併用しないでください。これはrequestAnimationFrame単位で、SDL2のfps待機付き同期flipとは異なります。`reset()`はCanvasではページ全体を再読込します。SDL2は`set_reset_handler`でアプリ側の再起動処理を登録してから呼び、未登録時はエラーを返します。

Canvasは`configure_screencast(seconds)`で明示的に録画を開始し、`screencast_gif(scale)`で公式エンコーダーによるGIFバイト列を取得します。空の録画はNone、エラー時は既存フレームを保持し、保存成功後は録画内容をクリアします。`reset_screencast()`は内容をクリアし、秒数0で録画を終了します。リングバッファは32MiB、最大30秒、倍率1〜16です。日本語のCanvas重ね描画・ソフトウェアカーソル・表示効果は録画に含みません。リサイズ時には録画を終了します。SDL2の`screencast(filename, scale)`は公式コアに接続します。

`screen_mode(0..2)`はSDL2では公式のシェーダー、Canvasでは通常・平滑化・走査線/周辺減光の簡易表示です。Canvas版は公式xBR/CRTと同じ画質ではありません。`user_data_dir`はSDL2のユーザーフォルダーを返し、ブラウザでは代わりに`user_data_store(vendor, app)`の`get/set/remove`でlocalStorageを使用します。ブラウザの容量制限・利用可否による失敗をResultで受け取れます。

`NodeScene`は公式`RcNode`のグラフを使うRustコールバック層です。`from_mesh`で名前・階層・材質・形状とMotionのモデル対応を変換し、`on_update/on_draw/on_collide/on_destroy`へクロージャを登録します。`update(frame_seconds)`は全体の更新通知、NodeごとのMotion自動再生、公式の運動積分、接触検出と通知、子から親への破棄通知・切離しの順に実行します。公式コアのvelocityはフレームごとの変位で、frame_secondsを掛け直しません。inactiveな親の更新は子にも及びます。コールバック中に登録を変更でき、同じシーンへの再入はエラーになります。

`NodeScene::draw(viewport, target)`はカメラ・可視性を親から継承し、Meshの付属Primitiveを描画します。カスタム描画では`with_node_draw_state` / `draw_node_primitive`を使用すると、不透明度とシェーディングも親から継承できます。描画状態をノードごとにリセットし、エラー・パニック時もカメラの深度バッファを戻します。Python継承の代わりにRustの登録式APIを使います。

[Nodeアニメーションの確認ページ](https://oosawak.github.io/pyxel-rust/examples/node-motion/) では、同じMeshの2体を別々のクリップ・速度で再生できます。`NodeScene::play_motion(node, motion, looping, speed, start_frame)`はNodeごとの再生を開始し、`update`のたびにspeedフレーム進めます。これは秒数ではなく、GLB読込時に指定したfpsでのフレーム単位です。逆再生（負のspeed）、速度0、ループ、非ループの終端停止に対応します。inactiveな親の下では再生を止め、再度activeになると続きから進みます。visibleは再生に影響しません。

`apply_motion(node, motion, frame, looping)`は指定部分木の同じモデル個体に属するパーツだけへ姿勢を適用します。既存の再生状態や他のクリップのチャンネルをリセットしません。`stop_motion(node)`は姿勢を保持して停止し、`motion_state(node)`で再生中のフレーム・速度・クリップを取得できます。親のプレイヤーを先に、子のプレイヤーを後に適用するため、子側の指定で上書きできます。

再生開始時には、その個体の全クリップが使用するパーツだけを元の姿勢へ戻します。アニメーション対象外の手動編集やモデル配置の親変換は保持します。Motionの適用はNodeの行列ハンドルを差し替える方式で、元Meshや別個体の再生結果を書き換えません。Motionは同じソースMeshのmotionsに属する必要があり、有限でない速度・フレームや不正な対象をエラーにします。使用中はMeshのパーツ構成とMotionリストを変更しないでください。従来の`MeshAnimationPlayer`も、Meshを直接描画する用途向けに維持しています。

対応表の`Mat4.pos/rot/scale`も再確認しました。公式3.0.0の[Pythonバインディング](https://github.com/kitao/pyxel/blob/e0b528f9a6f98bfc92bb47ebe1c7623cb3f609fd/crates/pyxel-binding/src/cube/mat4.rs#L23)は読み取り専用getterで、Rustの`pos()/rot()/scale_vec()`と同じです。以前の「書込setter不足」という判定を訂正しました。これは実装追加とは別の判定訂正です。

```rust,ignore
let mut scene = NodeScene::from_mesh(&mesh)?;
scene.root.borrow_mut().camera = Some(Camera::new());
let root = scene.root.clone();
scene.on_update(&root, |_, node| {
    // Update node.transform, its collider, or the hierarchy.
    Ok(())
});
let clip = mesh.borrow().motions[0].clone();
scene.play_motion(&root, &clip, true, 1.0, 0.0)?;
scene.update(1.0 / 30.0)?;
scene.draw([0.0, 0.0, 256.0, 192.0], None)?;
```

```bash
cargo test --locked --no-default-features --features wasm-backend --lib --test screen_recording --test scene_lifecycle
cargo test --locked --test scene_lifecycle --test retro3d
node scripts/playtest-screen-tools.cjs
cargo test --locked --no-default-features --features wasm-backend --test node_motion --test matrix_views
node scripts/playtest-node-motion.cjs
```

### ローカル確認（GitHub Pages 相当）

```bash
python3 -m http.server 8000 --directory docs
# → http://localhost:8000/examples/arisa-quest/
```

### CI（フォーマット・Lint・テスト）

```bash
bash scripts/ci.sh
```

---

## デプロイ

`docs/` ディレクトリを GitHub Pages として公開しています。  
自動デプロイは未設定のため、手動で `docs/` を更新後プッシュしてください。

---

## 参考

- [Pyxel 公式](https://github.com/kitao/pyxel) — API 設計の参考元
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — Rust ↔ JS バインディング
- [wasm-pack](https://rustwasm.github.io/docs/wasm-pack/) — WASM ビルドツール
- [Leaflet.js](https://leafletjs.com/) — arisa-quest の地図表示に使用
- [web-sys](https://docs.rs/web-sys) — Canvas 2D API バインディング

---

## ライセンス

MIT License
