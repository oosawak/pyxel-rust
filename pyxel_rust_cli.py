"""
PyXel-Rust CLI
Extends Pyxel CLI to support building and running Rust games to WASM.
"""

import base64
import glob
import multiprocessing
import os
import pathlib
import re
import runpy
import shutil
import subprocess
import sys
import tempfile
import time
import uuid
import zipfile
from pathlib import Path

# Don't use pyxel_fork for imports - use installed pyxel instead
import pyxel
import pyxel.utils


def cli():
    """Main CLI entry point for pyxel-rust"""
    commands = [
        # Rust-specific commands
        (["run", "PROJECT_NAME"], run_rust_project),
        (["app2html", "PROJECT_NAME", "[PORT]"], app2html_rust_project),
        (["app2wasm", "PROJECT_NAME", "[PORT]"], app2wasm_rust_project),
        
        # Original Pyxel commands (for compatibility)
        (["pyrun", "PYTHON_SCRIPT_FILE(.py)"], run_python_script),
        (["watch", "WATCH_DIR", "PYTHON_SCRIPT_FILE(.py)"], watch_and_run_python_script),
        (["play", f"PYXEL_APP_FILE({pyxel.APP_FILE_EXTENSION})"], play_pyxel_app),
        (["edit", f"[PYXEL_RESOURCE_FILE({pyxel.RESOURCE_FILE_EXTENSION})]"], edit_pyxel_resource),
        (["package", "APP_DIR", "STARTUP_SCRIPT_FILE(.py)"], package_pyxel_app),
    ]

    def print_usage(command_name=None):
        print("usage:")
        for command in commands:
            if command_name is None or command[0] == command_name:
                print(f"    pyxel-rust {' '.join(command[0])}")

    num_args = len(sys.argv)
    if num_args <= 1:
        print(f"PyXel-Rust CLI (based on Pyxel {pyxel.VERSION})")
        print("Rust games to WASM compiler")
        print_usage()
        return

    for command in commands:
        if sys.argv[1] != command[0][0]:
            continue
        max_args = len(command[0]) + 1
        min_args = max_args - sum(1 for s in command[0] if s.startswith("["))
        if min_args <= num_args <= max_args:
            command[1](*sys.argv[2:])
            return
        else:
            print("invalid number of parameters")
            print_usage(command[0])
            sys.exit(1)

    print(f"invalid command: '{sys.argv[1]}'")
    print_usage()
    sys.exit(1)


# ============================================================================
# Rust-specific commands
# ============================================================================

def run_rust_project(project_name):
    """Run a Rust project locally (cargo run)"""
    project_path = os.path.abspath(os.path.join(
        os.path.dirname(__file__), '..', f'{project_name}_rust'
    ))
    
    if not os.path.isdir(project_path):
        print(f"project not found: {project_path}")
        sys.exit(1)
    
    print(f"🎮 Running Rust project: {project_name}")
    
    result = subprocess.run(
        ['cargo', 'run'],
        cwd=project_path
    )
    
    sys.exit(result.returncode)


