"""Regression checks for coverage and misleading support claims."""
import importlib.util
import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('inventory', ROOT / 'scripts/generate-compatibility.py')
inventory = importlib.util.module_from_spec(spec)
spec.loader.exec_module(inventory)


class CompatibilityTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = inventory.build()

    def find(self, name, backend):
        matches = [row['backends'][backend] for row in self.data['entries'] if row['name'] == name]
        self.assertTrue(matches, name)
        return matches[0]

    def test_every_documented_api_is_reported_for_every_backend(self):
        baseline = json.loads(inventory.BASELINE.read_text())
        expected = {row['id'] for row in baseline['entries']}
        rows = self.data['entries']
        self.assertEqual(len({row['id'] for row in rows}), len(rows))
        self.assertEqual(expected, {row['id'] for row in rows if row['group'] != 'Rust拡張'})
        self.assertEqual(len(expected), 738)
        self.assertIn('Image.fill', [row['name'] for row in rows])
        self.assertIn('GAMEPAD4_BUTTON_DPAD_RIGHT', [row['name'] for row in rows])
        for row in rows:
            self.assertEqual(set(row['backends']), {'wasm', 'sdl2', 'wgpu'})
            for assessment in row['backends'].values():
                self.assertIn(assessment['status'], ['supported', 'partial', 'missing', 'unreviewed'])
                self.assertTrue(assessment['note'])
                self.assertTrue(assessment['source'].startswith('https://github.com/oosawak/pyxel-rust/blob/'))

    def test_empty_audio_functions_are_not_counted_as_support(self):
        self.assertEqual(self.find('playm', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('play', 'wasm')['status'], 'supported')
        for backend in ['sdl2','wasm']:
            self.assertEqual(self.find('is_playing', backend)['status'], 'supported')
        self.assertEqual(self.find('is_playing','wgpu')['status'],'missing')
        self.assertEqual(self.find('stop', 'wgpu')['status'], 'missing')
        self.assertEqual(self.find('playm', 'sdl2')['status'], 'supported')

    def test_semantic_and_backend_limitations_are_visible(self):
        self.assertEqual(self.find('rect', 'wasm')['status'], 'partial')
        self.assertEqual(self.find('btnp', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('mouse_wheel', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('Node.update', 'wasm')['status'], 'missing')
        self.assertEqual(self.find('Node.raycast', 'wgpu')['status'], 'missing')
        self.assertEqual(self.find('Mesh.from_glb', 'wasm')['status'], 'partial')
        self.assertIsNone(self.find('Mesh.from_glb', 'sdl2')['verification'])
        self.assertEqual(self.find('Tilemap', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('Font', 'sdl2')['status'], 'supported')

    def test_new_api_support_preserves_real_platform_limits(self):
        for name in ['elli','ellib','tri','trib','fill','ceil','atan2','noise','nseed','rndi','btnv','set_btn','input_keys','mouse']:
            self.assertEqual(self.find(name,'wasm')['status'],'supported',name)
        self.assertEqual(self.find('set_mouse_pos','wasm')['status'],'partial')
        self.assertEqual(self.find('noise','wgpu')['status'],'missing')
        self.assertEqual(self.find('tri','wgpu')['status'],'missing')
        self.assertEqual(self.find('rndi','wgpu')['status'],'partial')
        self.assertIn('未接続',self.find('GAMEPAD4_BUTTON_DPAD_RIGHT','wgpu')['note'])
        for backend in inventory.BACKENDS:
            self.assertEqual(self.find('KEY_F24',backend)['status'],'supported')
            self.assertEqual(self.find('MOUSE_POS_X',backend)['status'],'supported')

    def test_resources_do_not_hide_browser_io_and_backend_gaps(self):
        for name in ['Tilemap','Sound','Music','Channel','Tone','Font.text_width','play_pos']:
            self.assertEqual(self.find(name,'wasm')['status'],'supported',name)
        for name in ['Font','Image.from_image','Image.load','Image.save','Sound.save','Music.save','sounds','images']:
            self.assertEqual(self.find(name,'wasm')['status'],'partial',name)
        for name in ['Tilemap.from_tmx','Tilemap.load','load','save','gen_bgm','Node.update']:
            self.assertEqual(self.find(name,'wasm')['status'],'missing',name)
        self.assertEqual(self.find('Sound','wgpu')['status'],'missing')
        self.assertIn('旧JSフック',self.find('play_pos','wasm')['note'])

    def test_published_report_matches_reviewed_source(self):
        self.assertEqual(json.loads(inventory.OUTPUT.read_text()), self.data)


if __name__ == '__main__':
    unittest.main()
