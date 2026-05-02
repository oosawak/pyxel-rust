// New project command

use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub fn new_project(name: &str, template: &str) -> Result<()> {
    println!("📦 Creating new Pyxel-rust project: {}", name);

    let project_dir = Path::new(name);
    if project_dir.exists() {
        return Err(anyhow!("Project directory '{}' already exists", name));
    }

    // Create directory structure
    fs::create_dir_all(project_dir)?;
    fs::create_dir_all(project_dir.join("src"))?;

    // Create Cargo.toml
    let cargo_toml = match template {
        "game" | "basic" => create_game_cargo_toml(name),
        "minimal" => create_minimal_cargo_toml(name),
        _ => create_game_cargo_toml(name),
    };

    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    // Create game script
    let game_rs = match template {
        "game" => create_game_template(),
        "minimal" => create_minimal_template(),
        _ => create_basic_template(),
    };

    fs::write(project_dir.join("src").join("main.rs"), game_rs)?;

    // Create README
    fs::write(
        project_dir.join("README.md"),
        format!("# {}\n\nA Pyxel-rust game.\n\nRun with: `pyxel-rust run src/main.rs`\n", name),
    )?;

    println!("✅ Project created successfully!");
    println!("\n📂 Project structure:");
    println!("  {}/", name);
    println!("  ├── Cargo.toml");
    println!("  ├── README.md");
    println!("  └── src/");
    println!("      └── main.rs");
    println!("\n🚀 Next steps:");
    println!("  cd {}", name);
    println!("  pyxel-rust run src/main.rs");

    Ok(())
}

fn create_game_cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Pyxel-rust wrapper library with complete Pyxel API
pyxel-rust = {{ path = "../pyxel-rust" }}

[[bin]]
name = "{}"
path = "src/main.rs"
"#,
        name, name
    )
}

fn create_minimal_cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
# Pyxel-rust wrapper library with complete Pyxel API
pyxel-rust = {{ path = "../pyxel-rust" }}

[[bin]]
name = "{}"
path = "src/main.rs"
"#,
        name, name
    )
}

fn create_game_template() -> String {
    r#"use pyxel_rust::prelude::*;

fn main() {
    init(128, 128, "My Game", 60);
    run(update, draw);
}

fn update() {
    // Update game logic here
    if btn(KEY_Q) {
        quit();
    }
}

fn draw() {
    // Draw game graphics here
    cls(COLOR_BLACK);
    
    // Draw a circle in the center
    let x = width() / 2;
    let y = height() / 2;
    circfill(x as f32, y as f32, 20.0, COLOR_ORANGE);
    
    // Draw text
    text(10.0, 10.0, "Press Q to quit", COLOR_WHITE);
}
"#
    .to_string()
}

fn create_basic_template() -> String {
    r#"use pyxel_rust::prelude::*;

fn main() {
    init(160, 120, "Game", 60);
    run(update, draw);
}

fn update() {
    if btn(KEY_Q) {
        quit();
    }
}

fn draw() {
    cls(COLOR_BLACK);
    text(70.0, 55.0, "Hello, Pyxel!", COLOR_WHITE);
}
"#
    .to_string()
}

fn create_minimal_template() -> String {
    r#"use pyxel_rust::prelude::*;

fn main() {
    init(128, 128, "Game", 60);
    run(update, draw);
}

fn update() {
}

fn draw() {
    cls(COLOR_BLACK);
}
"#
    .to_string()
}

