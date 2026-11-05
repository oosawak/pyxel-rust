"""Generate the self-contained, textured GLB demo/fixture without external assets."""
import json
from pathlib import Path
import struct
import zlib


def png_chunk(kind, data):
    return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data))


def build_glb():
    positions = [(-1, 0, -1), (1, 0, -1), (1, 0, 1), (-1, 0, 1), (0, 2, 0)]
    indices = [0, 4, 1, 1, 4, 2, 2, 4, 3, 3, 4, 0, 0, 1, 2, 0, 2, 3]
    uv = [(0, 0), (1, 0), (1, 1), (0, 1), (0.5, 0.5)]
    # A 2x2 opaque yellow/cyan checker texture; no third-party assets.
    raw = bytes([0, 233, 195, 91, 255, 118, 150, 222, 255,
                 0, 118, 150, 222, 255, 233, 195, 91, 255])
    png = (b'\x89PNG\r\n\x1a\n' + png_chunk(b'IHDR', struct.pack('>IIBBBBB', 2, 2, 8, 6, 0, 0, 0))
           + png_chunk(b'IDAT', zlib.compress(raw)) + png_chunk(b'IEND', b''))
    binary = bytearray()
    views = []

    def append(data, target=None):
        while len(binary) % 4:
            binary.append(0)
        view = {'buffer': 0, 'byteOffset': len(binary), 'byteLength': len(data)}
        if target:
            view['target'] = target
        views.append(view)
        binary.extend(data)
        return len(views) - 1

    pos_view = append(struct.pack('<15f', *(v for p in positions for v in p)), 34962)
    uv_view = append(struct.pack('<10f', *(v for p in uv for v in p)), 34962)
    idx_view = append(struct.pack('<18H', *indices), 34963)
    image_view = append(png)
    document = {
        'asset': {'version': '2.0', 'generator': 'pyxel-rust procedural fixture'},
        'scene': 0, 'scenes': [{'nodes': [0]}],
        # Offset parent/child exercise fitting and composed node transforms.
        'nodes': [{'name': 'Offset root', 'translation': [10, 0, 0], 'children': [1]},
                  {'name': 'Textured pyramid', 'translation': [0, 3, 0], 'mesh': 0}],
        'meshes': [{'primitives': [{'attributes': {'POSITION': 0, 'TEXCOORD_0': 1},
                                    'indices': 2, 'material': 0}]}],
        'materials': [{'doubleSided': True, 'pbrMetallicRoughness': {
            'baseColorTexture': {'index': 0}, 'metallicFactor': 0, 'roughnessFactor': 1}}],
        'textures': [{'source': 0}],
        'images': [{'bufferView': image_view, 'mimeType': 'image/png'}],
        'buffers': [{'byteLength': len(binary)}], 'bufferViews': views,
        'accessors': [
            {'bufferView': pos_view, 'componentType': 5126, 'count': 5, 'type': 'VEC3',
             'min': [-1, 0, -1], 'max': [1, 2, 1]},
            {'bufferView': uv_view, 'componentType': 5126, 'count': 5, 'type': 'VEC2'},
            {'bufferView': idx_view, 'componentType': 5123, 'count': 18, 'type': 'SCALAR'},
        ],
    }
    encoded = json.dumps(document, separators=(',', ':')).encode()
    encoded += b' ' * (-len(encoded) % 4)
    binary += b'\0' * (-len(binary) % 4)
    return (b'glTF' + struct.pack('<II', 2, 12 + 8 + len(encoded) + 8 + len(binary))
            + struct.pack('<I4s', len(encoded), b'JSON') + encoded
            + struct.pack('<I4s', len(binary), b'BIN\0') + binary)


if __name__ == '__main__':
    output = Path(__file__).resolve().parent.parent / 'docs/examples/retro3d/assets/pyramid.glb'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(build_glb())
    print(f'Generated {output.name}: {output.stat().st_size} bytes')
