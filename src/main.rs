use akari_theme::theme::Theme;
use akari_theme::{Artifact, ArtifactContent, Generator, Palette, Variant, find_project_root};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::{Path, PathBuf};
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
    /// Generate theme files from the legacy palette pair
    Generate {
        /// Target tool (or 'all' to generate for all tools)
        #[arg(long)]
        tool: String,

        /// Output directory (defaults to dist/)
        #[arg(long)]
        out_dir: Option<PathBuf>,
    },
    /// Generate theme files from a `Theme` directory
    GenerateTheme {
        /// Directory containing theme.toml and its variant files
        #[arg(long)]
        theme_dir: PathBuf,

        /// Target tool (or 'all' to generate for all theme-based tools)
        #[arg(long)]
        tool: String,

        /// Output directory
        #[arg(long)]
        out_dir: PathBuf,
    },
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Writes every artifact under `out_root`, creating parent directories as needed.
/// Sets the mode explicitly because `fs::write` keeps an existing file's mode.
#[cfg(unix)]
fn set_executable(path: &Path, executable: bool) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mode = if executable { 0o755 } else { 0o644 };
    fs::set_permissions(path, fs::Permissions::from_mode(mode))
}

#[cfg(not(unix))]
fn set_executable(_path: &Path, _executable: bool) -> std::io::Result<()> {
    Ok(())
}

fn write_artifacts(artifacts: Vec<Artifact>, out_root: &Path) -> Result<(), akari_theme::Error> {
    for artifact in artifacts {
        let output_path = out_root.join(&artifact.rel_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        match &artifact.content {
            ArtifactContent::Text(content) => {
                fs::write(&output_path, content)?;
                set_executable(&output_path, artifact.executable)?;
            }
            ArtifactContent::Copy(src) => {
                fs::copy(src, &output_path)?;
            }
        }
        println!("  {}", artifact.rel_path.display());
    }
    Ok(())
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

            for tool_name in &tools {
                let artifacts = generator.generate_tool(tool_name, &night, &dawn)?;
                write_artifacts(artifacts, &out_root)?;
            }
        }
        Command::GenerateTheme {
            theme_dir,
            tool,
            out_dir,
        } => {
            let root = find_project_root()?;
            let theme = Theme::load(&theme_dir)?;
            let generator = Generator::new(root.join("templates"))?;

            let tools: Vec<String> = if tool == "all" {
                generator.available_theme_tools()
            } else {
                vec![tool]
            };

            for tool_name in &tools {
                let artifacts = generator.generate_theme_tool(tool_name, &theme, &theme_dir)?;
                write_artifacts(artifacts, &out_dir)?;
            }
        }
    }

    Ok(())
}