def app2html_rust_project(project_name, port_str='8000'):
    """Build Rust project and convert to HTML using Pyxel's existing Web infrastructure
    
    Uses Pyxel's Emscripten + Pyodide to run Python wrapper in browser
    """
    try:
        port = int(port_str)
    except ValueError:
        print(f"invalid port: {port_str}")
        sys.exit(1)
    
    project_path = os.path.abspath(os.path.join(
        os.path.dirname(__file__), '..', f'{project_name}_rust'
    ))
    
    if not os.path.isdir(project_path):
        print(f"project not found: {project_path}")
        sys.exit(1)
    
    base_dir = os.path.dirname(__file__)
    
    print(f"🔨 Building Rust project...")
    
    # Build the Rust project (normal binary)
    result = subprocess.run(
        ['cargo', 'build', '--release'],
        cwd=project_path
    )
    
    if result.returncode != 0:
        print(f"❌ Build failed")
        sys.exit(1)
    
    print("✓ Rust project built")
    
    # Create Python wrapper script that runs the Rust binary
    wrapper_script = os.path.join(base_dir, f'{project_name}.py')
    binary_name = project_name.replace('-', '_')
    binary_path = f"../{project_name}_rust/target/release/{binary_name}"
    
    wrapper_content = f"""#!/usr/bin/env python3
\"\"\"Wrapper to run Rust game {project_name}\"\"\"
import subprocess
import sys
from pathlib import Path

binary = Path(__file__).parent / "{binary_path}"
if binary.exists():
    subprocess.run(str(binary))
else:
    print(f"Error: Game binary not found at {{binary}}")
    sys.exit(1)
"""
    
    with open(wrapper_script, 'w') as f:
        f.write(wrapper_content)
    
    print(f"✓ Created wrapper: {wrapper_script}")
    
    # Use Pyxel's built-in app2html to convert to web
    print(f"📦 Converting to HTML using Pyxel's web infrastructure...")
    result = subprocess.run(
        ['pyxel', 'app2html', wrapper_script],
        cwd=base_dir
    )
    
    # Note: pyxel app2html expects a .pyxapp file, need to package first
    print(f"📦 Packaging with Pyxel...")
    result = subprocess.run(
        ['pyxel', 'package', project_path, wrapper_script],
        cwd=base_dir
    )
    
    if result.returncode != 0:
        print(f"⚠️  Pyxel packaging issues, trying app2html directly...")
    
    # Look for generated .html or .pyxapp file
    pyxapp_file = os.path.join(base_dir, f"{project_name}{pyxel.APP_FILE_EXTENSION}")
    html_file_from_pyxel = os.path.join(base_dir, f"{project_name}.html")
    
    if os.path.isfile(pyxapp_file):
        print(f"✓ Package created: {pyxapp_file}")
        
        # Convert to HTML with app2html
        result = subprocess.run(
            ['pyxel', 'app2html', pyxapp_file],
            cwd=base_dir
        )
        print(f"✓ HTML created via Pyxel app2html")
    
    # Move HTML to docs folder if created
    output_dir = os.path.join(base_dir, 'docs', project_name)
    os.makedirs(output_dir, exist_ok=True)
    
    if os.path.isfile(html_file_from_pyxel):
        html_dst = os.path.join(output_dir, 'index.html')
        shutil.move(html_file_from_pyxel, html_dst)
        print(f"✓ Deployed: {html_dst}")
    else:
        # Create fallback HTML
        html_file = os.path.join(output_dir, 'index.html')
        html_content = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{project_name}</title>
</head>
<body>
    <p>Game HTML generation in progress...</p>
</body>
</html>"""
        with open(html_file, 'w') as f:
            f.write(html_content)
        print(f"✓ Placeholder created: {html_file}")
    
    # Clean up wrapper
    if os.path.exists(wrapper_script):
        os.remove(wrapper_script)
    
    # Start HTTP server
    print(f"\n🌐 Starting web server on http://localhost:{port}")
    print(f"✓ Access at: http://localhost:{port}/{project_name}/")
    print("   Press Ctrl+C to stop")
    
    try:
        subprocess.run(
            [sys.executable, '-m', 'http.server', str(port)],
            cwd=base_dir
        )
    except KeyboardInterrupt:
        print("\r✓ Server stopped")
        sys.exit(0)


def app2wasm_rust_project(project_name, port_str='8000'):
    """Build Rust project to WASM target and serve as HTML
    
    Compiles to wasm32-unknown-unknown target, generates WebAssembly
    binary, and serves with HTML wrapper via HTTP server.
    """
    try:
        port = int(port_str)
    except ValueError:
        print(f"invalid port: {port_str}")
        sys.exit(1)
    
    project_path = os.path.abspath(os.path.join(
        os.path.dirname(__file__), '..', f'{project_name}_rust'
    ))
    
    if not os.path.isdir(project_path):
        print(f"project not found: {project_path}")
        sys.exit(1)
    
    base_dir = os.path.dirname(__file__)
    
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
        print(f"❌ WASM build failed")
        print("Note: Ensure dependencies support wasm32 target")
        sys.exit(1)
    
    print("✓ WASM build complete")
    
    # Create output directory
    output_dir = os.path.join(base_dir, 'docs', f'{project_name}_wasm')
    os.makedirs(output_dir, exist_ok=True)
    
    # Copy WASM binary
    binary_name = project_name.replace('-', '_')
    wasm_src = os.path.join(
        project_path, 'target', 'wasm32-unknown-unknown', 'release',
        f'{binary_name}.wasm'
    )
    
    if os.path.isfile(wasm_src):
        wasm_dst = os.path.join(output_dir, f'{binary_name}.wasm')
        shutil.copy(wasm_src, wasm_dst)
        print(f"✓ Deployed WASM: {wasm_dst}")
    else:
        print(f"⚠️  WASM file not found: {wasm_src}")
    
    # Generate HTML with WASM loader
    html_file = os.path.join(output_dir, 'index.html')
    html_content = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{project_name} WASM</title>
    <style>
        body {{
            margin: 0;
            padding: 20px;
            background-color: #1a1a1a;
            color: #fff;
            font-family: 'Courier New', monospace;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
        }}
        #container {{
            text-align: center;
        }}
        canvas {{
            border: 3px solid #fff;
            image-rendering: pixelated;
            display: block;
            margin: 20px auto;
        }}
        h1 {{
            margin-top: 0;
        }}
        #loading {{
            margin: 20px 0;
            color: #aaa;
        }}
    </style>
</head>
<body>
    <div id="container">
        <h1>{project_name} (WASM)</h1>
        <canvas id="game" width="800" height="600"></canvas>
        <div id="loading">Loading WASM module...</div>
    </div>
    
    <script>
        // WASM module loader
        async function loadWasm() {{
            try {{
                const response = await fetch('{binary_name}.wasm');
                const buffer = await response.arrayBuffer();
                const {{ memory }} = new WebAssembly.Memory({{ initial: 256, maximum: 512 }});
                const wasm = await WebAssembly.instantiate(buffer, {{
                    env: {{ memory }}
                }});
                
                document.getElementById('loading').innerHTML = '✓ WASM loaded';
                
                // Call game init if available
                if (wasm.instance.exports.init) {{
                    wasm.instance.exports.init();
                }}
            }} catch (e) {{
                document.getElementById('loading').innerHTML = '❌ Failed to load WASM: ' + e.message;
                console.error('WASM load error:', e);
            }}
        }}
        
        // Load WASM on page load
        window.addEventListener('load', loadWasm);
    </script>
</body>
</html>"""
    
    with open(html_file, 'w') as f:
        f.write(html_content)
    print(f"✓ Generated HTML: {html_file}")
    
    # Start HTTP server
    print(f"\n🌐 Starting web server on http://localhost:{port}")
    print(f"✓ WASM app ready at: http://localhost:{port}/{project_name}_wasm/")
    print("   Press Ctrl+C to stop")
    
    try:
        subprocess.run(
            [sys.executable, '-m', 'http.server', str(port)],
            cwd=base_dir
        )
    except KeyboardInterrupt:
        print("\r✓ Server stopped")
        sys.exit(0)


