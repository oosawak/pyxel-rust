// pyxel-rust CLI
// Main entry point for pyxel-rust command

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(name = "pyxel-rust")]
#[command(about = "Rust implementation of Pyxel game engine", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to game file or resource
    file: Option<PathBuf>,

    /// Open resource editor
    #[arg(long)]
    editor: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Pyxel project
    #[command(about = "Create a new Pyxel-rust project")]
    New {
        /// Project name
        name: String,

        /// Template type (basic, game, minimal)
        #[arg(short, long, default_value = "basic")]
        template: String,
    },

    /// Run a Pyxel game script
    #[command(about = "Run a Pyxel-rust game")]
    Run {
        /// Game file path
        file: PathBuf,

        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },

    /// Open resource editor
    #[command(about = "Open the Pyxel resource editor")]
    Editor {
        /// Resource file path
        file: Option<PathBuf>,
    },

    /// Version information
    #[command(about = "Show version information")]
    Version,

    /// Build project
    #[command(about = "Build Pyxel-rust project")]
    Build {
        /// Build release version
        #[arg(long)]
        release: bool,

        /// Build for WASM
        #[arg(long)]
        wasm: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup logging
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Handle commands
    match cli.command {
        Some(Commands::New { name, template }) => {
            commands::new_project(&name, &template)?;
        }

        Some(Commands::Run { file, debug }) => {
            commands::run_game(&file, debug)?;
        }

        Some(Commands::Editor { file }) => {
            commands::open_editor(file.as_deref())?;
        }

        Some(Commands::Version) => {
            println!("pyxel-rust version 0.1.0");
            println!("Based on Pyxel (https://github.com/kitao/pyxel)");
        }

        Some(Commands::Build { release, wasm }) => {
            commands::build_project(release, wasm)?;
        }

        None => {
            // If file is provided but no subcommand, treat as run
            if let Some(file) = cli.file {
                commands::run_game(&file, false)?;
            } else if cli.editor {
                commands::open_editor(None)?;
            } else {
                // Show help
                println!("Usage: pyxel-rust [OPTIONS] [FILE]");
                println!("\nOptions:");
                println!("  --editor         Open resource editor");
                println!("  -v, --verbose    Verbose output");
                println!("  -h, --help       Show this help message");
                println!("\nCommands:");
                println!("  new <name>       Create a new project");
                println!("  run <file>       Run a game script");
                println!("  editor [file]    Open resource editor");
                println!("  build            Build project");
                println!("  version          Show version information");
                println!("\nExamples:");
                println!("  pyxel-rust my_game.rs");
                println!("  pyxel-rust run my_game.rs");
                println!("  pyxel-rust new my_project");
                println!("  pyxel-rust my_resources.pyxres --editor");
            }
        }
    }

    Ok(())
}
