# pyxel-rust で WASM ゲームを開発する

pyxel-rust を基盤として、WebAssembly (WASM) で実行可能なブラウザゲームを開発するためのガイドです。

## プロジェクト構成

```
/Workspace/
├── pyxel-rust/              # ゲームエンジン基盤
├── sendai_daikannon/        # WASM ゲーム例
├── [その他のゲーム]/
└── docs/examples/
    └── arisa-quest/         # ゲーム統合アプリ
        ├── index.html
        └── sendai_daikannon_wasm/  # ビルド出力
```

## WASM ゲーム開発の基本方針

### 設計原則

1. **ゲームロジックは Rust で実装**
   - HP 管理、攻撃計算、状態管理など
   - 純粋な計算ロジック

2. **Canvas 描画は HTML/JavaScript で実装**
   - WASM では DOM/Canvas へのアクセスが限定的
   - 画面表示は JavaScript 側で担当

3. **ゲームライブラリは独立**
   - pyxel-rust に依存しない
   - `wasm-bindgen` だけで十分

4. **Feature フラグで環境を分岐**
   - ネイティブビルド：`sdl2_static` など
   - WASM ビルド：`wasm-backend`

## セットアップ手順

### 1. 新しいゲームプロジェクトを作成

```bash
mkdir -p /home/oosawak/Workspace/<game_name>/src
```

### 2. Cargo.toml を作成

```toml
[package]
name = "<game_name>"
version = "0.1.0"
edition = "2021"

[dependencies]
wasm-bindgen = { version = "0.2", optional = true }

[features]
wasm-backend = ["dep:wasm-bindgen"]

[lib]
crate-type = ["cdylib"]

[target.'cfg(target_arch = "wasm32")'.dependencies]
web-sys = { version = "0.3", features = ["console"] }
```

### 3. ゲームロジックを実装

```rust
// src/lib.rs
#![cfg_attr(target_arch = "wasm32", no_main)]

use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub enum GameState {
    Running,
    Victory,
    Defeat,
}

pub struct Game {
    pub enemy_hp: f32,
    pub player_hp: f32,
    pub state: GameState,
    // ...
}

impl Game {
    pub fn new(player_hp: f32) -> Self { ... }
    pub fn update(&mut self) { ... }
    pub fn get_state(&self) -> GameState { ... }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    static mut GAME: Option<Game> = None;

    #[wasm_bindgen]
    pub fn game_init(player_hp: f32) {
        unsafe { GAME = Some(Game::new(player_hp)); }
    }

    #[wasm_bindgen]
    pub fn game_update() {
        unsafe {
            if let Some(game) = &mut GAME {
                game.update();
            }
        }
    }

    #[wasm_bindgen]
    pub fn game_get_state() -> u32 {
        unsafe {
            match GAME.as_ref().map(|g| g.get_state()) {
                Some(GameState::Running) => 0,
                Some(GameState::Victory) => 1,
                Some(GameState::Defeat) => 2,
                None => 0,
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() { println!("Game"); }
```

### 4. Workspace に登録

`/Workspace/Cargo.toml`:
```toml
[workspace]
members = [
    "pyxel-rust",
    "<game_name>",
    # ...
]
```

### 5. WASM ビルド

```bash
cd /home/oosawak/Workspace/<game_name>
cargo build --target wasm32-unknown-unknown --release --features wasm-backend
```

### 6. JavaScript 生成

```bash
~/.cargo/bin/wasm-bindgen \
  /home/oosawak/Workspace/target/wasm32-unknown-unknown/release/<game_name>.wasm \
  --out-dir /path/to/output \
  --target web
```

### 7. HTML に統合

```html
<script type="module">
  import init, * as game from './wasm_output/lib.js';

  async function startGame() {
    await init();
    game.game_init(100); // プレイヤーHP = 100
    
    function gameLoop() {
      game.game_update();
      const state = game.game_get_state();
      // Canvas描画...
      requestAnimationFrame(gameLoop);
    }
    gameLoop();
  }

  startGame();
</script>
```

## pyxel-rust の Feature フラグ

### ネイティブビルド（デスクトップ）

```bash
cargo build --release
# または
cargo build --release --features sdl2_static
```

### WASM ビルド

**注意**: WASM ターゲットでは `default` フィーチャは使用しない

```bash
cargo build --target wasm32-unknown-unknown --release --features wasm-backend
```

### 詳細設定

```bash
# SDL2 動的リンク
cargo build --release --features sdl2_dynamic

# Wgpu バックエンド
cargo build --release --features wgpu-backend
```

## トラブルシューティング

### WASM ビルドエラー: CMake / wasm-ld

**症状**: `wasm-ld-18 doesn't exist`, CMake エラー

**原因**: pyxel-core が WASM をサポートしていない

**解決策**:
1. WASM ゲームは pyxel-rust に依存しない設計にする
2. `--features wasm-backend` で明示的にビルド
3. Canvas 描画は HTML/JavaScript で実装

### WASM ファイルが生成されない

**症状**: `cargo build` は成功するが `.wasm` ファイルが出現しない

