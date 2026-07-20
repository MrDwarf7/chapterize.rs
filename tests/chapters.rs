use std::io::Write;

/// A helper to create a temporary chapters file for integration tests.
fn write_chapters_file(contents: &str) -> (tempfile::NamedTempFile, std::path::PathBuf) {
    let mut f = tempfile::NamedTempFile::new().expect("temp file");
    f.write_all(contents.as_bytes()).expect("write");
    let path = f.path().to_path_buf();
    (f, path)
}

#[test]
fn load_valid_chapters_from_file() {
    let (_f, path) = write_chapters_file(
        "00:00 - Introduction\n\
         01:23 - The Call of Cthulhu\n\
         02:45:33 - The Shadow over Innsmouth\n",
    );
    let chapters = chapterize::chapters::load_chapters(&path).unwrap();
    assert_eq!(chapters.len(), 3);
    assert_eq!(chapters[0].title, "Introduction");
    assert_eq!(chapters[0].start_ms, 0);
    assert_eq!(chapters[1].start_ms, 83000);
    assert_eq!(chapters[2].start_ms, (2 * 3600 + 45 * 60 + 33) * 1000);
}

#[test]
fn empty_chapters_file_errors() {
    let (_f, path) = write_chapters_file("");
    let err = chapterize::chapters::load_chapters(&path).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("empty") || msg.contains("no valid"));
}

#[test]
fn only_comments_and_blanks_errors() {
    let (_f, path) = write_chapters_file("# just a comment\n   \n# another\n");
    let err = chapterize::chapters::load_chapters(&path).unwrap_err();
    assert!(err.to_string().contains("empty") || err.to_string().contains("no valid"));
}

#[test]
fn load_rejects_out_of_order() {
    let (_f, path) = write_chapters_file(
        "00:10 - Second\n\
         00:05 - First (out of order)\n",
    );
    let err = chapterize::chapters::load_chapters(&path).unwrap_err();
    assert!(err.to_string().contains("out of order") || err.to_string().contains("order"));
}

#[test]
fn load_rejects_bad_timestamp() {
    let (_f, path) = write_chapters_file("not-a-timestamp - Title\n");
    let err = chapterize::chapters::load_chapters(&path).unwrap_err();
    assert!(err.to_string().contains("parse") || err.to_string().contains("chapter line"));
}
