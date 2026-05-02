"""
Cubeboy Rust game wrapper for Pyxel app2html
This Python script loads and runs the Rust Cubeboy binary
"""

import os
import sys
import subprocess
from pathlib import Path

# Get the Rust binary path
project_dir = Path(__file__).parent.parent / "cubeboy_rust"
binary_path = project_dir / "target" / "debug" / "cubeboy_rust"

if binary_path.exists():
    # Run the compiled Rust binary
    subprocess.run(str(binary_path))
else:
    print(f"Error: Cubeboy binary not found at {binary_path}")
    print("Run 'cargo build' in cubeboy_rust directory first")
    sys.exit(1)
