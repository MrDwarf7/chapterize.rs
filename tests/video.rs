use std::path::PathBuf;

use tempfile::TempDir;

/// Create a tempdir with the given dummy video files.
fn setup_video_dir(files: &[&str]) -> TempDir {
    let dir = TempDir::new().expect("temp dir");
    for name in files {
        let p = dir.path().join(name);
        std::fs::write(p, "dummy content").expect("write dummy file");
    }
    dir
}

#[test]
fn resolve_video_finds_single_file() {
    let dir = setup_video_dir(&["test.mp4"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();

    let found = chapterize::video::resolve_video(&chapters, None).unwrap();
    assert_eq!(found.file_name().unwrap(), "test.mp4");
}

#[test]
fn resolve_video_finds_stem_match() {
    let dir = setup_video_dir(&["chapters.txt", "other.mkv"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();

    let found = chapterize::video::resolve_video(&chapters, None).unwrap();
    assert_eq!(found.file_name().unwrap(), "other.mkv");
}

#[test]
fn resolve_video_ambiguous_when_many() {
    let dir = setup_video_dir(&["a.mp4", "b.mp4", "chapters.txt"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();

    let result = chapterize::video::resolve_video(&chapters, None);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("multiple") || msg.contains("Ambiguous"), "got {msg}");
}

#[test]
fn resolve_video_prefers_non_copy() {
    let dir = setup_video_dir(&["original.mp4", "original.chapterized.mp4", "chapters.txt"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();

    let found = chapterize::video::resolve_video(&chapters, None).unwrap();
    assert_eq!(found.file_name().unwrap(), "original.mp4", "should skip .chapterized. variant");
}

#[test]
fn resolve_video_errors_on_no_video() {
    let dir = setup_video_dir(&["chapters.txt", "readme.pdf", "notes.txt"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();

    let result = chapterize::video::resolve_video(&chapters, None);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("no video") || msg.contains("NoVideo"), "got {msg}");
}

#[test]
fn default_output_appends_chapterized() {
    let video = PathBuf::from("path").join("to").join("video.mp4");
    let out = chapterize::video::default_output(&video).unwrap();
    assert_eq!(out.file_name().and_then(|n| n.to_str()), Some("video.chapterized.mp4"));
    assert_eq!(out.parent(), video.parent());
}

#[test]
fn default_output_handles_other_extension() {
    let video = PathBuf::from("cap.webm");
    let out = chapterize::video::default_output(&video).unwrap();
    assert_eq!(out.to_str().unwrap(), "cap.chapterized.webm");
}

#[test]
fn explicit_video_overrides_scan() {
    let dir = setup_video_dir(&["a.mp4", "b.mp4", "chapters.txt"]);
    let chapters = dir.path().join("chapters.txt");
    std::fs::write(&chapters, "00:00 - Test\n").unwrap();
    let explicit = dir.path().join("b.mp4");

    let found = chapterize::video::resolve_video(&chapters, Some(&explicit)).unwrap();
    // resolve_video canonicalizes; compare file names / canonical forms.
    assert_eq!(found.file_name().unwrap(), "b.mp4");
    assert_eq!(found, explicit.canonicalize().unwrap());
}
