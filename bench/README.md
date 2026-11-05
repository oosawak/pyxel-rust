# bench — Pyxel 3方式ベンチマーク

CubeBoy と LINE BOY を、3つの実装方式でブラウザ実測する計測一式。

- 本家 Pyxel（kitao/pyxel、Python／Pyodide）
- pyxel-rust（このリポジトリ、Rust／WASM）
- 素の JavaScript（HTMLプレイヤー版）

## 回し方

前提：Node.js と Python 3。

```sh
cd bench
./run-bench.sh
```

初回は計測材料（`site/`）を自動で組み立て、計測用 Chrome（Chrome for Testing）をダウンロードする。以後は計測だけが走る。12回分で10分ほど。結果は `results.json`、最終画面は `shot-*.png`。

計測は、公開サイトではなくローカルサーバー（127.0.0.1:8917）から同一データを配信して行う（ネットワークのぶれを除くため）。材料のうち pyxel-rust のサンプルはこのリポジトリの `docs/examples/` から、他は公開ページと CDN から取得する。

個別に測りたいときは `ONLY=plain-js` のように環境変数で方式を絞れる（`pyxel-python`／`pyxel-rust`／`plain-js`）。

## 測るもの

- 転送量（読み込んだバイト数）
- 動き出すまでの時間（画面の変化で判定）
- 実行中の FPS と CPU 使用率（CDP の Performance メトリクス）
- ロングタスク件数

## 結果と注意

2026-11-06 の実測は [RESULTS.md](RESULTS.md) にある。

FPS について：ヘッドレス計測では JavaScript 版の提示フレーム数が低く出る癖がある（条件を変えると rAF は約62/秒で回ることを確認済み）。実機の体感速度として読まないこと。

また、同じゲームでもコードは方式ごとに別物なので、数字は「方式そのものの実力差」ではなく「この3つの実装の実測」として読むもの。
