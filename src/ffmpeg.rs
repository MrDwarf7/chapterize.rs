use std::ffi::OsString;
use std::process::Command;

use crate::chapters::{format_ffmetadata, Chapter};
use crate::prelude::*;

/// Locate `ffmpeg` / `ffprobe`.
///
/// Order:
/// 1. Directory of the running `chapterize` binary (release-zip sibling layout)
/// 2. `PATH` (system install)
///
/// Windows also tries `name.exe`.
pub fn find_tool(name: &str) -> Result<PathBuf> {
    if let Some(path) = find_beside_exe(name) {
        return Ok(path);
    }
    find_on_path(name)
}

fn find_beside_exe(name: &str) -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    for candidate in binary_candidates(name) {
        let full = dir.join(&candidate);
        if full.is_file() {
            return Some(full);
        }
    }
    None
}

/// Locate `name` on PATH. Windows also tries `name.exe`.
pub fn find_on_path(name: &str) -> Result<PathBuf> {
    let path_os = std::env::var_os("PATH").ok_or_else(|| Error::BinaryNotFound(name.into()))?;

    for dir in std::env::split_paths(&path_os) {
        for candidate in binary_candidates(name) {
            let full = dir.join(&candidate);
            if full.is_file() {
                return Ok(full);
            }
        }
    }

    Err(Error::BinaryNotFound(name.into()))
}

fn binary_candidates(name: &str) -> Vec<OsString> {
    #[cfg(windows)]
    {
        let mut out = Vec::with_capacity(2);
        out.push(OsString::from(name));
        if !name.ends_with(".exe") && !name.ends_with(".EXE") {
            out.push(OsString::from(format!("{name}.exe")));
        }
        out
    }

    #[cfg(not(windows))]
    {
        vec![OsString::from(name)]
    }
}

/// Duration of `video` in milliseconds via ffprobe.
pub fn probe_duration_ms(ffprobe: &Path, video: &Path) -> Result<u64> {
    let output = Command::new(ffprobe)
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(video)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::Duration(stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let secs: f64 = stdout
        .trim()
        .parse()
        .map_err(|e| Error::Duration(format!("could not parse duration {stdout:?}: {e}")))?;

    if secs <= 0.0 {
        return Err(Error::Duration(format!("non-positive duration: {secs}")));
    }

    Ok((secs * 1000.0).round() as u64)
}

pub struct RunOpts<'a> {
    pub video:     &'a Path,
    pub output:    &'a Path,
    pub chapters:  &'a [Chapter],
    pub overwrite: bool,
    pub dry_run:   bool,
}

/// Write temp FFMETADATA and invoke ffmpeg to mux chapters onto a stream-copy.
pub fn chapterize(opts: RunOpts<'_>) -> Result<()> {
    let ffmpeg = find_tool(FFMPEG_BINARY_NAME)?;
    let ffprobe = find_tool(FFPROBE_BINARY_NAME)?;

    info!("ffmpeg: {}", ffmpeg.display());
    info!("ffprobe: {}", ffprobe.display());

    let duration_ms = probe_duration_ms(&ffprobe, opts.video)?;
    info!("duration: {duration_ms} ms");

    let meta_body = format_ffmetadata(opts.chapters, duration_ms)?;
    debug!("ffmetadata:\n{meta_body}");

    let meta_path = std::env::temp_dir().join(format!("chapterize-{}-{}.ffmeta", std::process::id(), timestamp_slug()));

    std::fs::write(&meta_path, meta_body.as_bytes())?;
    info!("wrote metadata: {}", meta_path.display());

    let mut cmd = Command::new(&ffmpeg);
    cmd.arg("-hide_banner").arg("-loglevel").arg("error");

    if opts.overwrite {
        cmd.arg("-y");
    } else {
        cmd.arg("-n"); // fail if output exists
    }

    cmd.arg("-i")
        .arg(opts.video)
        .arg("-i")
        .arg(&meta_path)
        .arg("-map_metadata")
        .arg("0")
        .arg("-map_chapters")
        .arg("1")
        .arg("-codec")
        .arg("copy")
        .arg(opts.output);

    info!("command: {cmd:?}");

    if opts.dry_run {
        warn!("dry-run: not invoking ffmpeg");
        let _ = std::fs::remove_file(&meta_path);
        return Ok(());
    }

    let result = cmd.output();
    let _ = std::fs::remove_file(&meta_path);

    let output = result?;
    if !output.status.success() {
        let status = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(Error::FfmpegFailed { status, stderr });
    }

    info!("wrote {}", opts.output.display());
    Ok(())
}

fn timestamp_slug() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
