"""
PyXel-Rust CLI
Extends Pyxel CLI with all original pyxel commands plus Rust game support.

Usage is a superset of the pyxel command:
  All original pyxel commands work identically.
  Additional rust_* commands support building and deploying Rust games.
"""

import os
import shutil
import subprocess
import sys
from pathlib import Path

import pyxel
import pyxel.utils

# Import all original pyxel CLI functions directly
from pyxel.cli import (
    run_python_script,
    watch_and_run_python_script,
    play_pyxel_app,
    edit_pyxel_resource,
    package_pyxel_app,
    create_executable_from_pyxel_app,
    create_html_from_pyxel_app,
    copy_pyxel_examples,
    _complete_extension,
    _check_file_exists,
    _check_dir_exists,
    _files_in_dir,
)


def cli():
    """Main CLI entry point for pyxel-rust"""
    commands = [
        # ---- Original Pyxel commands (100% compatible) ----
        (["run", "PYTHON_SCRIPT_FILE(.py)"], run_python_script),
        (["watch", "WATCH_DIR", "PYTHON_SCRIPT_FILE(.py)"], watch_and_run_python_script),
        (["play", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], play_pyxel_app),
        (["edit", f"[PYXEL_RESOURCE_FILE({pyxel.RESOURCE_FILE_EXTENSION})]"], edit_pyxel_resource),
        (["package", "APP_DIR", "STARTUP_SCRIPT_FILE(.py)"], package_pyxel_app),
        (["app2exe", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], create_executable_from_pyxel_app),
        (["app2html", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], create_html_from_pyxel_app),
        (["copy_examples"], copy_pyxel_examples),
        # ---- Rust-specific commands ----
        (["rust_run", "PROJECT_NAME"], rust_run),
        (["rust_package", "PROJECT_NAME"], rust_package),
        (["rust_play", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], rust_play),
        (["rust_app2html", "PROJECT_NAME", "[PORT]"], rust_app2html),
        (["rust_app2wasm", "PROJECT_NAME", "[PORT]"], rust_app2wasm),
        # ---- Sprite tools ----
        (["sprite_prompt", "DESCRIPTION", "[--name NAME]", "[--frame-size N]", "[--cols N]", "[--bg COLOR]", "[--facing DIR]"], _sprite_prompt_stub),
    ]

    # sprite_prompt uses its own argparse — intercept before the standard dispatcher
    if len(sys.argv) >= 2 and sys.argv[1] == 'sprite_prompt':
        sprite_prompt()
        return

    def print_usage(command_name=None):
        print("usage:")
        for command in commands:
            if command_name is None or command[0][0] == command_name:
                print(f"    pyxel-rust {' '.join(command[0])}")

    num_args = len(sys.argv)
    if num_args <= 1:
        print(f"PyXel-Rust {pyxel.VERSION}, Pyxel CLI with Rust game support")
        print_usage()
        return

    for command in commands:
        if sys.argv[1] != command[0][0]:
            continue
        max_args = len(command[0]) + 1
        min_args = max_args - sum(1 for s in command[0] if s.startswith("["))
        if not (min_args <= num_args <= max_args):
            print("invalid number of parameters")
            print_usage(command[0][0])
            sys.exit(1)
        command[1](*sys.argv[2:])
        return

    print(f"invalid command: '{sys.argv[1]}'")
    print_usage()
    sys.exit(1)


# ============================================================================
# Rust-specific commands
# ============================================================================

def _find_rust_project(project_name):
    """Return absolute path to PROJECT_NAME_rust directory, or exit with error."""
    project_path = os.path.abspath(os.path.join(
        os.path.dirname(__file__), '..', f'{project_name}_rust'
    ))
    if not os.path.isdir(project_path):
        print(f"project not found: {project_path}")
        sys.exit(1)
    return project_path


def _read_cargo_metadata(project_path):
    """Read title/author/version from Cargo.toml if present."""
    cargo_toml = Path(project_path) / 'Cargo.toml'
    meta = {}
    if not cargo_toml.exists():
        return meta
    import re
    text = cargo_toml.read_text(encoding='utf-8')
    for key in ('name', 'version', 'authors', 'description'):
        m = re.search(rf'^{key}\s*=\s*"(.+?)"', text, re.MULTILINE)
        if m:
            meta[key] = m.group(1)
    return meta


