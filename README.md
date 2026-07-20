# chapterize.rs

Apply YouTube-style chapter markers to a video using ffmpeg.

## How it works

1. Parse chapter lines (`H:MM:SS`, `HH:MM:SS`, or `MM:SS` + `-` + title).
2. Resolve the video (via `--video` flag or auto-detect in the same folder).
3. `ffprobe` probes duration for the final chapter end time.
4. Write a temp `;FFMETADATA1` file.
5. `ffmpeg -i video -i meta -map_metadata 0 -map_chapters 1 -codec copy out`.

Stream copy only, no re-encode.

## Usage

```bash
# chapters.txt next to the video; auto-scans the folder for mp4/mkv/avi/mov/webm/m4v
chapterize path/to/chapters.txt

# explicit video + output
chapterize chapters.txt -v movie.mp4 -o movie.with-chapters.mp4 -y

# print metadata + ffmpeg argv, don't run ffmpeg
chapterize chapters.txt --dry-run
```

### Flags

| Flag           | Meaning                                             |
| -------------- | --------------------------------------------------- |
| `(positional)` | Path to chapters file                               |
| `-v, --video`  | Video path. Default: scan chapters file's directory |
| `-o, --output` | Output path. Default: `<stem>.chapterized.<ext>`    |
| `-y, --yes`    | Overwrite output if it exists                       |
| `--dry-run`    | Build metadata, show the command, do not run ffmpeg |

Logging: `RUST_LOG=debug chapterize chapters.txt` (default level is `info`).

## Install

### One-liner (Linux/macOS)

```bash
curl -fsSL https://github.com/MrDwarf7/chapterize.rs/raw/main/build/install.sh | sh
```

Downloads the pre-built binary for your OS/arch from the latest GitHub release
and installs it to `/usr/local/bin` (or `~/.local/bin` if that's not writable).
Set `CHAPTERIZE_VERSION=v0.1.0` to pin a specific version.

### Cargo

```bash
cargo install chapterize
```

### System packages

| OS      | Command                 |
| ------- | ----------------------- |
| Arch    | `pacman -S ffmpeg`      |
| macOS   | `brew install ffmpeg`   |
| Windows | `winget install ffmpeg` |

### Release zip

GitHub Releases ship per-target archives with `ffmpeg` and `ffprobe` bundled:

```
chapterize-<target>-<tag>.zip
  chapterize[.exe]
  ffmpeg[.exe]
  ffprobe[.exe]
  README.md
  LICENSE
  FFMPEG_NOTICE.txt
  third_party/ffmpeg/
```

Targets:

| OS      | Arch          | Triple                      |
| ------- | ------------- | --------------------------- |
| Linux   | x86_64        | `x86_64-unknown-linux-gnu`  |
| Linux   | arm64         | `aarch64-unknown-linux-gnu` |
| Windows | x86_64        | `x86_64-pc-windows-msvc`    |
| Windows | arm64         | `aarch64-pc-windows-msvc`   |
| macOS   | Intel         | `x86_64-apple-darwin`       |
| macOS   | Apple Silicon | `aarch64-apple-darwin`      |

FFmpeg remains a separate program under its own license. You can delete the bundled binaries and use a system install instead.

Bundled FFmpeg source:

- Linux/Windows: [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (LGPL static)
- macOS: [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static)

## Build

```bash
cargo build --release
cargo test
cargo make a       # if cargo-make is installed
```

Local ffmpeg fetch (same script CI uses):

```bash
./build/fetch-ffmpeg.sh x86_64-unknown-linux-gnu /tmp/chapterize-dist
```
