pub use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use log::{debug, info, warn};
use regex::Regex;

pub use crate::error::{Error, Result};

/// YT-style chapter line:
///   `HH:MM:SS - Title`
///   `H:MM:SS - Title`
///   `MM:SS - Title`
/// Optional trailing part marker is kept inside the title: `(1/10)`.
pub static RE_CHAPTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:(\d{1,2}):)?(\d{1,2}):(\d{2})\s+-\s+(.+?)\s*$").expect("chapter regex"));

/// Extensions we will treat as the source video when scanning a folder.
pub static COMMON_FORMATS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "m4v"];

pub const FFMPEG_BINARY_NAME: &str = "ffmpeg";
pub const FFPROBE_BINARY_NAME: &str = "ffprobe";
