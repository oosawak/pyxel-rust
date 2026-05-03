# アリサクエスト 開発引き継ぎ資料

> 作成日: 2026-05-02  
> プロジェクトオーナー: oosawak  
> GitHub: https://github.com/oosawak/pyxel-rust  
> 公開URL: https://oosawak.github.io/pyxel-rust/examples/rpg/

---

## 1. プロジェクト概要

**アリサクエスト** は、Rust + WebAssembly で動作するターン制 RPG バトルデモです。  
主人公「アリサ」がネコテックワールドを舞台に戦う、ブラウザで動くゲームです。

### ビジョン（将来像）
```
Flutter マップアプリ（位置情報）
    ↓ 場所でエンカウント
アリサクエスト WASM バトルエンジン（このプロジェクト）
    ↓ バトル結果を返す
Flutter アプリに結果反映 + AI モンスター生成
```

---

## 2. リポジトリ・フォルダ構成

```
/home/oosawak/Workspace/
├── pyxel-rust/                  ← メインリポジトリ (GitHub: oosawak/pyxel-rust)
│   ├── src/                     ← pyxel-rust エンジン本体 (Rust ライブラリ)
│   │   └── backend/
│   │       ├── wasm_backend/mod.rs   ← WASM描画・入力・テキスト処理
│   │       └── font.rs               ← ASCII ビットマップフォント (4×6px)
│   ├── docs/                    ← GitHub Pages で公開されるファイル
│   │   ├── index.html           ← ゲーム一覧トップページ
│   │   ├── examples/
│   │   │   └── rpg/             ← アリサクエスト WASM ゲーム
│   │   │       ├── index.html   ← ゲームの HTML (モバイルゲームパッド含む)
│   │   │       └── pkg/         ← WASM ビルド成果物 (自動生成)
│   │   │           ├── rpg_rust_lib.js
│   │   │           └── rpg_rust_lib_bg.wasm
│   │   └── assets/design/       ← ゲーム用デザインアセット (PNG)
│   └── examples/
│       └── rpg.rs               ← デスクトップ用スタンドアロン版 (参考用)
│
└── arisa_quest/                 ← アリサクエスト ゲームコード (メインの作業場所)
    ├── Cargo.toml
    └── src/
        └── main.rs              ← ★ ゲームのメインコード (598行)
```

> **注意**: `rpg_rust/` は別プロジェクト (Python R..P...G の Rust 移植) です。  
> アリサクエストのコードは `arisa_quest/` にあります。

---

## 3. デザインアセット一覧

場所: `/home/oosawak/Workspace/pyxel-rust/docs/assets/design/`

| ファイル名 | サイズ | 内容 |
|-----------|--------|------|
| `arisa-player-sprites.png` | 1536×1024 | アリサの全アクションスプライト |
| `arisa-character-design.png` | - | アリサ キャラクターターンアラウンド |
| `arisa-3d-art.png` | - | アリサ 3D アート "WELCOME TO THE FUTURE!" |
| `arisa-3d-turnaround.png` | - | アリサ 4方向 3D ビュー |
| `arisa-items.png` | - | アリサ専用装備・アイテム |
| `enemy-sprites.png` | 1536×1024 | 敵 No.01-07 (7体) |
| `enemy-sprites-2.png` | 1536×1024 | 敵 No.08-14 (7体) |
| `enemy-sprites-3.png` | 1536×1024 | 敵 No.15-22 (8体) |
| `enemy-sprites-4.png` | 1536×1024 | 敵 No.23-30 (8体) |
| `enemy-sprites-5.png` | 1536×1024 | 敵 No.31-40 (10体) |
| `boss-sprites.png` | 1536×1024 | ボス B01-B08 (8体) |
| `slime-cat-3d.png` | - | スライムキャット 3D ターンアラウンド |
| `slime-cat-portrait.png` | - | スライムキャット バストポートレート |
| `slime-cat-actions.png` | - | スライムキャット カラーバリエーション・アクション |
| `battle-backgrounds.png` | 1536×1024 | バトル背景 25種 |
| `arisa-world-backgrounds.png` | 1536×1024 | ワールド背景 20種 (番号・説明付き) |
| `arisa-world-backgrounds-2.png` | 1536×1024 | ワールド背景 28種 (詳細版) |
| `ui-mockup-map-battle.png` | - | アプリ UI モックアップ (Flutter マップ＋バトル) |
| `neko-tech-items.png` | - | ネコテック アイテムセット |
| `item-sprites.png` | - | 汎用 RPG アイテム |

