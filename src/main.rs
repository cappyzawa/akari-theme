mod generator;
mod palette;
mod terminal;

use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "akari-gen")]
#[command(about = "Generate akari theme files from palette definitions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate theme files
    Generate {
        /// Target tool (or 'all' to generate for all tools)
        #[arg(long)]
        tool: Tool,

        /// Theme variant (night or dawn). If not specified, generates both.
        #[arg(long)]
        variant: Option<Variant>,
    },
}

#[derive(Clone, ValueEnum)]
enum Tool {
    Helix,
    Ghostty,
    Fzf,
    Starship,
    Tmux,
    Zsh,
    Nvim,
    Vscode,
    Terminal,
    Chrome,
    All,
}

#[derive(Clone, ValueEnum)]
enum Variant {
    Night,
    Dawn,
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { tool, variant } => {
            let variants = match variant {
                Some(v) => vec![v],
                None => vec![Variant::Night, Variant::Dawn],
            };

            let tools = match tool {
                Tool::All => vec![
                    Tool::Helix,
                    Tool::Ghostty,
                    Tool::Fzf,
                    Tool::Starship,
                    Tool::Tmux,
                    Tool::Zsh,
                    Tool::Nvim,
                    Tool::Vscode,
                    Tool::Terminal,
                    Tool::Chrome,
                ],
                t => vec![t],
            };

            for t in &tools {
                match t {
                    Tool::Zsh | Tool::Nvim => {
                        // These generate a single file containing both variants
                        generate_combined(t)?;
                    }
                    Tool::All => unreachable!(),
                    _ => {
                        for v in &variants {
                            generate(t, v)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn generate(tool: &Tool, variant: &Variant) -> Result<(), Box<dyn std::error::Error>> {
    let root = find_project_root()?;

    let palette_path = match variant {
        Variant::Night => root.join("palette/akari-night.toml"),
        Variant::Dawn => root.join("palette/akari-dawn.toml"),
    };

    let palette = palette::Palette::from_path(&palette_path)?;
    let generator = generator::Generator::new(root.join("templates"))?;

    let variant_name = match variant {
        Variant::Night => "night",
        Variant::Dawn => "dawn",
    };

    let (template, output_path) = match tool {
        Tool::Helix => (
            "helix.tera",
            root.join(format!("helix/akari-{variant_name}.toml")),
        ),
        Tool::Ghostty => ("ghostty.tera", root.join(format!("ghostty/akari-{variant_name}"))),
        Tool::Fzf => (
            "fzf.tera",
            root.join(format!("fzf/akari-{variant_name}.sh")),
        ),
        Tool::Starship => (
            "starship.tera",
            root.join(format!("starship/akari-{variant_name}.toml")),
        ),
        Tool::Tmux => (
            "tmux.tera",
            root.join(format!("tmux/akari-{variant_name}.conf")),
        ),
        Tool::Vscode => (
            "vscode.tera",
            root.join(format!("vscode/themes/akari-{variant_name}-color-theme.json")),
        ),
        Tool::Chrome => (
            "chrome.tera",
            root.join(format!("chrome/{variant_name}/manifest.json")),
        ),
        Tool::Terminal => {
            // Terminal uses a special generator, not templates
            let variant_title = if variant_name == "night" { "Night" } else { "Dawn" };
            let output_path = root.join(format!("terminal/Akari-{variant_title}.terminal"));
            let content = terminal::generate(&palette)?;
            fs::write(&output_path, content)?;
            println!("Generated: {}", output_path.display());
            return Ok(());
        }
        Tool::Zsh | Tool::Nvim | Tool::All => unreachable!(),
    };

    let content = generator.render(template, &palette)?;
    fs::write(&output_path, content)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn generate_combined(tool: &Tool) -> Result<(), Box<dyn std::error::Error>> {
    let root = find_project_root()?;

    let night = palette::Palette::from_path(root.join("palette/akari-night.toml"))?;
    let dawn = palette::Palette::from_path(root.join("palette/akari-dawn.toml"))?;
    let generator = generator::Generator::new(root.join("templates"))?;

    let (template, output_path) = match tool {
        Tool::Zsh => ("zsh.tera", root.join("zsh/akari.zsh")),
        Tool::Nvim => ("nvim-palette.tera", root.join("nvim/lua/akari/palette.lua")),
        _ => unreachable!(),
    };

    let content = generator.render_combined(template, &night, &dawn)?;
    fs::write(&output_path, content)?;

    println!("Generated: {}", output_path.display());
    Ok(())
}

fn find_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current = std::env::current_dir()?;

    loop {
        if current.join("palette").is_dir() && current.join("Cargo.toml").is_file() {
            return Ok(current);
        }

        if !current.pop() {
            return Err(
                "Could not find project root (directory containing palette/ and Cargo.toml)".into(),
            );
        }
    }
}
