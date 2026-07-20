use std::path::PathBuf;

#[test]
fn find_on_path_finds_ffmpeg() {
    // Must be available in test CI.
    let found = chapterize::ffmpeg::find_on_path("ffmpeg");
    assert!(found.is_ok(), "ffmpeg should be on PATH in CI: {:?}", found);
    let path = found.unwrap();
    assert!(path.is_file(), "found path should be a file: {}", path.display());
}

#[test]
fn find_on_path_missing_errors() {
    let result = chapterize::ffmpeg::find_on_path("binary-that-definitely-does-not-exist-12345");
    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("could not find") || msg.contains("not found"));
}

#[test]
fn probe_duration_ms_needs_real_ffprobe_or_errors_gracefully() {
    // Without a real video, this should fail with a clear error, not panic.
    let missing = PathBuf::from("/tmp/chapterize-test-nonexistent-video-12345.mp4");
    // We can still get ffprobe to look up a missing file.
    let path = chapterize::ffmpeg::find_on_path("ffprobe").unwrap();
    let result = chapterize::ffmpeg::probe_duration_ms(&path, &missing);
    assert!(result.is_err(), "should fail on missing video: {:?}", result);
    let err = result.unwrap_err().to_string();
    // Should mention the error, not panic.
    assert!(!err.is_empty(), "error message should not be empty");
}