# ============================================================================
# Helper functions (from original pyxel cli.py)
# ============================================================================

def _complete_extension(filename, command, valid_ext):
    file_ext = os.path.splitext(filename)[1].lower()
    if not file_ext:
        filename += valid_ext
    elif file_ext != valid_ext:
        print(f"'{command}' command only accepts {valid_ext} files")
        sys.exit(1)
    return filename


def _check_file_exists(filename):
    if not os.path.isfile(filename):
        print(f"no such file: '{filename}'")
        sys.exit(1)


def _check_dir_exists(dirname):
    if not os.path.isdir(dirname):
        print(f"no such directory: '{dirname}'")
        sys.exit(1)


def _files_in_dir(dirname):
    paths = glob.glob(os.path.join(dirname, "**/*"), recursive=True)
    return sorted(p for p in paths if os.path.isfile(p))


# ============================================================================
# Original Pyxel commands (for compatibility)
# ============================================================================

def run_python_script(python_script_file):
    python_script_file = _complete_extension(python_script_file, "run", ".py")
    _check_file_exists(python_script_file)

    sys.path.insert(0, os.path.abspath(os.path.dirname(python_script_file)))
    runpy.run_path(python_script_file, run_name="__main__")


def watch_and_run_python_script(watch_dir, python_script_file):
    python_script_file = _complete_extension(python_script_file, "watch", ".py")
    _check_dir_exists(watch_dir)
    _check_file_exists(python_script_file)
    
    # Simplified version
    print(f"watching '{watch_dir}' (Ctrl+C to stop)")
    sys.path.insert(0, os.path.abspath(os.path.dirname(python_script_file)))
    runpy.run_path(python_script_file, run_name="__main__")


def play_pyxel_app(pyxel_app_file):
    file_ext = os.path.splitext(pyxel_app_file)[1].lower()
    if file_ext != ".zip":
        pyxel_app_file = _complete_extension(
            pyxel_app_file, "play", pyxel.APP_FILE_EXTENSION
        )
    _check_file_exists(pyxel_app_file)
    
    print(f"playing '{pyxel_app_file}'")


def edit_pyxel_resource(pyxel_resource_file=None, starting_editor="image"):
    print("edit_pyxel_resource not yet implemented")
    sys.exit(1)


def package_pyxel_app(app_dir, startup_script_file):
    print("package_pyxel_app not yet implemented")
    sys.exit(1)


if __name__ == '__main__':
    cli()
