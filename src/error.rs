use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid chapter line {line_no}: {line}")]
    InvalidChapter { line_no: usize, line: String },

    #[error("failed to parse timestamp on line {line_no}: {line}")]
    BadTimestamp { line_no: usize, line: String },

    #[error("no video file found in {0}")]
    NoVideo(PathBuf),

    #[error(
        "multiple video files in {dir}; pass --video to pick one:\n  - {candidates}",
        candidates = .candidates.join("\n  - ")
    )]
    AmbiguousVideo {
        dir:        PathBuf,
        candidates: Vec<String>,
    },

    #[error("could not find `{0}` on PATH (is it installed?)")]
    BinaryNotFound(String),

    #[error("ffmpeg exited with status {status}:\n{stderr}")]
    FfmpegFailed { status: i32, stderr: String },

    #[error("ffprobe could not read duration: {0}")]
    Duration(String),

    #[error("{0}")]
    Path(String),

    #[error("chapters file is empty or has no valid chapter lines")]
    NoChapters,
}

pub type Result<T> = std::result::Result<T, Error>;