def rust_run(project_name):
    """Run a Rust project locally (cargo run)"""
    project_path = _find_rust_project(project_name)
    print(f"🎮 Running Rust project: {project_name}")
    result = subprocess.run(['cargo', 'run'], cwd=project_path)
    sys.exit(result.returncode)


def rust_play(pyxel_app_file):
    """Extract a Rust .pyxapp and run the bundled Rust binary directly.

    pyxel play cannot be used for Rust .pyxapp files because pyxel's Python
    runtime (Pyodide/Emscripten) does not support subprocess.  This command
    extracts the .pyxapp, finds the binary, and executes it natively.
    """
    import tempfile
    import zipfile
    import stat

    pyxel_app_file = _complete_extension(pyxel_app_file, "rust_play", pyxel.APP_FILE_EXTENSION)
    _check_file_exists(pyxel_app_file)

    with tempfile.TemporaryDirectory() as tmp:
        # Extract pyxapp (it's a zip file)
        with zipfile.ZipFile(pyxel_app_file) as zf:
            zf.extractall(tmp)

        # Find the binary: any executable file that is not a .py file
        tmp_path = Path(tmp)
        binary = None
        for f in tmp_path.rglob('*'):
            if f.is_file() and f.suffix not in ('.py', '.pyxres', '.pyxapp'):
                binary = f
                break

        if binary is None:
            print(f"❌ No binary found in {pyxel_app_file}")
            sys.exit(1)

        # Ensure executable permission
        binary.chmod(binary.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)

        print(f"🎮 Running: {binary.name}")
        result = subprocess.run([str(binary)])
        sys.exit(result.returncode)


def rust_package(project_name):
    """Build a Rust project and package it as a .pyxapp file.

    Workflow:
      1. cargo build --release
      2. Assemble app directory:
           startup.py        -- Python wrapper that launches the binary
           {binary}          -- the compiled Rust binary
           *.pyxres          -- any Pyxel resource files found in the project
      3. pyxel package -> PROJECT_NAME.pyxapp
    """
    import tempfile

    project_path = _find_rust_project(project_name)
    binary_name = project_name.replace('-', '_') + '_rust'

    # 1. Build
    print(f"🔨 Building {project_name}...")
    result = subprocess.run(['cargo', 'build', '--release'], cwd=project_path)
    if result.returncode != 0:
        print("❌ Build failed")
        sys.exit(1)
    print("✓ Build complete")

    binary_src = Path(project_path) / 'target' / 'release' / binary_name
    if not binary_src.exists():
        binary_src = Path(project_path) / 'target' / 'release' / project_name.replace('-', '_')
    if not binary_src.exists():
        print(f"❌ Binary not found: {binary_src}")
        sys.exit(1)

    # 2. Assemble app directory in a temp location
    # pyxel package writes {app_dir.name}.pyxapp to cwd, so we chdir into tmp
    dest = Path.cwd() / f"{project_name}{pyxel.APP_FILE_EXTENSION}"
    saved_cwd = Path.cwd()

    with tempfile.TemporaryDirectory() as tmp:
        app_dir = Path(tmp) / project_name
        app_dir.mkdir()

        # Copy binary
        binary_dst = app_dir / binary_src.name
        shutil.copy2(binary_src, binary_dst)
        binary_dst.chmod(0o755)
        print(f"✓ Copied binary: {binary_src.name}")

        # Copy *.pyxres resource files from the project root
        for res in Path(project_path).glob('*.pyxres'):
            shutil.copy2(res, app_dir / res.name)
            print(f"✓ Copied resource: {res.name}")

        # Read Cargo metadata for the startup script header
        meta = _read_cargo_metadata(project_path)
        title   = meta.get('name', project_name)
        author  = meta.get('authors', '')
        version = meta.get('version', '')
        desc    = meta.get('description', f'Rust game: {project_name}')

        # Write startup.py
        startup = app_dir / 'startup.py'
        startup.write_text(
            f"# title: {title}\n"
            f"# author: {author}\n"
            f"# desc: {desc}\n"
            f"# version: {version}\n"
            "import subprocess, sys\n"
            "from pathlib import Path\n"
            "\n"
            f"binary = Path(__file__).parent / {repr(binary_src.name)}\n"
            "binary.chmod(0o755)\n"
            "result = subprocess.run([str(binary)])\n"
            "sys.exit(result.returncode)\n",
            encoding='utf-8',
        )
        print("✓ Created startup.py")

        # 3. Package with pyxel
        # pyxel package writes {project_name}.pyxapp into cwd, so chdir to tmp first
        print(f"📦 Packaging as {project_name}.pyxapp ...")
        os.chdir(tmp)
        try:
            package_pyxel_app(str(app_dir), str(startup))
        finally:
            os.chdir(saved_cwd)

        generated = Path(tmp) / f"{project_name}{pyxel.APP_FILE_EXTENSION}"
        if not generated.exists():
            print(f"❌ Could not find generated {project_name}.pyxapp in {tmp}")
            sys.exit(1)

        shutil.move(str(generated), str(dest))

    print(f"✅ Created: {dest}")
    print(f"   Play with:       pyxel-rust play {dest.name}")
    print(f"   Make HTML with:  pyxel-rust app2html {dest.name}")


