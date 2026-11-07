"""Build the reviewed Pyxel 3.0 API inventory. --check detects stale reports.

The baseline contains every API-reference heading, expanded drawing groups and
individual constants. Status rules are reviewed mappings, not claims inferred
from symbol presence. Runtime verification is recorded separately.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASELINE = ROOT / 'scripts/pyxel-3.0-api.json'
OUTPUT = ROOT / 'docs/assets/compatibility/data.json'
ENGINE_REV = 'bd82b72206511e75844d1b4f97c928c7520b486d'
BACKENDS = ['wasm', 'sdl2', 'wgpu']


def read(path):
    return (ROOT / path).read_text()


def evidence(path, token=''):
    lines = read(path).splitlines()
    line = next((i + 1 for i, value in enumerate(lines) if token and token in value), 1)
    return f'https://github.com/oosawak/pyxel-rust/blob/{ENGINE_REV}/{path}#L{line}'


def result(status, note, path, token='', verification=None):
    return dict(status=status, note=note, source=evidence(path, token), verification=verification)


def classify(row, backend):
    name, category = row['name'], row['category']
    native = backend == 'sdl2'
    soft = not native
    lib = 'src/lib.rs'
    api = 'src/api/'
    missing = lambda path=lib: result('missing', 'pyxel-rustの公開APIに対応する入口がありません。依存先に存在するだけの機能は対応に数えていません。', path)
    if row['group'] == 'Cube / 3D':
        if backend == 'wgpu':
            return result('missing', 'wgpu構成ではCubeの型と描画APIが公開されません。', lib, 'pub use pyxel::cube')
        snake = {'Vec3': 'vec3', 'Mat4': 'mat4', 'Quat': 'quat', 'RaycastHit': 'raycast_hit'}.get(category, category.lower())
        path = f'vendor/pyxel-cube-core/src/cube/{snake}.rs'
        member = name.split('.')[-1]
        if category == 'Node':
            if member in ['on_update', 'on_draw', 'on_collide', 'on_destroy', 'update', 'draw', 'from_mesh']:
                return result('missing', 'Nodeのデータ型は公開していますが、Pythonバインディングが担う生成・コールバック・更新描画のライフサイクルは移植していません。', path, 'pub struct Node')
            if member in ['apply_motion', 'play_motion', 'stop_motion']:
                return result('partial', 'MeshAnimationPlayerでモデルのアニメーションは操作できます。NodeのモーションAPIと自動更新は未実装です。', api+'animation3d.rs', 'pub struct MeshAnimationPlayer', 'animation' if backend == 'wasm' else None)
            if member in ['raycast', 'raycast_all', 'overlap_sphere', 'overlap_box']:
                return result('supported', f'cube::Scene::{member}に相当する公式コア処理を公開しています。Node.updateとの自動連携はありません。', 'vendor/pyxel-cube-core/src/cube/scene.rs', f'pub fn {member}(')
            state = ['dither', 'depth_test', 'depth_write', 'depth_offset', 'decal', 'shaded']
            draw = ['pset', 'line', 'tri', 'trib', 'rect', 'rectb', 'circ', 'circb', 'elli', 'ellib', 'box', 'boxb', 'sphere', 'sphereb', 'plane', 'sprite', 'prim', 'text']
            if member in state + draw:
                return result('partial', 'cube::drawとDrawContext / DrawStateを使う明示的な描画に対応します。Nodeメソッドとしての呼出しや継承状態の自動設定は未対応です。', 'src/api/graphics3d.rs', 'pub fn render_3d')
        if category == 'Mesh' and member == 'from_glb':
            return result('partial' if soft else 'supported', 'GLBバイト列から読込。画像・バッファはBIN内への埋め込みが必要です。ファイルパス・外部URIは使えません。' if soft else '公式Mesh::from_glbを公開。公式3.0.0自身がスキニング・モーフに非対応です。', path, 'pub fn from_glb', 'glb' if backend == 'wasm' else None)
        if category == 'Mat4' and member in ['pos', 'rot', 'scale']:
            return result('partial', '読み取りメソッドは公開しています。Pythonの書込可能なプロパティと同じsetterはなく、行列の更新が必要です。', path, 'pub fn pos')
        if category == 'Shading' and member == 'palette':
            return result('unreviewed', 'Shadingコア型は公開済みですが、このPythonプロパティとの等価性は未確認です。', path)
        return result('supported', '公式Cubeコアの型・演算をRustから公開。Pythonのプロパティや定数にはRustメソッドでの相当操作を含みます。個別の動作テストは未実施です。', path)
    if name.isupper():
        constants = read(api+'constants.rs')
        declared = re.search(r'pub const '+re.escape(name)+r'\b', constants)
        exported = native and re.search(r'\b'+re.escape(name)+r'\b', constants.split('mod keys_pyxel {', 1)[1])
        if not (declared or exported):
            return missing(api+'constants.rs')
        note = 'この名前の定数を公開しています。入力イベントや個別デバイスの動作確認とは別です。'
        if soft and name.startswith(('KEY_', 'MOUSE_', 'GAMEPAD')):
            note += 'キー番号はバックエンド固有です。ブラウザは対応する物理キーと標準マッピングのゲームパッド4台に接続します。wgpuのゲームパッド入力は未接続です。'
        return result('supported', note, api+'constants.rs', name)
    if category == 'System':
        path = api+'system.rs'
        if name in ['width', 'height', 'frame_count', 'run', 'quit']:
            return result('supported', 'バックエンドへ接続しています。終了はイベントループ停止要求です。' if name == 'quit' else 'バックエンドへ接続しています。', path, 'pub fn '+name)
        if name == 'init':
            return result('partial', 'width/height/title/fpsのみ公開。quit_key、display_scale、キャプチャ設定、headless等は指定できません。', path, 'pub fn init')
        return missing(path)
    if category == 'Input':
        path = api+'input.rs'
        if name == 'btnp':
            return result('supported', 'btnp / btnp_holdで単発・ゲームフレーム数に基づくhold/repeatを処理します。', path, 'pub fn btnp_hold', 'browser-input' if backend == 'wasm' else None)
        if name in ['btn', 'btnr', 'mouse_x', 'mouse_y']:
            return result('supported', 'キーボード／マウス座標のバックエンドに接続。ゲームパッド等の入力経路すべての対応を意味しません。', path, 'pub fn '+name, 'browser-input' if backend == 'wasm' else None)
        if name in ['mouse_wheel', 'mouse']:
            return result('supported', '公式コアへ接続。' if native else 'マウスイベント／カーソル表示に接続。ホイールは方向を整数ステップに正規化し、毎フレームリセットします。', path, 'pub fn '+name, 'browser-input' if backend == 'wasm' else None)
        if name in ['btnv', 'set_btn', 'set_btnv', 'input_keys']:
            return result('supported', '公式コアへ接続。' if native else 'デジタル・アナログ入力状態とキー押下一覧を読み書きできます。ブラウザの標準ゲームパッドは毎フレーム取得し、切断時に解除します。wgpuのゲームパッドイベントは未接続です。', path, 'pub fn '+name)
        if name == 'set_mouse_pos':
            return result('supported' if native else 'partial', '公式コアでカーソル位置を変更します。' if native else '論理座標の変更に対応します。ブラウザ／wgpuのOSカーソルは移動しません。次のマウス移動で座標が更新されます。', path, 'pub fn '+name)
        return missing(path)
    if category == 'Graphics':
        path = api+'graphics.rs'
        if name in ['clip', 'camera', 'pal', 'dither', 'cls', 'pget', 'pset', 'line']:
            return result('supported', '描画バックエンドへ接続。引数なしのリセットは*_reset()を使用します。', path, 'pub fn '+name)
        if name in ['rectb', 'circb']:
            return result('supported', '公式名の輪郭描画APIを公開しています。', path, 'pub fn '+name, 'drawing' if backend == 'wasm' else None)
        if name in ['elli', 'ellib', 'tri', 'trib', 'fill'] and backend != 'wgpu':
            return result('supported', '公式コアのラスタライザーで楕円・三角形・塗りつぶしを描画。既存画素・クリップ・カメラ・ディザを維持します。', path, 'pub fn '+name, 'drawing' if backend == 'wasm' else None)
        if name in ['rect', 'circ']:
            return result('partial', '公式と名前が異なります。塗りつぶしはrectfill/circfill、輪郭はrect/circです。公式名をそのまま移植できません。', path, 'pub fn '+('rect' if name.startswith('rect') else 'circ'))
        if name in ['blt', 'bltm', 'text']:
            return result('partial', '基本描画のみ。blt/bltmのrotate/scale、textの任意Font引数は公開していません。', path, 'pub fn '+name)
        if name == 'screen' and native:
            return result('supported', 'ルートのscreen()から公式画面イメージを取得できます。', lib, 'pub use pyxel::screen')
        if name == 'colors' and soft:
            return result('partial', '16色のパレットをset/get_palette_colorで操作。公式colors配列や多色拡張とは異なります。', path, 'pub fn set_palette_color')
        if name in ['images', 'tilemaps'] and soft:
            return result('partial', '固定バンクをimage_* / tilemap_*関数で部分的に操作できます。公式リソース配列は未公開です。', path, 'pub fn image_rect')
        return missing(path)
    if category == 'Math':
        path = api+'math.rs'
        if name in ['sgn', 'sqrt', 'sin', 'cos', 'floor', 'ceil', 'atan2', 'clamp']:
            return result('supported', 'Rustの実装あり。floor等の戻り値型はPython APIと異なります。', path, 'pub fn '+name, 'math' if name in ['ceil', 'atan2'] and backend != 'wgpu' else None)
        if name in ['rseed', 'rndi', 'rndf']:
            return result('supported' if backend != 'wgpu' else 'partial', '公式コアの乱数生成器に接続。同一seedの整数・小数の列を比較テスト済みです。' if backend != 'wgpu' else '独自xorshift乱数で公式の乱数列とは一致しません。', path, 'pub fn '+name, 'math' if backend != 'wgpu' else None)
        if name in ['nseed', 'noise'] and backend != 'wgpu':
            return result('supported', '公式コアのPerlinノイズに接続。noiseのy/zはOptionで省略でき、既定値は0です。', path, 'pub fn '+name, 'math')
        return missing(path)
    if category == 'Audio':
        path = api+'audio.rs'
        if name in ['play', 'playm', 'stop']:
            if native:
                return result('supported', '公式コアの音声再生へ接続。今回のブラウザテストで音声出力は検証していません。', path, 'pub fn '+name)
            if backend == 'wasm' and name != 'playm':
                return result('partial', 'ページ側のpyxelPlaySound / pyxelStopSoundフックが必要です。playはsec/loop/resumeを渡しません。このデモは音声フックを用意していません。', path, 'pub fn '+name)
            return result('missing', '関数名は存在しますが、このバックエンドでは処理が空です。', path, 'pub fn '+name)
        return missing(path)
    if category == 'Resource':
        return missing(api+'resource.rs')
    if category in ['Image', 'Tilemap', 'Sound', 'Music', 'Channel', 'Font', 'Tone']:
        if category in ['Tone', 'Font']:
            return result('missing', f'{category}型をpyxel-rustの公開APIから再公開していません。', lib)
        if not native and (backend != 'wasm' or category != 'Image'):
            return result('missing', '公式リソース型はこの構成で公開されません。Image/Tilemapのプレースホルダー型やバンク関数はクラス全体の実装ではありません。', api+'resource.rs')
        member = name.split('.')[-1]
        path = f'vendor/pyxel-cube-core/src/{category.lower()}.rs'
        if not native and member in ['from_image', 'load', 'save']:
            return result('missing', 'Image型にファイルI/Oメソッドは含まれますが、ブラウザ用読込／保存に接続していません。', path, 'pub fn '+member)
        if member in ['mml', 'pcm']:
            return result('supported', '公式コアのset_mml/clear_mml、load_pcm/clear_pcmとして公開。ファイル読込には実ファイルが必要です。', path)
        return result('supported', 'ルートで再公開した公式コア型のメソッド／フィールドを使用します。Pythonの名称はRustのpixel/set_pixel等に読み替えます。ブラウザImageはオフスクリーン用で描画バンクと別です。', path)
    return result('unreviewed', 'API単位の対応状況は未確認です。', lib)


def build():
    baseline = json.loads(BASELINE.read_text())
    rows = baseline['entries']
    for row in rows:
        row['backends'] = {b: classify(row, b) for b in BACKENDS}
    # Rust-specific helpers expose important stubs not present in official docs.
    for name, path, assessments in [
        ('is_playing', 'src/api/audio.rs', {b: ('missing', '全構成で常にfalseを返すスタブです。再生状態の確認に使えません。') for b in BACKENDS}),
        ('set_palette_color / get_palette_color', 'src/api/graphics.rs', {b: ('missing' if b == 'sdl2' else 'supported', 'SDL2ではsetterが空、getterが常に0です。' if b == 'sdl2' else '16色パレットの読み書きを実装しています。') for b in BACKENDS}),
        ('image_* / tilemap_*', 'src/api/graphics.rs', {b: ('missing' if b == 'sdl2' else 'partial', 'SDL2側の画像・タイルマップ補助関数は空処理または固定値です。' if b == 'sdl2' else 'バンクの一部操作のみ。公式クラス全体を提供するものではありません。') for b in BACKENDS}),
    ]:
        rows.append(dict(id='extension:'+name, name=name, signature=name, category='Rust extensions', group='Rust拡張', reference=None, backends={b: result(*assessments[b], path) for b in BACKENDS}))
    source_paths = sorted({str(p.relative_to(ROOT)) for folder in ['src/api', 'src/backend', 'vendor/pyxel-cube-core/src'] for p in (ROOT / folder).rglob('*.rs')} | {'src/lib.rs', 'Cargo.toml', 'Cargo.lock'})
    return dict(schema=1, version=baseline['version'], upstream=baseline['upstream'], engineRevision=ENGINE_REV,
                scope='公式3.0.0の2D/共通・Cube APIリファレンスの全見出し（描画グループと定数は個別展開）。CLI・エディタUI・非公開APIは対象外。',
                sourceHashes={path: hashlib.sha256((ROOT / path).read_bytes()).hexdigest() for path in source_paths}, entries=rows)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='Fail if the report differs from the reviewed rules or engine source')
    args = parser.parse_args()
    encoded = json.dumps(build(), ensure_ascii=False, indent=2)+'\n'
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != encoded:
            raise SystemExit('Compatibility report is stale. Review the changed engine/API and update the rules before regenerating.')
    else:
        OUTPUT.parent.mkdir(parents=True, exist_ok=True)
        OUTPUT.write_text(encoded)
    print(f'Compatibility inventory: {len(json.loads(encoded)["entries"])} entries, 3 backends; '+('current' if args.check else 'generated'))
