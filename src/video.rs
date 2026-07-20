use crate::prelude::*;

/// Resolve the video path: explicit flag, or scan the chapters file's directory.
pub fn resolve_video(chapters_path: &Path, explicit: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = explicit {
        let p = p
            .canonicalize()
            .map_err(|e| Error::Path(format!("video {}: {e}", p.display())))?;
        if !p.is_file() {
            return Err(Error::Path(format!("video is not a file: {}", p.display())));
        }
        return Ok(p);
    }

    let chapters_path = chapters_path
        .canonicalize()
        .map_err(|e| Error::Path(format!("chapters file {}: {e}", chapters_path.display())))?;

    let dir = chapters_path
        .parent()
        .ok_or_else(|| Error::Path(format!("chapters path has no parent: {}", chapters_path.display())))?;

    let mut candidates: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            path.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|ext| COMMON_FORMATS.iter().any(|c| c.eq_ignore_ascii_case(ext)))
        })
        .collect();

    candidates.sort();

    match candidates.len() {
        0 => Err(Error::NoVideo(dir.to_path_buf())),
        1 => Ok(candidates.remove(0)),
        _ => {
            // Prefer names that don't look like duplicates (" copy", ".chapterized.").
            let preferred: Vec<PathBuf> = candidates
                .iter()
                .filter(|p| {
                    let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    let lower = name.to_ascii_lowercase();
                    !lower.contains(" copy") && !lower.contains(".chapterized.") && !lower.contains("_chapterized.")
                })
                .cloned()
                .collect();

            match preferred.len() {
                1 => Ok(preferred.into_iter().next().expect("len checked")),
                _ => {
                    Err(Error::AmbiguousVideo {
                        dir:        dir.to_path_buf(),
                        candidates: candidates.iter().map(|p| p.display().to_string()).collect(),
                    })
                }
            }
        }
    }
}

/// Default output path: `<stem>.chapterized.<ext>` beside the video.
pub fn default_output(video: &Path) -> Result<PathBuf> {
    let parent = video.parent().unwrap_or_else(|| Path::new("."));
    let stem = video
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| Error::Path(format!("bad video file name: {}", video.display())))?;
    let ext = video.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    Ok(parent.join(format!("{stem}.chapterized.{ext}")))
}