def rust_app2html(project_name, port_str='8000'):
    """Package a Rust project and convert it to a standalone HTML file.

    Runs rust_package first, then pyxel app2html, then serves docs/ on localhost.
    Output is placed in docs/examples/{project_name}/index.html.
    """
    try:
        port = int(port_str)
    except ValueError:
        print(f"invalid port: {port_str}")
        sys.exit(1)

    base_dir = Path(__file__).parent
    pyxapp = Path.cwd() / f"{project_name}{pyxel.APP_FILE_EXTENSION}"

    # 1. Package the Rust project
    rust_package(project_name)

    if not pyxapp.exists():
        print(f"❌ Expected {pyxapp} but it was not created")
        sys.exit(1)

    # 2. Convert to HTML (creates {project_name}.html in cwd)
    print(f"🌐 Converting to HTML...")
    create_html_from_pyxel_app(str(pyxapp))

    html_src = Path.cwd() / f"{project_name}.html"
    if not html_src.exists():
        print(f"❌ HTML generation failed: {html_src} not found")
        sys.exit(1)

    # 3. Deploy to docs/examples/{project_name}/index.html
    output_dir = base_dir / 'docs' / 'examples' / project_name
    output_dir.mkdir(parents=True, exist_ok=True)
    html_dst = output_dir / 'index.html'
    shutil.move(str(html_src), str(html_dst))
    print(f"✅ Deployed: {html_dst}")

    # 4. Serve docs/ on localhost
    print(f"\n🌐 Starting web server on http://localhost:{port}")
    print(f"   Open: http://localhost:{port}/examples/{project_name}/")
    print("   Press Ctrl+C to stop")
    try:
        subprocess.run(
            [sys.executable, '-m', 'http.server', str(port)],
            cwd=str(base_dir / 'docs'),
        )
    except KeyboardInterrupt:
        print("\r✓ Server stopped")
        sys.exit(0)