### スプライトシートのレイアウト（推定）
全スプライトシートは 1536×1024px。  
自動的にグリッドを計算する際は以下を参考に:
- 4列 × 2行 → 各セル 384×512px（7〜8体収録シート向け）
- 実際のレイアウトは Python で確認可能:
```python
from PIL import Image
img = Image.open("enemy-sprites.png")
print(img.size)  # (1536, 1024)
```

---

## 4. ビルド方法

### 前提条件
```bash
# Rust toolchain (nightly 不要、stable で OK)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# WASM ターゲット追加
rustup target add wasm32-unknown-unknown

# wasm-pack インストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
# または
~/.cargo/bin/wasm-pack --version  # すでにインストール済みの確認

# SDL2 (デスクトップ版に必要、Linux)
sudo apt install libsdl2-dev
```

### WASM ビルド（GitHub Pages 向け）

```bash
cd /home/oosawak/Workspace

# WASM ビルド (arisa_quest → docs/examples/rpg/pkg/ に出力)
~/.cargo/bin/wasm-pack build arisa_quest \
  --target web \
  --out-dir ../pyxel-rust/docs/examples/rpg/pkg \
  --no-default-features \
  --features wasm-backend

# .gitignore を削除 (これをしないと pkg/ がコミットされない)
rm pyxel-rust/docs/examples/rpg/pkg/.gitignore
```

> ⚠️ `--out-dir` のパスは `arisa_quest/` からの相対パスで指定します

### デスクトップ版ビルド（動作確認用）

```bash
cd /home/oosawak/Workspace

# ビルド (SDL2 バックエンド)
cargo build -p arisa_quest

# 実行
./target/debug/arisa_quest
```

### コンパイルエラーが出る場合

```bash
# SDL2 が見つからない場合
sudo apt install libsdl2-dev

# WASM ターゲットがない場合
rustup target add wasm32-unknown-unknown

# wasm-pack がない場合
cargo install wasm-pack
```

---

## 5. デプロイ方法（GitHub Pages）

```bash
cd /home/oosawak/Workspace/pyxel-rust

# WASM ビルド後に実行
git add docs/examples/rpg/pkg/
git add docs/examples/rpg/index.html  # HTML を変更した場合
git commit -m "Update Arisa Quest WASM build"
git push origin master
```

GitHub Pages は master ブランチの `docs/` ディレクトリを自動で公開します。  
プッシュ後 1〜3 分で反映されます。

---

## 6. 現在の実装状態

### 実装済み ✅
| 機能 | 説明 |
|------|------|
| タイトル画面 | アリサ＆スライムのアニメーション、Z キーでスタート |
| マップ探索 | 20×15 タイルマップ、矢印キーで移動 |
| ランダムエンカウント | 草地を歩くとバトル発生 |
| バトルコマンド | 「たたかう / スキル / どうぐ / にげる」2×2 グリッド選択 |
| スキル | MP 消費、2倍ダメージ |
| どうぐ | ポーション使用（HP 回復）|
| 5種の敵 | スライム/シャドウキャット/フレイムインプ/ゴースト/ロックゴーレム |
| 敵スプライト | コードで描画（プリミティブ図形） |
| HP/MP バー | バトル UI に表示 |
| EXP・レベルアップ | バトル勝利後に EXP 獲得、LV UP 演出 |
| フラッシュエフェクト | ダメージ時に点滅 |
| モバイルゲームパッド | タッチ対応 D-pad + ZX ボタン |
| 日本語テキスト | Canvas fillText + DotGothic16 フォントで描画 |
| WASM デプロイ | GitHub Pages で公開済み |

### 未実装 / 改善余地あり ❌
| 機能 | 優先度 | 備考 |
|------|--------|------|
| 実アートアセット表示 | ★★★ | AI生成スプライトシートをバトル画面に表示 |
| バトル背景 | ★★★ | `battle-backgrounds.png` / `arisa-world-backgrounds-2.png` の表示 |
| 実敵スプライト | ★★★ | `enemy-sprites-*.png` からの切り出し表示 |
| アリサスプライト | ★★★ | `arisa-player-sprites.png` の表示 |
| タウン/NPCシステム | ★★ | 宿屋、ショップ、会話 |
| ボスバトル | ★★ | `boss-sprites.png` の8体 |
| BGM/SE | ★★ | バトル・フィールド音楽 |
| セーブシステム | ★★ | localStorage 使用 |
| 全40体の敵対応 | ★ | 現在は5体のみ |
| マルチゾーン | ★ | 雪原/砂漠/溶岩 エリア |

---

## 7. 技術スタック詳細

