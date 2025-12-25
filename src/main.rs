use akari_gen::{Generator, find_project_root, tools};
use clap::{Parser, Subcommand, ValueEnum};
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
        tool: CliTool,

        /// Output directory (defaults to project root)
        #[arg(long)]
        out_dir: Option<std::path::PathBuf>,
    },
}

#[derive(Clone, ValueEnum)]
enum CliTool {
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

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), akari_gen::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { tool, out_dir } => {
            let root = find_project_root()?;
            let out_root = out_dir.unwrap_or_else(|| root.join("dist"));

            // Load palettes once
            let palettes = tools::Palettes::load(&root.join("palette"))?;
            let generator = Generator::new(root.join("templates"))?;

            // Get tools to generate
            let generators: Vec<Box<dyn tools::ThemeGenerator>> = match tool {
                CliTool::All => tools::all(),
                _ => {
                    let name = match tool {
                        CliTool::Helix => "helix",
                        CliTool::Ghostty => "ghostty",
                        CliTool::Fzf => "fzf",
                        CliTool::Starship => "starship",
                        CliTool::Tmux => "tmux",
                        CliTool::Zsh => "zsh",
                        CliTool::Nvim => "nvim",
                        CliTool::Vscode => "vscode",
                        CliTool::Terminal => "terminal",
                        CliTool::Chrome => "chrome",
                        CliTool::All => unreachable!(),
                    };
                    tools::by_name(name).into_iter().collect()
                }
            };

            // Generate and write artifacts
            for tool in generators {
                let artifacts = tool.artifacts(&palettes, &generator)?;
                for artifact in artifacts {
                    let output_path = out_root.join(&artifact.rel_path);

                    // Ensure parent directory exists
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)?;
                    }

                    fs::write(&output_path, &artifact.content)?;
                    println!("Generated: {}", output_path.display());
                }
            }
        }
    }

    Ok(())
}
