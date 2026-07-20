use clap::Parser;

use crate::prelude::*;

/// Apply YT-style chapter markers to a video via ffmpeg.
#[derive(Parser, Debug)]
#[command(
    name = "chapterize",
    about = "Apply YouTube-style chapters to a video with ffmpeg.",
    author = "Blake B./MrDwarf7",
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Path to the chapters.txt file (lines: `HH:MM:SS - Title`).
    #[arg(index = 1, value_hint = clap::ValueHint::FilePath)]
    pub input_chapters: PathBuf,

    /// Video to chapterize. Default: scan the chapters file's directory.
    #[arg(short = 'v', long = "video", value_hint = clap::ValueHint::FilePath)]
    pub video: Option<PathBuf>,

    /// Output path. Default: `<video_stem>.chapterized.<ext>` next to the video.
    #[arg(short = 'o', long = "output", value_hint = clap::ValueHint::FilePath)]
    pub output: Option<PathBuf>,

    /// Overwrite the output file if it already exists (passes -y to ffmpeg).
    #[arg(short = 'y', long = "yes", default_value_t = false)]
    pub overwrite: bool,

    /// Print the ffmpeg command and metadata, do not run ffmpeg.
    #[arg(long = "dry-run", default_value_t = false)]
    pub dry_run: bool,
}

impl Cli {
    #[inline]
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
