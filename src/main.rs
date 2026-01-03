use akari_theme::{ArtifactContent, Generator, Palette, Variant, find_project_root};
use clap::{Parser, Subcommand};
use std::fs;
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
        tool: String,

        /// Output directory (defaults to dist/)
        #[arg(long)]
        out_dir: Option<std::path::PathBuf>,
    },
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), akari_theme::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { tool, out_dir } => {
            let root = find_project_root()?;
            let out_root = out_dir.unwrap_or_else(|| root.join("dist"));

            // Load palettes
            let palette_dir = root.join("palette");
            let night = Palette::from_path(
                palette_dir.join(Variant::Night.palette_filename()),
                Variant::Night,
            )?;
            let dawn = Palette::from_path(
                palette_dir.join(Variant::Dawn.palette_filename()),
                Variant::Dawn,
            )?;

            let generator = Generator::new(root.join("templates"))?;

            // Get tools to generate
            let tools: Vec<String> = if tool == "all" {
                generator.available_tools()?
            } else {
                vec![tool]
            };

            // Generate and write artifacts
            for tool_name in &tools {
                let artifacts = generator.generate_tool(tool_name, &night, &dawn)?;
                for artifact in artifacts {
                    let output_path = out_root.join(&artifact.rel_path);

                    // Ensure parent directory exists
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    match &artifact.content {
                        ArtifactContent::Text(content) => {
                            fs::write(&output_path, content)?;
                        }
                        ArtifactContent::Copy(src) => {
                            fs::copy(src, &output_path)?;
                        }
                    }
                    println!("  {}", artifact.rel_path.display());
                }
            }
        }
    }

    Ok(())
}