**原因**: `crate-type = ["cdylib"]` が `Cargo.toml` に不足している

**解決策**:
```toml
[lib]
crate-type = ["cdylib"]
```

### JavaScript で WASM 関数が見つからない

**症状**: `game.game_init is not a function`

**原因**: 
1. wasm-bindgen で `.js` ファイルが生成されていない
2. HTML で正しく import されていない

**解決策**:
```bash
# 出力ディレクトリを確認
ls -la /path/to/wasm_output/
# sendai_daikannon_lib.js が存在すること

# HTML で正しく import
import init, * as game from './sendai_daikannon_wasm/sendai_daikannon_lib.js';
```

## ベストプラクティス

### ✅ 推奨

- ゲームロジックは `static mut` で状態を保持
- 毎フレーム `game_update()` を呼び出す
- Canvas 描画は `requestAnimationFrame` のコールバックで実施
- HP/攻撃力などは getter 関数で取得

### ❌ 非推奨

- Rust 側で Canvas 描画を実装（web-sys の機能は制限的）
- 大きなメモリを WASM と JavaScript で共有
- ゲームロジックをいくつもの module に分散

## 完全な例

### Rust コード

```rust
// sendai_daikannon/src/lib.rs
#![cfg_attr(target_arch = "wasm32", no_main)]

use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState { Running, Victory, Defeat }

#[derive(Clone)]
pub struct Attack { pub power: f32, pub y: f32 }

pub struct Game {
    pub enemy_hp: f32,
    pub player_hp: f32,
    pub state: GameState,
    pub player_attacks: VecDeque<Attack>,
    pub enemy_attacks: VecDeque<Attack>,
    pub frame: u32,
}

impl Game {
    pub fn new(player_hp: f32) -> Self {
        Self {
            enemy_hp: 100.0,
            player_hp,
            state: GameState::Running,
            player_attacks: VecDeque::new(),
            enemy_attacks: VecDeque::new(),
            frame: 0,
        }
    }

    pub fn update(&mut self) {
        if self.state != GameState::Running { return; }
        self.frame += 1;
        // ゲームロジック...
    }

    pub fn add_attack(&mut self, power: f32) {
        self.player_attacks.push_back(Attack { power, y: 750.0 });
    }

    pub fn get_state(&self) -> GameState { self.state }
    pub fn get_player_hp(&self) -> f32 { self.player_hp }
    pub fn get_enemy_hp(&self) -> f32 { self.enemy_hp }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    static mut GAME: Option<Game> = None;

    #[wasm_bindgen]
    pub fn game_init(player_hp: f32) {
        unsafe { GAME = Some(Game::new(player_hp)); }
    }

    #[wasm_bindgen]
    pub fn game_update() {
        unsafe {
            if let Some(game) = &mut GAME {
                game.update();
            }
        }
    }

    #[wasm_bindgen]
    pub fn game_add_attack(power: f32) {
        unsafe {
            if let Some(game) = &mut GAME {
                game.add_attack(power);
            }
        }
    }

    #[wasm_bindgen]
    pub fn game_get_enemy_hp() -> f32 {
        unsafe { GAME.as_ref().map(|g| g.get_enemy_hp()).unwrap_or(0.0) }
    }

    #[wasm_bindgen]
    pub fn game_get_player_hp() -> f32 {
        unsafe { GAME.as_ref().map(|g| g.get_player_hp()).unwrap_or(0.0) }
    }

    #[wasm_bindgen]
    pub fn game_get_state() -> u32 {
        unsafe {
            match GAME.as_ref().map(|g| g.get_state()) {
                Some(GameState::Running) => 0,
                Some(GameState::Victory) => 1,
                Some(GameState::Defeat) => 2,
                None => 0,
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() { println!("Game running..."); }
```

### HTML コード

```html
<!DOCTYPE html>
<html>
<head>
    <title>Game</title>
    <style>
        canvas { border: 1px solid #000; }
    </style>
</head>
<body>
    <canvas id="canvas" width="800" height="600"></canvas>

    <script type="module">
        import init, * as game from './sendai_daikannon_wasm/sendai_daikannon_lib.js';

        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');
        let gameRunning = false;

        async function startGame() {
            await init();
            game.game_init(100);
            gameRunning = true;
            gameLoop();
        }

        function gameLoop() {
            if (!gameRunning) return;

            game.game_update();
            const playerHp = game.game_get_player_hp();
            const enemyHp = game.game_get_enemy_hp();
            const state = game.game_get_state();

            // Canvas描画
            ctx.fillStyle = '#fff';
            ctx.fillRect(0, 0, 800, 600);
            ctx.fillStyle = '#000';
            ctx.fillText(`Enemy: ${enemyHp.toFixed(1)}`, 10, 20);
            ctx.fillText(`Player: ${playerHp.toFixed(1)}`, 10, 40);

            if (state !== 0) {
                gameRunning = false;
            }

            requestAnimationFrame(gameLoop);
        }

        startGame();
    </script>
</body>
</html>
```

## 参考リンク

- [wasm-bindgen](https://docs.rs/wasm-bindgen/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Rust and WebAssembly Book](https://rustwasm.org/book/)