### pyxel-rust エンジン
- **場所**: `/home/oosawak/Workspace/pyxel-rust/src/`
- Python の [Pyxel](https://github.com/kitao/pyxel) ゲームエンジンを Rust で再実装したカスタムエンジン
- WASM バックエンド: HTML5 Canvas 2D API を使用
- デスクトップバックエンド: SDL2

### API リファレンス（主要関数）
```rust
use pyxel_rust::prelude::*;

// 初期化
init(width: u32, height: u32, title: &str, fps: u32, quit_key: u32);

// 描画
cls(col: u8);                                    // 画面クリア
rectfill(x: f32, y: f32, w: f32, h: f32, col: u8);
circfill(x: f32, y: f32, r: f32, col: u8);
line(x1: f32, y1: f32, x2: f32, y2: f32, col: u8);
pset(x: f32, y: f32, col: u8);
text(x: f32, y: f32, s: &str, col: u8);         // ASCII のみ
blt(x, y, img, sx, sy, w, h, colkey: Option<u8>); // イメージバンクから描画

// 入力
btn(key: u32) -> bool;    // 押し続けている間 true
btnp(key: u32) -> bool;   // 押した瞬間のみ true
// キー定数: KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_Z, KEY_X, KEY_RETURN, KEY_ESCAPE

// ユーティリティ
frame_count() -> u32;
rnd_int(min: i32, max: Option<i32>) -> i32;
```

### 日本語テキスト表示（重要）
pyxel のビットマップフォントは ASCII のみ対応のため、日本語は特別処理が必要。  
実装済み: `wasm_backend/mod.rs` の `draw_text()` が非 ASCII 文字を検出すると  
Canvas の `fillText()` で **DotGothic16** フォントを使って描画する。

```rust
// 日本語テキストはそのまま呼べばOK (WASM では自動的に fillText 経由)
text(x, y, "スライムが現れた！", WHITE);
```

### ゲームループパターン
```rust
use std::rc::Rc;
use std::cell::RefCell;

fn run_game() {
    init(160, 120, "アリサクエスト", 30, KEY_Q);
    let game = Rc::new(RefCell::new(Game::new()));
    let upd = game.clone();
    let drw = game.clone();
    run(
        Box::new(move || upd.borrow_mut().update()),
        Box::new(move || drw.borrow().draw()),
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn main() { run_game(); }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&info.to_string().into());
    }));
    run_game();
}
```

---

## 8. WASM と JavaScript の役割分担（設計思想）

アリサクエスト Web 版は「WASM がバトルエンジン」「JS が世界（地図・GPS・UI）」という明確な分担で設計されています。

### 現在の役割分担

| 領域 | 担当 | 理由 |
|------|------|------|
| バトルロジック（HP/MP/コマンド/状態遷移） | **WASM (Rust)** | 複雑な計算・型安全性 |
| バトルキャラクター描画（bg-canvas） | **JavaScript** | スプライトシート・Canvas API 直接操作が楽 |
| バトル状態エクスポート | **WASM → JS** | `get_game_state()` など関数経由で渡す |
| GPS・位置情報取得 | **JavaScript 必須** | ブラウザ Geolocation API |
| Leaflet 地図表示 | **JavaScript 必須** | JS ライブラリ |
| 距離計算・エンカウント判定 | **JavaScript** | GPS データが JS 側にあるため |
| UI アニメーション・CSS 演出 | **JavaScript / CSS** | DOM 操作・CSS アニメーション優位 |
| タッチ・スワイプ操作 | **JavaScript 必須** | ブラウザ Touch Events API |
| サイバー演出（マトリックス雨） | **JavaScript** | Canvas API アニメーション |

### WASM エクスポート関数（JS から呼ぶもの）

```javascript
// ゲーム状態 (0=タイトル, 1=スタンバイ, 2=バトル)
Module.get_game_state()

// キャラクター情報
Module.get_enemy_idx()      // 敵スプライトIndex
Module.get_bg_idx()         // 背景Index
Module.get_enemy_flash()    // 敵フラッシュフラグ
Module.get_player_flash()   // プレイヤーフラッシュフラグ

// ステータス
Module.get_player_hp()
Module.get_player_max_hp()
Module.get_player_mp()
Module.get_player_max_mp()
Module.get_player_level()
```

### JS → WASM へ渡す情報

```javascript
// キー入力（WASM 側でポーリング）
// WASM が内部で pyxel の btn() を使うため、
// JS 側からの注入は不要（WASM キャンバスが入力を受け取る）
```

### WASM に移せるもの（将来の選択肢）

- 距離計算・戦闘パラメータ計算 → 計算量が増えれば移す価値あり
- 敵 AI のルーティング・行動パターン → 複雑化した場合

### WASM に移せないもの（JS 必須）

- **GPS・Geolocation API** → ブラウザ標準 API、JS からしか呼べない
- **Leaflet 地図** → JS ライブラリ
- **DOM・CSS 操作** → JS/CSS の領域
- **Touch/Pointer Events** → ブラウザイベント、JS で受け取る必要あり

### 設計原則

> **「WASMはゲームのコア（ルール・演算）、JSは現実世界とのブリッジ（GPS・地図・UI）」**

この分割は変えないこと。WASMにGPS処理を持たせようとすると、ブラウザAPIの壁にぶつかる。

---

## 9. 高品質バトル画面の実装方針

オーナーの要望: **AI生成アートアセットをバトル画面に表示し、スマホアプリ並みのクオリティにする**

### 推奨アプローチ: HTML5 Canvas ハイブリッド方式

```
[下レイヤー: HTML5 Canvas]  ← AI生成背景 + 敵/プレイヤースプライト
[上レイヤー: pyxel Canvas]  ← HP バー・コマンドメニュー・テキスト
   mix-blend-mode: screen   ← バトル背景エリア (BLACK) は透過
```

**手順:**
1. `docs/examples/rpg/` に画像ファイルをコピー
2. `index.html` に背景 Canvas を追加（pyxel Canvas の後ろに配置）
3. JavaScript でスプライトシートを読み込み、バトル状態に応じて描画
4. WASM から JS にバトル情報を渡す関数をエクスポート:
   ```rust
   #[wasm_bindgen]
   pub fn get_battle_enemy_idx() -> i32 { ... }
   #[wasm_bindgen]
   pub fn get_game_state() -> i32 { ... }  // 0=title, 1=map, 2=battle
   ```
5. pyxel 側ではバトル背景エリアを BLACK で塗りつぶす（透過扱いになる）

### スプライト切り出し方法（JavaScript 例）
```javascript
// enemy-sprites-4.png (No.23-30, 4列×2行, 各384×512px) から No.25 を表示
const COLS = 4, ROWS = 2;
const cellW = spriteSheet.width / COLS;   // 384px
const cellH = spriteSheet.height / ROWS;  // 512px
const idx = 2;  // No.25 = index 2 (0始まり)
const sx = (idx % COLS) * cellW;
const sy = Math.floor(idx / COLS) * cellH;
ctx.drawImage(spriteSheet, sx, sy, cellW, cellH, destX, destY, destW, destH);
```

---

## 9. 関連プロジェクト

| プロジェクト | 場所 | 説明 |
|------------|------|------|
| **アリサクエスト** | `/home/oosawak/Workspace/arisa_quest/` | このプロジェクト |
| **R..P...G 移植版** | `/home/oosawak/Workspace/rpg_rust/` | Python RPG の Rust 忠実移植 (進行中) |
| **Flutter アプリ** | `/home/oosawak/Workspace/pyxel_flutter_demo/` | ゲームランチャー + 将来的に地図連携 |
| **Cubeboy** | `/home/oosawak/Workspace/cubeboy_rust/` | アクションゲーム (完成) |
| **Lineboy** | `/home/oosawak/Workspace/lineboy_rust/` | ラインアクション (完成) |
| **pyxel-rust エンジン** | `/home/oosawak/Workspace/pyxel-rust/` | カスタム Rust ゲームエンジン |

---

## 10. よくある問題と解決方法

### Q: wasm-pack ビルドが失敗する
```bash
# パスを確認
~/.cargo/bin/wasm-pack --version
# ない場合はインストール
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Q: 日本語が表示されない
WASM 版では DotGothic16 フォントが読み込まれていることを確認:
```html
<link href="https://fonts.googleapis.com/css2?family=DotGothic16&display=swap" rel="stylesheet">
```
オフライン環境では Google Fonts が使えないため、フォントファイルをローカルに配置する必要があります。

### Q: pkg/.gitignore のせいで WASM がコミットされない
```bash
rm docs/examples/rpg/pkg/.gitignore
git add docs/examples/rpg/pkg/
```

### Q: デスクトップで実行したい (SDL2)
```bash
sudo apt install libsdl2-dev  # Ubuntu/Debian
brew install sdl2              # macOS
cargo run -p arisa_quest
```

### Q: 画面が真っ黒/白で何も表示されない
ブラウザのコンソール (F12) を開いて Rust のパニックメッセージを確認:
```javascript
// index.html の JS コンソールにエラーが出るはず
```

---

## 11. コンタクト・参考リンク

- **GitHub リポジトリ**: https://github.com/oosawak/pyxel-rust
- **公開ゲーム (GitHub Pages)**: https://oosawak.github.io/pyxel-rust/
- **アリサクエスト直リンク**: https://oosawak.github.io/pyxel-rust/examples/rpg/
- **Pyxel (Python 版, 参考)**: https://github.com/kitao/pyxel
- **wasm-pack ドキュメント**: https://rustwasm.github.io/docs/wasm-pack/