def rust_app2wasm(project_name, port_str='8000'):
    """Build Rust project to WASM target and serve as HTML

    Compiles to wasm32-unknown-unknown target, generates WebAssembly
    binary, and serves with HTML wrapper via HTTP server.
    """
    try:
        port = int(port_str)
    except ValueError:
        print(f"invalid port: {port_str}")
        sys.exit(1)

    project_path = _find_rust_project(project_name)
    base_dir = Path(__file__).parent

    print(f"🔨 Building {project_name} to WASM (wasm32-unknown-unknown)...")

    # Build to WASM target
    env = os.environ.copy()
    env['RUSTFLAGS'] = '-C target-feature=+bulk-memory'
    
    result = subprocess.run(
        ['cargo', 'build', '--target', 'wasm32-unknown-unknown', '--release'],
        cwd=project_path,
        env=env
    )
    
    if result.returncode != 0:
        print("❌ WASM build failed")
        print("Note: Ensure dependencies support wasm32 target")
        sys.exit(1)

    print("✓ WASM build complete")

    binary_name = project_name.replace('-', '_')
    output_dir = base_dir / 'docs' / 'examples' / f'{project_name}_wasm'
    output_dir.mkdir(parents=True, exist_ok=True)

    # Copy WASM binary
    wasm_src = (
        Path(project_path) / 'target' / 'wasm32-unknown-unknown' / 'release'
        / f'{binary_name}.wasm'
    )
    if wasm_src.is_file():
        wasm_dst = output_dir / wasm_src.name
        shutil.copy(wasm_src, wasm_dst)
        print(f"✓ Deployed WASM: {wasm_dst}")
    else:
        print(f"⚠️  WASM file not found: {wasm_src}")

    # Generate HTML with WASM loader
    html_file = output_dir / 'index.html'
    html_file.write_text(
        "<!DOCTYPE html>\n"
        "<html lang=\"en\">\n"
        "<head>\n"
        "  <meta charset=\"UTF-8\">\n"
        "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n"
        f"  <title>{project_name} WASM</title>\n"
        "  <style>\n"
        "    body { margin:0; background:#1a1a1a; color:#fff; display:flex;\n"
        "           justify-content:center; align-items:center; min-height:100vh; }\n"
        "    #container { text-align:center; }\n"
        "    canvas { border:3px solid #fff; image-rendering:pixelated;\n"
        "             display:block; margin:20px auto; }\n"
        "  </style>\n"
        "</head>\n"
        "<body>\n"
        "  <div id=\"container\">\n"
        f"    <h1>{project_name}</h1>\n"
        "    <canvas id=\"game\" width=\"800\" height=\"600\"></canvas>\n"
        "    <p id=\"loading\">Loading WASM...</p>\n"
        "  </div>\n"
        "  <script>\n"
        "    async function loadWasm() {\n"
        "      try {\n"
        f"        const r = await fetch('{binary_name}.wasm');\n"
        "        const buf = await r.arrayBuffer();\n"
        "        const wasm = await WebAssembly.instantiate(buf, {\n"
        "          env: { memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }) }\n"
        "        });\n"
        "        document.getElementById('loading').textContent = '✓ WASM loaded';\n"
        "        if (wasm.instance.exports.init) wasm.instance.exports.init();\n"
        "      } catch (e) {\n"
        "        document.getElementById('loading').textContent = '❌ ' + e.message;\n"
        "      }\n"
        "    }\n"
        "    window.addEventListener('load', loadWasm);\n"
        "  </script>\n"
        "</body>\n"
        "</html>\n",
        encoding='utf-8',
    )
    print(f"✓ Generated HTML: {html_file}")

    # Start HTTP server
    print(f"\n🌐 Starting web server on http://localhost:{port}")
    print(f"   Open: http://localhost:{port}/examples/{project_name}_wasm/")
    print("   Press Ctrl+C to stop")
    try:
        subprocess.run(
            [sys.executable, '-m', 'http.server', str(port)],
            cwd=str(base_dir / 'docs'),
        )
    except KeyboardInterrupt:
        print("\r✓ Server stopped")
        sys.exit(0)


if __name__ == '__main__':
    cli()


# ============================================================================
# Sprite tools
# ============================================================================

# Stub used only for usage-line display in the command table
def _sprite_prompt_stub(*_):
    sprite_prompt()


