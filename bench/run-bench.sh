#!/bin/sh
# Pyxel 3方式ベンチマーク実行スクリプト（Mac / Linux 共用）
# 使い方: このフォルダ（bench/）の中で  ./run-bench.sh  を実行するだけ。
# 初回は計測材料（site/）の組み立てと計測用 Chrome のダウンロードを行う。
# 結果は results.json と shot-*.png に出る。
set -e
cd "$(dirname "$0")"

CFT_VERSION="157.0.8103.38"
REPO_ROOT="$(cd .. && pwd)"
HP="https://oosawak.github.io/html-player"
PYXEL_WASM="https://cdn.jsdelivr.net/gh/kitao/pyxel@2.8.3/wasm"
PYODIDE="https://cdn.jsdelivr.net/pyodide/v0.29.3/full"

# ---- 1. 計測材料（site/）の組み立て ----
if [ ! -f site/.assembled ]; then
  echo "計測材料を組み立てます..."
  mkdir -p site/html-player/games site/html-player/assets/denki-chip \
           site/pyxel-rust/examples \
           site/pyxel-runtime/images site/pyxel-runtime/pyodide/v0.29.3
  for g in cube-boy line-boy cube-boy-pyx line-boy-pyx; do
    curl -sL -o "site/html-player/games/$g.tmp" "$HP/games/$g/index.html"
    mkdir -p "site/html-player/games/$g"
    mv "site/html-player/games/$g.tmp" "site/html-player/games/$g/index.html"
  done
  curl -sL -o site/html-player/assets/hp.js "$HP/assets/hp.js"
  curl -sL -o site/html-player/assets/fonts.css "$HP/assets/fonts.css"
  curl -sL -o site/html-player/assets/denki-chip/x8y12pxDenkiChip.woff2 \
    "$HP/assets/denki-chip/x8y12pxDenkiChip.woff2"
  # pyxel-rust のサンプルはこのリポジトリ自身から
  cp -r "$REPO_ROOT/docs/examples/cubeboy" "$REPO_ROOT/docs/examples/lineboy" site/pyxel-rust/examples/
  # 本家 Pyxel の Web 実行環境（Pyxel 本体＋Pyodide）を CDN から取得
  for f in pyxel.js pyxel.css import_hook.py pyxel-2.8.3-cp38-abi3-emscripten_4_0_9_wasm32.whl; do
    curl -sL -o "site/pyxel-runtime/$f" "$PYXEL_WASM/$f"
  done
  for i in pyxel_logo_76x32.png touch_to_start_114x14.png click_to_start_114x14.png \
           gamepad_cross_98x98.png gamepad_button_98x98.png gamepad_menu_92x26.png pyxel_icon_64x64.ico; do
    curl -sL -o "site/pyxel-runtime/images/$i" "$PYXEL_WASM/images/$i"
  done
  for f in pyodide.js pyodide.asm.js pyodide.asm.wasm python_stdlib.zip pyodide-lock.json; do
    curl -sL -o "site/pyxel-runtime/pyodide/v0.29.3/$f" "$PYODIDE/$f"
  done
  # ローカル配信向けの書き換え（CDN 参照を同一サーバー内に寄せる／ローカル用の CSP 緩和）
  sed -i.bak 's|https://cdn.jsdelivr.net/pyodide/v0.29.3/full/pyodide.js|/pyxel-runtime/pyodide/v0.29.3/pyodide.js|' site/pyxel-runtime/pyxel.js
  sed -i.bak 's|https://cdn.jsdelivr.net/gh/kitao/pyxel@2.8.3/wasm/pyxel.js|/pyxel-runtime/pyxel.js|' \
    site/html-player/games/cube-boy-pyx/index.html site/html-player/games/line-boy-pyx/index.html
  sed -i.bak "s|'wasm-unsafe-eval'|'wasm-unsafe-eval' 'unsafe-eval'|" \
    site/html-player/games/cube-boy-pyx/index.html site/html-player/games/line-boy-pyx/index.html
  sed -i.bak 's|https://oosawak.github.io/html-player/|/html-player/|g' \
    site/html-player/games/cube-boy/index.html site/html-player/games/line-boy/index.html \
    site/html-player/assets/hp.js site/html-player/assets/fonts.css
  sed -i.bak "s|default-src https://oosawak.github.io|default-src 'self' https://oosawak.github.io|" \
    site/html-player/games/cube-boy/index.html site/html-player/games/line-boy/index.html
  find site -name "*.bak" -delete
  touch site/.assembled
fi

# ---- 2. 計測用 Chrome ----
OS="$(uname -s)"; ARCH="$(uname -m)"
if [ -n "$CHROME_PATH" ] && [ -x "$CHROME_PATH" ]; then
  CHROME_BIN="$CHROME_PATH"
else
  if [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ]; then PLAT="mac-arm64"; else PLAT="mac-x64"; fi
    CHROME_DIR="chrome-$PLAT"
    CHROME_BIN="$PWD/$CHROME_DIR/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
  else
    PLAT="linux64"; CHROME_DIR="chrome-linux64"; CHROME_BIN="$PWD/$CHROME_DIR/chrome"
  fi
  if [ ! -x "$CHROME_BIN" ]; then
    echo "Chrome for Testing ($PLAT) をダウンロードします..."
    curl -sL -o "chrome-$PLAT.zip" "https://storage.googleapis.com/chrome-for-testing-public/$CFT_VERSION/$PLAT/chrome-$PLAT.zip"
    rm -rf "$CHROME_DIR"; unzip -q "chrome-$PLAT.zip"; rm -f "chrome-$PLAT.zip"
  fi
fi

# ---- 3. 依存とサーバー ----
if [ ! -d node_modules/puppeteer-core ]; then
  echo "puppeteer-core をインストールします..."
  npm install --no-fund --no-audit puppeteer-core
fi
SERVER_STARTED=""
if curl -s -o /dev/null "http://127.0.0.1:8917/html-player/games/cube-boy/index.html"; then
  echo "ローカルサーバーは起動済みです"
else
  echo "ローカルサーバーを起動します (127.0.0.1:8917)"
  python3 -m http.server 8917 --bind 127.0.0.1 --directory "$PWD/site" > server.log 2>&1 &
  SERVER_STARTED=$!
  sleep 1
fi
trap '[ -n "$SERVER_STARTED" ] && kill $SERVER_STARTED 2>/dev/null || true' EXIT

echo "計測を開始します（12回分、10分ほどかかります）"
CHROME_PATH="$CHROME_BIN" node bench.js
echo "完了: results.json / shot-*.png を確認してください"
