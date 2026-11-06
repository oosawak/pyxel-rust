export const examples = [
  { slug: 'c01-hello-cube', title: 'Hello Cube', description: '6個の立方体が公転・回転する最初のサンプル。', size: [200,150], help: '回転・公転を確認。', source: 'c01_hello_cube.py' },
  { slug: 'c02-basic-shapes', title: 'Basic Shapes', description: '10種類の基本図形、テクスチャ、ビルボード、輪郭線。', size: [240,240], help: 'マウス：カメラ回転 / Space：ワイヤー表示（押している間）', source: 'c02_basic_shapes.py' },
  { slug: 'c03-custom-shapes', title: 'Custom Shapes', description: '変形する敵をロックオンし、曲がるレーザーを発射。', size: [256,192], help: '敵の上をドラッグ：ロック / 離す：発射 / マウスの高さ：カメラ調整', source: 'c03_custom_shapes.py' },
  { slug: 'c04-mesh-and-motion', title: 'Mesh and Motion', description: 'ガゼルのGLBモデルと歩行・走行・食事の3モーション。', size: [320,240], help: '← → ↑ ↓：カメラ / 1・2・3：モーション / S：陰影', source: 'c04_mesh_and_motion.py' },
  { slug: 'c05-3d-collision', title: '3D Collision', description: '庭園を歩き、ジャンプしてコインと山頂を目指す。', size: [320,240], help: '← → ↑ ↓：移動 / Space：2段ジャンプ / S：陰影 / R：開始位置へ', source: 'c05_3d_collision.py' },
  { slug: 'c06-3d-physics', title: '3D Physics', description: '積み木、球、カプセル、回転する足場と投射体。', size: [320,240], help: 'マウス：照準 / クリック・Space：発射 / 矢印：カメラ / R：リセット', source: 'c06_3d_physics.py' },
];
export const officialUrl = example => `https://kitao.github.io/pyxel/web/showcase/examples/${example.slug}.html`;
