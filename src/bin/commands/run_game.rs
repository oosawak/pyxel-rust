// Run game command

use anyhow::{anyhow, Result};
use std::process::{Command, Stdio};
use std::path::Path;
use std::thread;
use std::time::Duration;

pub fn run_example(name: &str) -> Result<()> {
    println!("🎮 Running example: {}", name);
    
    let output = Command::new("cargo")
        .args(&["run", "--example", name])
        .current_dir(".")
        .output()?;
    
    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        return Err(anyhow!("Failed to run example: {}", name));
    }
    
    Ok(())
}

pub fn app2html(name: &str, port: u16) -> Result<()> {
    println!("📦 Converting {} to HTML/WASM...", name);
    
    // Check if wasm-pack is available
    let check_wasm = Command::new("wasm-pack")
        .arg("--version")
        .output();
    
    if check_wasm.is_err() {
        return Err(anyhow!(
            "wasm-pack not found. Install with: cargo install wasm-pack"
        ));
    }
    
    println!("🔨 Building WASM library...");
    
    let build_status = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-dir", "web/pkg", "--release"])
        .current_dir(".")
        .status()?;
    
    if !build_status.success() {
        return Err(anyhow!("WASM build failed"));
    }
    
    println!("✓ WASM build complete");
    
    // Check if web directory exists
    if !Path::new("web").exists() {
        return Err(anyhow!("web/ directory not found"));
    }
    
    println!("🌐 Starting web server on http://localhost:{}", port);
    
    // Start HTTP server
    let mut server = Command::new("python3")
        .args(&["-m", "http.server", &port.to_string()])
        .current_dir("web")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    
    // Wait a moment for server to start, then open browser
    thread::sleep(Duration::from_millis(500));
    
    let url = format!("http://localhost:{}", port);
    let _ = open_browser(&url);
    
    println!("✓ Server running. Press Ctrl+C to stop.");
    
    // Wait for server to finish (until Ctrl+C)
    server.wait()?;
    
    println!("✓ Server stopped");
    Ok(())
}

fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(url).spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open").arg(url).spawn()?;
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("start").arg(url).spawn()?;
    }
    
    Ok(())
}

pub fn run_game(file: &Path, _debug: bool) -> Result<()> {
    if !file.exists() {
        return Err(anyhow!("Game file not found: {:?}", file));
    }

    println!("🎮 Running Pyxel-rust game: {:?}", file);
    println!("⚠️  Game execution not yet implemented");

    Ok(())
}
