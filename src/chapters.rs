use crate::prelude::*;

/// One chapter marker: title + start time in milliseconds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    pub title:    String,
    pub start_ms: u64,
}

impl Chapter {
    /// Parse a single YT-style chapter line into start_ms + title.
    ///
    /// Pattern groups: optional hours, mins, secs, title.
    /// `MM:SS` => hours=0; `HH:MM:SS` / `H:MM:SS` use the leading group.
    pub fn parse_line(line_no: usize, line: &str) -> Result<Option<Self>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let caps = RE_CHAPTER.captures(line).ok_or_else(|| {
            Error::InvalidChapter {
                line_no,
                line: line.to_string(),
            }
        })?;

        let hours: u64 = caps.get(1).map(|m| m.as_str().parse().unwrap_or(0)).unwrap_or(0);
        let mins: u64 = caps
            .get(2)
            .ok_or_else(|| {
                Error::BadTimestamp {
                    line_no,
                    line: line.to_string(),
                }
            })?
            .as_str()
            .parse()
            .map_err(|_| {
                Error::BadTimestamp {
                    line_no,
                    line: line.to_string(),
                }
            })?;
        let secs: u64 = caps
            .get(3)
            .ok_or_else(|| {
                Error::BadTimestamp {
                    line_no,
                    line: line.to_string(),
                }
            })?
            .as_str()
            .parse()
            .map_err(|_| {
                Error::BadTimestamp {
                    line_no,
                    line: line.to_string(),
                }
            })?;
        let title = caps
            .get(4)
            .ok_or_else(|| {
                Error::InvalidChapter {
                    line_no,
                    line: line.to_string(),
                }
            })?
            .as_str()
            .trim()
            .to_string();

        if title.is_empty() {
            return Err(Error::InvalidChapter {
                line_no,
                line: line.to_string(),
            });
        }

        let start_ms = ((hours * 3600) + (mins * 60) + secs) * 1000;

        Ok(Some(Self { title, start_ms }))
    }
}

/// Read and parse every chapter line from `path`.
pub fn load_chapters(path: &Path) -> Result<Vec<Chapter>> {
    let contents = std::fs::read_to_string(path)?;
    let mut chapters = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        let line_no = idx + 1;
        if let Some(ch) = Chapter::parse_line(line_no, line)? {
            chapters.push(ch);
        }
    }

    if chapters.is_empty() {
        return Err(Error::NoChapters);
    }

    // Chapters must be non-decreasing in time.
    for pair in chapters.windows(2) {
        if pair[1].start_ms < pair[0].start_ms {
            return Err(Error::Path(format!(
                "chapters out of order: {:?} ({}ms) then {:?} ({}ms)",
                pair[0].title, pair[0].start_ms, pair[1].title, pair[1].start_ms
            )));
        }
    }

    Ok(chapters)
}

/// Build FFMETADATA chapter blocks. `duration_ms` ends the final chapter.
pub fn format_ffmetadata(chapters: &[Chapter], duration_ms: u64) -> Result<String> {
    if chapters.is_empty() {
        return Err(Error::NoChapters);
    }

    let mut out = String::from(";FFMETADATA1\n");

    for (i, chapter) in chapters.iter().enumerate() {
        let end_ms = if i + 1 < chapters.len() {
            chapters[i + 1].start_ms.saturating_sub(1)
        } else {
            duration_ms.saturating_sub(1).max(chapter.start_ms)
        };

        // Escape special chars per ffmetadata rules: `\`, `=`, `;`, `#`, `\n`
        let title = escape_ffmeta(&chapter.title);

        out.push_str(&format!(
            "\n[CHAPTER]\nTIMEBASE=1/1000\nSTART={}\nEND={}\ntitle={}\n",
            chapter.start_ms, end_ms, title
        ));
    }

    Ok(out)
}

fn escape_ffmeta(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '=' | ';' | '#' | '\n' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hh_mm_ss() {
        let ch = Chapter::parse_line(1, "01:17:22 - The Call of Cthulhu (1/3)")
            .unwrap()
            .unwrap();
        assert_eq!(ch.title, "The Call of Cthulhu (1/3)");
        assert_eq!(ch.start_ms, (3600 + (17 * 60) + 22) * 1000);
    }

    #[test]
    fn parse_mm_ss() {
        let ch = Chapter::parse_line(1, "03:12 - Cold Open").unwrap().unwrap();
        assert_eq!(ch.start_ms, (3 * 60 + 12) * 1000);
    }

    #[test]
    fn skips_blank_and_comment() {
        assert!(Chapter::parse_line(1, "   ").unwrap().is_none());
        assert!(Chapter::parse_line(1, "# note").unwrap().is_none());
    }

    #[test]
    fn metadata_ends_last_chapter() {
        let chapters = vec![
            Chapter {
                title:    "A".into(),
                start_ms: 0,
            },
            Chapter {
                title:    "B".into(),
                start_ms: 1000,
            },
        ];
        let meta = format_ffmetadata(&chapters, 5000).unwrap();
        assert!(meta.contains("START=0"));
        assert!(meta.contains("END=999"));
        assert!(meta.contains("START=1000"));
        assert!(meta.contains("END=4999"));
        assert!(meta.starts_with(";FFMETADATA1"));
    }

    #[test]
    fn parse_line_errors_on_bad_format() {
        // No dash separator
        assert!(Chapter::parse_line(1, "garbage").is_err());
        // Text before timestamp
        assert!(Chapter::parse_line(1, "intro 00:30").is_err());
    }

    #[test]
    fn parse_line_empty_title_errors() {
        let result = Chapter::parse_line(1, "00:10 -   ");
        assert!(result.is_err());
    }

    #[test]
    fn parse_single_digit_hour() {
        let ch = Chapter::parse_line(1, "1:15:30 - Episode").unwrap().unwrap();
        assert_eq!(ch.start_ms, (3600 + (15 * 60) + 30) * 1000);
    }

    #[test]
    fn escape_ffmeta_backslash() {
        assert_eq!(escape_ffmeta(r"a\b"), r"a\\b");
    }

    #[test]
    fn escape_ffmeta_equals_sign() {
        assert_eq!(escape_ffmeta("key=value"), r"key\=value");
    }

    #[test]
    fn escape_ffmeta_newline() {
        // The function prefixes real newlines with backslash.
        assert_eq!(escape_ffmeta("A\nB"), "A\\\nB");
    }

    #[test]
    fn load_chapters_errors_on_missing_file() {
        let result = load_chapters(Path::new("/nonexistent/path/chapters.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn format_ffmetadata_empty_errors() {
        let result = format_ffmetadata(&[], 1000);
        assert!(result.is_err());
    }

    #[test]
    fn format_ffmetadata_single_chapter() {
        let chapters = vec![Chapter { title: "Only".into(), start_ms: 5000 }];
        let meta = format_ffmetadata(&chapters, 10000).unwrap();
        assert!(meta.contains("START=5000"));
        assert!(meta.contains("END=9999"));
    }
}
