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
        self.assertEqual(self.find('rect', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('btnp', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('mouse_wheel', 'wasm')['status'], 'supported')
        self.assertEqual(self.find('Node.update', 'wasm')['status'], 'supported')
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
        for name in ['Sound.save','Music.save','dropped_files']:
            self.assertEqual(self.find(name,'wasm')['status'],'partial',name)
        for name in ['screencast','screen_mode','show','flip','reset','user_data_dir']:
            self.assertEqual(self.find(name,'wasm')['status'],'partial',name)
        for name in ['Node.update','Node.from_mesh','Node.apply_motion','Node.play_motion','Node.stop_motion','Mat4.pos','Mat4.rot','Mat4.scale','Node.draw','Node.on_update','Node.on_draw','Node.on_collide','Node.on_destroy','screen','cursor','font','blt3d','bltm3d','reset_screencast']:
            self.assertEqual(self.find(name,'wasm')['status'],'supported',name)
        self.assertEqual(self.find('Node.update','wgpu')['status'],'missing')
        for name in ['gen_bgm','colors','images','sounds','title','resize','icon','fullscreen','integer_scale','perf_monitor','input_text','set_input_text','set_dropped_files']:
            self.assertEqual(self.find(name,'wasm')['status'],'supported',name)
        for name in ['load','save','screenshot','load_pal','save_pal','gen_bgm']:
            self.assertEqual(self.find(name,'sdl2')['status'],'supported',name)
        self.assertEqual(self.find('input_text','wgpu')['status'],'supported')
        self.assertEqual(self.find('Sound','wgpu')['status'],'missing')
        self.assertIn('旧JSフック',self.find('play_pos','wasm')['note'])

    def test_initialization_and_filled_shapes_are_implemented(self):
        for backend in ['wasm','sdl2']:
            for name in ['init','rect','circ']:
                self.assertEqual(self.find(name,backend)['status'],'supported')
                self.assertEqual(self.find(name,backend)['verification'],'compat-options')
        self.assertEqual(self.find('init','wgpu')['status'],'partial')
        self.assertEqual(self.find('rect','wgpu')['status'],'supported')
        image = self.find('Image.save','wasm')
        self.assertEqual(image['status'],'supported')
        self.assertIn('仮想ファイル名',image['note'])

    def test_named_browser_file_operations_are_backed_by_real_decoders(self):
        for name in ['load','save','load_pal','save_pal','screenshot','Font','Image.from_image','Image.load','Image.save','Tilemap.from_tmx','Tilemap.load','Sound.pcm']:
            assessment=self.find(name,'wasm')
            self.assertEqual(assessment['status'],'supported',name)
            self.assertEqual(assessment['verification'],'named-files',name)
        for name in ['Sound.save','Music.save','Mesh.from_glb','user_data_dir','dropped_files']:
            self.assertEqual(self.find(name,'wasm')['status'],'partial',name)

    def test_node_painter_support_has_runtime_evidence(self):
        for backend in ['wasm','sdl2']:
            for member in ['dither','depth_test','depth_write','depth_offset','decal','shaded','pset','line','tri','trib','rect','rectb','circ','circb','elli','ellib','box','boxb','sphere','sphereb','plane','sprite','prim','text']:
                assessment = self.find('Node.'+member,backend)
                self.assertEqual(assessment['status'],'supported',member)
                self.assertEqual(assessment['verification'],'node-painter',member)
                self.assertIn('/src/api/scene3d/painter.rs#L',assessment['source'])
                self.assertNotEqual(assessment['source'].split('#L')[-1],'1')
                self.assertEqual(self.find('Node.'+member,'wgpu')['status'],'missing')

    def test_published_report_matches_reviewed_source(self):
        self.assertEqual(json.loads(inventory.OUTPUT.read_text()), self.data)


if __name__ == '__main__':
    unittest.main()
