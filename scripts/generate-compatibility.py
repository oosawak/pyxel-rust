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
ENGINE_REV = '2eecdce4ab12d360763d85e6c3cbe5142d99cb1c'
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
        if name in ['blt', 'bltm', 'text'] and backend != 'wgpu':
            token = {'blt':'blt_transform','bltm':'bltm_transform','text':'text_with_font'}[name]
            return result('supported', '基本関数に加え、blt_transform / bltm_transformで回転・拡大縮小、text_with_fontで任意Fontに対応します。既存の基本関数の引数は維持しています。', path, 'pub fn '+token, 'resources' if backend == 'wasm' else None)
        if name in ['blt', 'bltm', 'text']:
            return result('partial', '基本描画のみ。blt/bltmのrotate/scale、textの任意Font引数は公開していません。', path, 'pub fn '+name)
        if name == 'screen' and native:
            return result('supported', 'ルートのscreen()から公式画面イメージを取得できます。', lib, 'pub use pyxel::screen')
        if name == 'colors' and soft:
            return result('partial', '16色のパレットをset/get_palette_colorで操作。公式colors配列や多色拡張とは異なります。', path, 'pub fn set_palette_color')
        if name in ['images', 'tilemaps'] and backend != 'wgpu':
            return result('partial', '公式リソース型の共有ハンドルを返します。内容の編集は描画に即時反映します。返り値Vecの要素差し替え・バンク数変更は反映しません。', api+'resource.rs', 'pub fn '+name, 'resources' if backend == 'wasm' else None)
        if name in ['images', 'tilemaps']:
            return result('partial', '固定バンクのimage_* / tilemap_*関数のみ。公式リソース型は未公開です。', path, 'pub fn image_rect')
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
        if name in ['channels', 'tones', 'sounds', 'musics'] and backend != 'wgpu':
            return result('partial', '公式シンセサイザーが使う共有リソースハンドルを取得・編集できます。返り値Vecの要素差し替え・バンク数変更は反映しません。', api+'resource.rs', 'pub fn '+name, 'resources' if backend == 'wasm' else None)
        if name in ['play', 'playm', 'stop', 'play_pos']:
            if native:
                return result('supported', '公式SDL2コアの再生・停止・再生位置へ接続。', path, 'pub fn '+name)
            if backend == 'wasm':
                return result('supported', '公式シンセサイザーの出力をWeb Audioへ接続。sec/loop/resume、曲の複数チャンネル再生、停止、位置取得に対応します。初回はユーザー操作からresume_browser_audioが必要です。旧JSフックを設定したページではその再生処理を優先し、位置・状態は公式内蔵エンジンのみの情報です。', path, 'pub fn '+name, 'resources-audio')
            return result('missing', 'wgpuでは音声出力・再生制御が未接続です。', path, 'pub fn '+name)
        return missing(path)
    if category == 'Resource':
        return missing(api+'resource.rs')
    if category in ['Image', 'Tilemap', 'Sound', 'Music', 'Channel', 'Font', 'Tone']:
        if backend == 'wgpu':
            return result('missing', '公式リソース型はこの構成で公開されません。プレースホルダー型やバンク関数はクラス全体の実装ではありません。', api+'resource.rs')
        member = name.split('.')[-1]
        path = f'vendor/pyxel-cube-core/src/{category.lower()}.rs'
        if backend == 'wasm':
            if category == 'Tilemap' and member in ['from_tmx', 'load']:
                return result('missing', 'TMXのファイルパス読込はブラウザ用に接続していません。共有Tilemapの手動編集・描画は使えます。', path, 'pub fn '+member)
            if category == 'Font' and name == 'Font':
                return result('partial', 'Font::from_bytesでBDF/TTF/OTFを読み込みます。ファイルパスによるFont::newはブラウザで使えません。', path, 'pub fn from_bytes', 'resources')
            if category == 'Image' and member in ['from_image', 'load', 'save']:
                token = {'from_image':'from_image_bytes','load':'load_bytes','save':'png_bytes'}[member]
                return result('partial', 'バイト列から画像を読み込み、PNGバイト列として保存できます。ブラウザのファイル選択・ダウンロードはページ側で行います。filename指定・保存時scaleは未対応で、表示パレットは16色です。', path, 'pub fn '+token, 'resources')
            if category in ['Sound', 'Music'] and member == 'save':
                return result('partial', 'wav_bytesで指定秒数を公式シンセサイザーからWAV化し、ブラウザで保存できます。ファイルパス指定・FFmpegによる動画変換は使えません。', path, 'pub fn wav_bytes', 'resources-audio')
            if category == 'Sound' and member == 'pcm':
                loading = 'filename' in row['signature']
                return result('partial' if loading else 'supported', 'load_pcm_bytesで音声バイト列をデコードします。ファイルパス指定は使えません。' if loading else 'clear_pcmでPCMを解除します。', path, 'pub fn '+('load_pcm_bytes' if loading else 'clear_pcm'), 'resources-audio')
        if member == 'mml':
            return result('supported', '公式コアのset_mml / clear_mmlとして公開しています。', path, 'pub fn set_mml', 'resources-audio' if backend == 'wasm' else None)
        if member == 'pcm':
            return result('supported', '公式コアのload_pcm / clear_pcmとして公開しています。', path, 'pub fn load_pcm')
        return result('supported', '公式リソース型のメソッド／フィールドを公開しています。共有バンクの編集は描画・音声エンジンへ反映します。Python名はRustのpixel/set_pixel、tile/set_tile、play_position、total_seconds等に読み替えます。ファイルI/Oの制限は別項目に記載しています。', path, verification='resources' if backend == 'wasm' and name in ['Tilemap','Tilemap.imgsrc','Tilemap.pget','Tilemap.pset','Font.text_width','Sound','Sound.total_sec','Music','Music.set','Music.seqs','Channel','Channel.gain','Channel.play','Channel.stop','Channel.play_pos'] else None)
    return result('unreviewed', 'API単位の対応状況は未確認です。', lib)


def build():
    baseline = json.loads(BASELINE.read_text())
    rows = baseline['entries']
    for row in rows:
        row['backends'] = {b: classify(row, b) for b in BACKENDS}
    # Rust-specific helpers expose important stubs not present in official docs.
    for name, path, assessments in [
        ('is_playing', 'src/api/audio.rs', {b: ('missing' if b == 'wgpu' else 'supported', 'wgpuの音声出力は未接続です。' if b == 'wgpu' else '内蔵エンジンの実際の再生位置から判定します。旧JSフックだけで再生する音声の状態は含みません。') for b in BACKENDS}),
        ('set_palette_color / get_palette_color', 'src/api/graphics.rs', {b: ('supported', '表示パレットの読み書きへ接続。Canvasでは画像のPNG保存にも反映します。') for b in BACKENDS}),
        ('image_* / tilemap_*', 'src/api/graphics.rs', {b: ('partial' if b == 'wgpu' else 'supported', 'wgpuではバンクの一部操作のみ。公式クラス型は未公開です。' if b == 'wgpu' else '公式の共有画像・タイルマップバンクに接続しています。') for b in BACKENDS}),
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
