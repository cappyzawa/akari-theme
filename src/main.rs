use clap::{Parser, Subcommand};
use katazome::theme::Theme;
use katazome::{Artifact, ArtifactContent, Generator};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "katazome")]
#[command(about = "Generate theme files from a theme directory")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate theme files from a `Theme` directory
    Generate {
        /// Directory containing theme.toml and its variant files
        #[arg(long)]
        theme_dir: PathBuf,

        /// Target tool (or 'all' to generate for all theme-based tools)
        #[arg(long)]
        tool: String,

        /// Output directory
        #[arg(long)]
        out_dir: PathBuf,

        /// Read templates from this directory instead of the ones built into katazome
        #[arg(long)]
        templates_dir: Option<PathBuf>,
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

fn write_artifacts(artifacts: Vec<Artifact>, out_root: &Path) -> Result<(), katazome::Error> {
    for artifact in artifacts {
        let output_path = out_root.join(&artifact.rel_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        match &artifact.content {
            ArtifactContent::Text(content) => fs::write(&output_path, content)?,
            ArtifactContent::Bytes(bytes) => fs::write(&output_path, bytes)?,
        }
        set_executable(&output_path, artifact.executable)?;
        println!("  {}", artifact.rel_path.display());
    }
    Ok(())
}

fn run() -> Result<(), katazome::Error> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            theme_dir,
            tool,
            out_dir,
            templates_dir,
        } => {
            let theme = Theme::load(&theme_dir)?;
            let generator = match templates_dir {
                Some(dir) => Generator::new(dir)?,
                None => Generator::embedded()?,
            };

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
