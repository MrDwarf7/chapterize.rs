# chapterize

Apply YouTube-style chapter markers to a video using ffmpeg.

```
HH:MM:SS - Title
00:01:26 - The Nameless City
00:33:12 - The Hound (1/2)
```

## Requirements

`ffmpeg` and `ffprobe` must be available either:

1. **Next to the `chapterize` binary** (release zip layout), or
2. **On your `PATH`** (system install)

Lookup is OS-aware (`ffmpeg` on Unix; `ffmpeg` / `ffmpeg.exe` on Windows).

### System install

- Arch: `pacman -S ffmpeg`
- macOS: `brew install ffmpeg`
- Windows: `winget install ffmpeg` (or use the release zip)

### Release zip

GitHub Releases ship per-target zips:

```text
chapterize-<target>-<tag>.zip
  chapterize[.exe]
  ffmpeg[.exe]
  ffprobe[.exe]
  README.md
  LICENSE
  FFMPEG_NOTICE.txt
  third_party/ffmpeg/   # upstream attribution / licenses
```

Targets:

| OS | Arch | Triple |
|----|------|--------|
| Linux | x86_64 | `x86_64-unknown-linux-gnu` |
| Linux | arm64 | `aarch64-unknown-linux-gnu` |
| Windows | x86_64 | `x86_64-pc-windows-msvc` |
| Windows | arm64 | `aarch64-pc-windows-msvc` |
| macOS | Intel | `x86_64-apple-darwin` |
| macOS | Apple Silicon | `aarch64-apple-darwin` |

Bundled FFmpeg comes from:

- Linux/Windows: [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (LGPL static)
- macOS: [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static)

FFmpeg remains a **separate** program under its own license (see `FFMPEG_NOTICE.txt`). You can delete the bundled binaries and use a system install instead.

## Usage

```bash
# chapters.txt sits next to the video; scans the folder for mp4/mkv/avi/mov/webm/m4v
chapterize path/to/chapters.txt

# explicit video + output
chapterize chapters.txt -v movie.mp4 -o movie.with-chapters.mp4 -y

# print metadata + ffmpeg argv without writing
chapterize chapters.txt --dry-run
```

### Flags

| Flag | Meaning |
|------|---------|
| `-v, --video` | Video path (default: scan chapters file's directory) |
| `-o, --output` | Output path (default: `<stem>.chapterized.<ext>`) |
| `-y, --yes` | Overwrite output if it exists |
| `--dry-run` | Build metadata and show the command; do not run ffmpeg |

### Logging

```bash
RUST_LOG=debug chapterize chapters.txt
```

Default log level is `info`.

## How it works

1. Parse YT chapter lines (`H:MM:SS` / `HH:MM:SS` / `MM:SS` + ` - ` + title).
2. Resolve the video (flag, or unique match in the same folder).
3. `ffprobe` duration for the final chapter `END=`.
4. Write a temp `;FFMETADATA1` file.
5. `ffmpeg -i video -i meta -map_metadata 0 -map_chapters 1 -codec copy out`.

Stream copy only — no re-encode.

## Build

```bash
cargo build --release
cargo test
cargo make a   # if cargo-make is installed (see Makefile.toml)
```

Local fetch of arch-matched ffmpeg into a directory (same script CI uses):

```bash
./build/fetch-ffmpeg.sh x86_64-unknown-linux-gnu /tmp/chapterize-dist
```

## Notes

- If the chapters folder has multiple videos, pass `--video` (names containing ` copy` or `.chapterized.` are ignored when a single preferred candidate remains).
- `old/` holds the previous half-working implementation; safe to delete once you're happy with `src/`.
