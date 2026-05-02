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
    /// Run a Rust game example
    #[command(about = "Run a Rust game example")]
    Run {
        /// Game name (example directory name)
        name: String,
    },

    /// Convert Rust game to HTML/WASM and serve
    #[command(about = "Convert Rust game to HTML/WASM and serve locally")]
    App2Html {
        /// Game name (example directory name)
        name: String,

        /// Port number
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },

    /// Create a new Pyxel project
    #[command(about = "Create a new Pyxel-rust project")]
    New {
        /// Project name
        name: String,

        /// Template type (basic, game, minimal)
        #[arg(short, long, default_value = "basic")]
        template: String,
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
        Some(Commands::Run { name }) => {
            commands::run_example(&name)?;
        }

        Some(Commands::App2Html { name, port }) => {
            commands::app2html(&name, port)?;
        }

        Some(Commands::New { name, template }) => {
            commands::new_project(&name, &template)?;
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
            // Show help
            println!("Usage: pyxel-rust [OPTIONS] [COMMAND]");
            println!("\nCommands:");
            println!("  run <name>              Run a game example");
            println!("  app2html <name> [-p]    Convert game to HTML/WASM");
            println!("  new <name>              Create a new project");
            println!("  editor [file]           Open resource editor");
            println!("  build                   Build project");
            println!("  version                 Show version information");
            println!("\nOptions:");
            println!("  -v, --verbose          Verbose output");
            println!("  -h, --help             Show this help message");
            println!("\nExamples:");
            println!("  pyxel-rust run cubeboy");
            println!("  pyxel-rust app2html cubeboy");
            println!("  pyxel-rust app2html cubeboy -p 3000");
            println!("  pyxel-rust new my_project");
        }
    }

    Ok(())
}

