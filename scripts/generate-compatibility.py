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
ENGINE_REV = '4c19225518b21ba314e2fb80b5266ef3ad7b6f86'
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
                return result('supported', 'NodeSceneでRustクロージャを登録し、階層の更新・物理・衝突・子から親への破棄通知と描画を実行します。from_meshは名前・親子関係・材質・形状・Motionのモデル対応を変換します。drawはカメラ・可視性を継承し、カスタム描画にはwith_node_draw_stateで不透明度・シェーディングを適用します。Pythonの継承の代わりに明示的な登録を使用し、同じシーンの再入は拒否します。updateは更新通知→NodeごとのMotion再生→物理→衝突→破棄の順に実行します。Motionはモデル個体ごとに進み、元Meshを書き換えません。', api+'scene3d.rs', 'pub fn '+member, 'scene-lifecycle')
            if member in ['apply_motion', 'play_motion', 'stop_motion']:
                return result('supported', 'NodeSceneのapply_motion / play_motion / stop_motionで、同じMeshに属するMotionを個体・部分木ごとに適用します。updateごとにspeedフレーム進め、逆再生・速度0・ループ・終了・inactiveによる停止に対応します。停止時の姿勢を保持し、再生開始時はアニメーション対象だけを原姿勢へ戻します。', api+'scene3d/motion.rs', 'pub fn '+member, 'node-motion')
            if member in ['raycast', 'raycast_all', 'overlap_sphere', 'overlap_box']:
                return result('supported', f'cube::Scene::{member}に相当する公式コア処理を公開しています。NodeSceneのrootへ同じ空間クエリーを適用できます。', 'vendor/pyxel-cube-core/src/cube/scene.rs', f'pub fn {member}(')
            state = ['dither', 'depth_test', 'depth_write', 'depth_offset', 'decal', 'shaded']
            draw = ['pset', 'line', 'tri', 'trib', 'rect', 'rectb', 'circ', 'circb', 'elli', 'ellib', 'box', 'boxb', 'sphere', 'sphereb', 'plane', 'sprite', 'prim', 'text']
            if member in state + draw:
                return result('partial', 'cube::drawとDrawContext / DrawStateを使います。NodeSceneのon_drawではwith_node_draw_stateを使用すると親の不透明度・シェーディングを継承できます。Python Nodeメソッドと同じ呼出し形式ではありません。', 'src/api/scene3d.rs', 'pub fn with_node_draw_state')
        if category == 'Mesh' and member == 'from_glb':
            return result('partial' if soft else 'supported', 'GLBバイト列から読込。画像・バッファはBIN内への埋め込みが必要です。ファイルパス・外部URIは使えません。' if soft else '公式Mesh::from_glbを公開。公式3.0.0自身がスキニング・モーフに非対応です。', path, 'pub fn from_glb', 'glb' if backend == 'wasm' else None)
        if category == 'Mat4' and member in ['pos', 'rot', 'scale']:
            return result('supported', '公式3.0.0 Pythonバインディングもgetterのみの読み取り専用プロパティです。Rustはpos() / rot() / scale_vec()で同じ分解値を取得します。返された値の変更は元の行列を書き換えません。以前の書込setter不足という判定を訂正しました。', path, 'pub fn '+('scale_vec' if member == 'scale' else member), 'matrix-views')
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
        if name in ['title', 'icon', 'fullscreen', 'resize', 'perf_monitor', 'integer_scale'] and backend != 'wgpu':
            note = '公式SDL2コアのウィンドウ操作へ接続します。' if native else 'document/Canvasのウィンドウ操作へ接続します。全画面はユーザー操作・ブラウザの許可が必要です。整数倍は利用可能な幅で計算し、ページ側のCSSに従います。resizeは論理画面を再作成します。'
            return result('supported', note, path, 'pub fn '+name, 'browser-system' if backend == 'wasm' else None)
        if name in ['show','flip','reset','screen_mode'] and backend != 'wgpu':
            note = {'show':'showで現在の画面を表示し続けます。ブラウザでは非同期ループとして直ちに戻ります。', 'flip':'SDL2は公式flip_screen。ブラウザはflip().awaitでrequestAnimationFrameごとに表示します。run/showとの同時実行は拒否し、fps指定の同期待機とは異なります。', 'reset':'ブラウザはページ全体を再読込します。SDL2はset_reset_handlerでアプリ側の再起動処理を登録してから呼びます。未登録時はエラーを返します。', 'screen_mode':'SDL2は公式の3つのシェーダー。ブラウザは通常・平滑化・走査線/周辺減光の簡易Canvas表示で、公式xBR/CRTと同じ画質ではありません。'}[name]
            return result('partial' if backend == 'wasm' or name == 'reset' else 'supported', note, path, 'pub '+('async ' if backend == 'wasm' and name == 'flip' else '')+'fn '+name, 'screen-tools' if backend == 'wasm' else None)
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
        if name in ['input_text', 'set_input_text', 'dropped_files', 'set_dropped_files']:
            note = '公式SDL2コアの文字入力・ドロップ情報へ接続します。' if native else 'フレーム単位の文字入力・ドロップ情報を読み書きできます。Canvasはbrowser_text_inputでIME用入力欄を有効にでき、通常キーも取得します。wgpuはwinitの文字・IME・ドロップイベントを取得します。'
            if name == 'dropped_files' and backend == 'wasm':
                return result('partial', note+' ブラウザでは絶対パスを取得できず、名前とdropped_file_bytesのメモリ内容を使用します。', path, 'pub fn '+name, 'browser-system')
            return result('supported', note, path, 'pub fn '+name, 'browser-system' if backend == 'wasm' else None)
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
        if name in ['screen','cursor','font'] and backend != 'wgpu':
            return result('supported', '実際の画面・カーソル・標準フォントの共有Imageを返します。直接編集は描画へ反映されます。Canvasのscreenはリサイズ後も同じハンドルです。日本語のCanvas重ね描画はscreenの論理画素に含まれません。', api+'resource.rs', 'pub fn '+name, 'screen-tools' if backend == 'wasm' else None)
        if name in ['blt3d','bltm3d'] and backend != 'wgpu':
            return result('supported', '公式Imageの透視投影ラスタライザーで画像・タイルマップを画面へ描画します。位置・回転・FOV・透過色を指定できます。', path, 'pub fn '+name, 'screen-tools' if backend == 'wasm' else None)
        if name == 'colors' and backend != 'wgpu':
            return result('supported', 'colors()の可変参照から公式パレットを編集できます。Canvasも最大256色の表示と描画時pal変換に対応します。', api+'resource.rs', 'pub fn colors', 'browser-system' if backend == 'wasm' else None)
        if name == 'colors' and soft:
            return result('partial', '16色のパレットをset/get_palette_colorで操作。公式colors配列や多色拡張とは異なります。', path, 'pub fn set_palette_color')
        if name in ['images', 'tilemaps'] and backend != 'wgpu':
            return result('supported', '共有ハンドルから内容を編集し、with_images / with_tilemapsでバンク配列の差し替え・増減も反映できます。', api+'resource.rs', 'pub fn with_'+name, 'browser-system' if backend == 'wasm' else None)
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
            return result('supported', '公式シンセサイザーの共有ハンドルを編集できます。with_'+name+'でバンク配列の差し替え・増減も反映できます。', api+'resource.rs', 'pub fn with_'+name)
        if name == 'gen_bgm' and backend != 'wgpu':
            return result('supported', '公式BGM生成器でseed・preset・transpose・instrumentationからMMLを生成します。play=trueで公式シンセサイザーへ接続します。ブラウザ音声の初回有効化にはユーザー操作が必要です。', path, 'pub fn gen_bgm', 'browser-system' if backend == 'wasm' else None)
        if name in ['play', 'playm', 'stop', 'play_pos']:
            if native:
                return result('supported', '公式SDL2コアの再生・停止・再生位置へ接続。', path, 'pub fn '+name)
            if backend == 'wasm':
                return result('supported', '公式シンセサイザーの出力をWeb Audioへ接続。sec/loop/resume、曲の複数チャンネル再生、停止、位置取得に対応します。初回はユーザー操作からresume_browser_audioが必要です。旧JSフックを設定したページではその再生処理を優先し、位置・状態は公式内蔵エンジンのみの情報です。', path, 'pub fn '+name, 'resources-audio')
            return result('missing', 'wgpuでは音声出力・再生制御が未接続です。', path, 'pub fn '+name)
        return missing(path)
    if category == 'Resource':
        path = api+'resource.rs'
        if name in ['load', 'save', 'load_pal', 'save_pal', 'screenshot'] and backend != 'wgpu':
            if native:
                return result('supported', '公式SDL2コアのファイル操作へ接続します。', path, 'pub fn '+name)
            token = {'load':'load_bytes', 'save':'save_bytes', 'load_pal':'load_pal_text', 'save_pal':'save_pal_text', 'screenshot':'browser_screenshot'}[name]
            source = 'src/backend/wasm_backend/system.rs' if name == 'screenshot' else path
            return result('partial', 'メモリI/Oの'+token+'を使用します。pyxresは公式ZIP/TOML（画像・タイルマップ・従来音声・曲配列）、パレットは別のテキスト、画面は倍率指定PNGです。任意ファイルパスにはアクセスできず、選択・保存UIはページ側で行います。MML・PCM・Tone・Font・Nodeはpyxresに含みません。', source, 'pub '+('async ' if name == 'screenshot' else '')+'fn '+token, 'browser-system')
        if name in ['user_data_dir','screencast','reset_screencast'] and backend != 'wgpu':
            if native:
                return result('supported', '公式SDL2コアのユーザーディレクトリ・GIF録画操作へ接続します。', path, 'pub fn '+name)
            if name == 'user_data_dir':
                return result('partial', 'user_data_store(vendor,app)のget/set/removeで名前空間ごとのlocalStorageに永続化します。ファイルシステムのディレクトリやパスは提供できず、容量・利用可否はブラウザに従います。', 'src/backend/wasm_backend/storage.rs', 'pub fn user_data_store', 'screen-tools')
            return result('supported' if name == 'reset_screencast' else 'partial', 'configure_screencastで明示的に録画を開始し、screencast_gifで公式エンコーダーのGIFバイト列を保存します。直近フレームのリングバッファは32MiB/最大30秒、倍率1..16です。reset_screencastで内容をクリアします。Canvasの日本語重ね描画・カーソル・表示効果は含まず、リサイズ時は録画を終了します。', 'src/backend/wasm_backend/recording.rs', 'pub fn '+('reset_screencast' if name == 'reset_screencast' else 'screencast_gif'), 'screen-tools')
        return missing(path)
    if category in ['Image', 'Tilemap', 'Sound', 'Music', 'Channel', 'Font', 'Tone']:
        if backend == 'wgpu':
            return result('missing', '公式リソース型はこの構成で公開されません。プレースホルダー型やバンク関数はクラス全体の実装ではありません。', api+'resource.rs')
        member = name.split('.')[-1]
        path = f'vendor/pyxel-cube-core/src/{category.lower()}.rs'
        if backend == 'wasm':
            if category == 'Tilemap' and member in ['from_tmx', 'load']:
                return result('partial', 'from_tmx_text / load_tmx_textでTMX文字列を読み込みます。埋め込みタイルセット・8×8タイル・CSV層に対応。ファイルパス指定は使えません。', path, 'pub fn '+('from_tmx_text' if member == 'from_tmx' else 'load_tmx_text'), 'browser-system')
            if category == 'Font' and name == 'Font':
                return result('partial', 'Font::from_bytesでBDF/TTF/OTFを読み込みます。ファイルパスによるFont::newはブラウザで使えません。', path, 'pub fn from_bytes', 'resources')
            if category == 'Image' and member in ['from_image', 'load', 'save']:
                token = {'from_image':'from_image_bytes','load':'load_bytes','save':'png_bytes'}[member]
                return result('partial', 'バイト列から画像を読み込み、PNGバイト列として保存できます。ブラウザのファイル選択・ダウンロードはページ側で行います。filename指定・画像保存時scaleは未対応です。表示パレットは最大256色で、画面全体の倍率PNGはbrowser_screenshotを使用します。', path, 'pub fn '+token, 'resources')
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
