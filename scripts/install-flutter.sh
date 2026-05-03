#!/bin/bash
# Flutter SDK インストールスクリプト
# 実行: bash scripts/install-flutter.sh

set -e

FLUTTER_VERSION="3.27.4"
INSTALL_DIR="$HOME/flutter"

echo "=== Flutter $FLUTTER_VERSION インストール ==="

# すでにあれば確認（壊れてる場合は再インストール）
if [ -f "$INSTALL_DIR/bin/flutter" ]; then
    if [ -f "$INSTALL_DIR/bin/internal/shared.sh" ]; then
        echo "Flutter はすでにインストール済みです: $INSTALL_DIR"
        $INSTALL_DIR/bin/flutter --version
        exit 0
    else
        echo "Flutter が壊れています。削除して再インストールします..."
        rm -rf "$INSTALL_DIR"
    fi
fi

# ダウンロード
TARBALL="/tmp/flutter_linux_${FLUTTER_VERSION}-stable.tar.xz"
if [ ! -f "$TARBALL" ]; then
    echo "ダウンロード中..."
    curl -L "https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_${FLUTTER_VERSION}-stable.tar.xz" -o "$TARBALL"
fi

# 展開
echo "展開中..."
tar xf "$TARBALL" -C "$HOME"

# PATH 設定
SHELL_RC="$HOME/.bashrc"
if ! grep -q "flutter/bin" "$SHELL_RC"; then
    echo 'export PATH="$HOME/flutter/bin:$PATH"' >> "$SHELL_RC"
    echo "PATH を .bashrc に追加しました"
fi

export PATH="$HOME/flutter/bin:$PATH"

# 確認
echo "=== インストール完了 ==="
flutter --version

# 依存チェック
echo ""
echo "=== 環境チェック ==="
flutter doctor

echo ""
echo "次のコマンドでFlutter Webプロジェクトを作成してください:"
echo "  export PATH=\"\$HOME/flutter/bin:\$PATH\""
echo "  cd /home/oosawak/Workspace"
echo "  flutter create --platforms web pyxel_flutter_demo"
echo "  cd pyxel_flutter_demo"
echo "  flutter pub add webview_flutter webview_flutter_web"
