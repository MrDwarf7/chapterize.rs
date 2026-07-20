mod chapters;
mod cli;
mod error;
mod ffmpeg;
mod prelude;
mod video;

use chapters::load_chapters;
use cli::Cli;
use ffmpeg::chapterize;
use prelude::*;
use video::{default_output, resolve_video};

fn main() {
    init_logging();
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse_args();
    debug!("cli: {cli:?}");

    let chapters_path = cli.input_chapters.canonicalize().map_err(|e| {
        Error::Path(format!(
            "chapters file {}: {e}",
            cli.input_chapters.display()
        ))
    })?;

    let chapters = load_chapters(&chapters_path)?;
    info!(
        "loaded {} chapters from {}",
        chapters.len(),
        chapters_path.display()
    );

    let video = resolve_video(&chapters_path, cli.video.as_deref())?;
    info!("video: {}", video.display());

    let output = match cli.output {
        Some(p) => p,
        None => default_output(&video)?,
    };
    info!("output: {}", output.display());

    chapterize(ffmpeg::RunOpts {
        video: &video,
        output: &output,
        chapters: &chapters,
        overwrite: cli.overwrite,
        dry_run: cli.dry_run,
    })?;

    Ok(())
}

fn init_logging() {
    let mut builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    builder.format_timestamp_secs();
    let _ = builder.try_init();
}