def sprite_prompt():
    """Generate an AI image prompt + spec JSON for a pyxel-rust sprite sheet.

    Usage:
        pyxel-rust sprite_prompt "cute cat warrior" --name mychar
        pyxel-rust sprite_prompt "red dragon boss" --frame-size 256 --cols 6

    Writes {name}.sprite-spec.json and prints the full AI prompt to stdout.
    The spec JSON is shared with the game engine so both sides use the same rules.
    """
    import argparse
    import json

    BG_COLORS = {
        'magenta': {'hex': '#FF00FF', 'name': 'solid pure magenta (#FF00FF)'},
        'green':   {'hex': '#00FF00', 'name': 'solid pure green (#00FF00)'},
        'blue':    {'hex': '#0000FF', 'name': 'solid pure blue (#0000FF)'},
    }

    DEFAULT_ANIMS = [
        'Idle', 'Walk', 'Run', 'Jump',
        'MeleeAttack', 'RangedAttack', 'Damage', 'TurnInPlace',
        'SpecialAttack', 'Singing', 'Resting', 'Victory',
    ]

    parser = argparse.ArgumentParser(
        prog='pyxel-rust sprite_prompt',
        description='Generate an AI image prompt for a pyxel-rust compatible sprite sheet',
    )
    parser.add_argument('description',
                        help='Character description  e.g. "cute chibi cat warrior"')
    parser.add_argument('--name', default=None,
                        help='Base name for output files (default: derived from description)')
    parser.add_argument('--frame-size', type=int, default=128, metavar='N',
                        help='Frame size in pixels (default: 128)')
    parser.add_argument('--cols', type=int, default=8, metavar='N',
                        help='Frames per animation row (default: 8)')
    parser.add_argument('--bg', default='magenta', choices=list(BG_COLORS),
                        help='Background colour for chromakey (default: magenta)')
    parser.add_argument('--facing', default='right', choices=['right', 'left'],
                        help='Character facing direction (default: right)')
    parser.add_argument('--anims', default=None, metavar='ANIM;...',
                        help='Semicolon-separated animation list (default: 12 standard rows)')

    opts = parser.parse_args(sys.argv[2:])

    anims = opts.anims.split(';') if opts.anims else DEFAULT_ANIMS
    rows = len(anims)
    bg = BG_COLORS[opts.bg]
    frame_size = opts.frame_size
    cols = opts.cols
    total_w = frame_size * cols
    total_h = frame_size * rows

    name = opts.name or opts.description.lower().replace(' ', '-')[:32]
    name = ''.join(c if c.isalnum() or c in '-_' else '-' for c in name).strip('-')

    # Write spec JSON (shared with the game engine)
    spec = {
        'frame_w': frame_size,
        'frame_h': frame_size,
        'cols': cols,
        'bg': bg['hex'],
        'facing': opts.facing,
        'anims': anims,
    }
    spec_file = Path(f'{name}.sprite-spec.json')
    spec_file.write_text(json.dumps(spec, indent=2, ensure_ascii=False), encoding='utf-8')
    print(f'✓ Spec written: {spec_file}', file=sys.stderr)

    # Build animation list for the prompt
    anim_lines = '\n'.join(
        f'{i + 1}. {anim} (exactly {cols} frames, duplicate to fill if needed)'
        for i, anim in enumerate(anims)
    )

    prompt = f"""\
A production-ready 2D sprite sheet of {opts.description}.

STRICT TECHNICAL REQUIREMENTS:
- Each frame must be exactly {frame_size}x{frame_size} pixels
- The entire sheet must be a perfect grid: {cols} columns x {rows} rows
- Total image size: {total_w}x{total_h} pixels
- Frames must be tightly packed with NO spacing or padding
- Each animation row must start from the LEFTMOST column
- Character must be centered horizontally in every frame
- Character feet must be consistently positioned 2 pixels above the bottom edge
- Character scale must remain identical across all frames

BACKGROUND RULE:
- Background must be {bg['name']}
- No gradients, no patterns, no transparency
- No checkerboard transparency

VISUAL CLEANLINESS:
- NO grid lines, NO guides, NO borders
- NO text, NO labels, NO UI elements, NO annotations

ORIENTATION RULE (CRITICAL):
- Character must ALWAYS face {opts.facing.upper()}
- Use a 3/4 side view (face must always be visible)
- NO front-facing views, NO back-facing views
- NO camera rotation between frames

ANIMATION STRUCTURE (ALL MUST BE EXACTLY {cols} FRAMES PER ROW):
{anim_lines}

EFFECT RULE:
- NO visual effects at all
- No glow, no particles, no beams, no magic, no dust

STYLE:
- Clean, game-ready sprite style
- Slight chibi proportions, stable silhouette
- Consistent lighting and shading
- Designed for real-time game use

FINAL OUTPUT REQUIREMENT:
- Image size must be exactly {total_w}x{total_h} pixels
- Clean sprite atlas ready for direct use in a game engine\
"""

    print(prompt)
    print(f'\n✓ Prompt printed above. Spec: {spec_file}', file=sys.stderr)
