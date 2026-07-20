<!-- PROJECT LOGO / BANNER -->
<p align="center">
  <img src="assets/logo.svg" alt="chapterize" width="150">
</p>

<p align="center">
  <strong>chapterize</strong> — Apply YouTube-style chapter markers to video files
  <br>
  <a href="https://crates.io/crates/chapterize"><img src="https://img.shields.io/crates/v/chapterize" alt="crates.io"></a>
  <a href="https://github.com/mrdwarf7/chapterize.rs/actions/workflows/build.yml"><img src="https://github.com/mrdwarf7/chapterize.rs/actions/workflows/build.yml/badge.svg" alt="build"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="license"></a>
  <a href="https://github.com/mrdwarf7/chapterize.rs/releases"><img src="https://img.shields.io/github/v/release/mrdwarf7/chapterize.rs" alt="release"></a>
</p>

<!-- TAGLINE + DESCRIPTION -->
## chapterize

A CLI tool that reads chapter timestamps and titles, then applies them as chapter markers to video files using ffmpeg's metadata system. Stream copy only — no re-encoding.

```bash
# chapters.txt next to the video; auto-scans the folder for mp4/mkv/avi/mov/webm/m4v
chapterize path/to/chapters.txt
```

## Features

- **Read chapter files** — Parse `H:MM:SS`, `HH:MM:SS`, or `MM:SS` + `-` + title lines
- **Auto-detect video** — Scans the chapters file's directory for a matching video
- **ffprobe integration** — Probes video duration for the final chapter end time
- **Stream copy** — No re-encoding, preserves original quality
- **Dry-run mode** — Preview metadata and command without executing ffmpeg
- **Cross-platform** — Linux (x86_64, arm64), macOS (Intel, Apple Silicon), Windows (x86_64, arm64)

## Installation

### Cargo (Recommended)

```bash
cargo install chapterize
```

### One-liner (Linux/macOS)

```bash
curl -fsSL https://github.com/mrdwarf7/chapterize.rs/raw/main/build/install.sh | sh
```

Installs to `/usr/local/bin` (or `~/.local/bin` if not writable). Set `CHAPTERIZE_VERSION=vX.Y.Z` to pin a version.

### Release Archives

Download from [Releases](https://github.com/mrdwarf7/chapterize.rs/releases/latest).

Each archive contains the binary plus a **bundled ffmpeg/ffprobe** matched to the target platform:

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

FFmpeg is a separate program under its own (L)GPL license. You can delete the bundled binaries and use a system install on `PATH` instead.

### Supported Targets

| OS      | Arch          | Triple                      |
|---------|---------------|-----------------------------|
| Linux   | x86_64        | `x86_64-unknown-linux-gnu`  |
| Linux   | arm64         | `aarch64-unknown-linux-gnu` |
| macOS   | Intel         | `x86_64-apple-darwin`       |
| macOS   | Apple Silicon | `aarch64-apple-darwin`      |
| Windows | x86_64        | `x86_64-pc-windows-msvc`    |
| Windows | arm64         | `aarch64-pc-windows-msvc`   |

## Usage

```bash
chapterize [OPTIONS] <CHAPTERS_FILE>
```

### Examples

```bash
# chapters.txt next to the video; auto-scans the folder
chapterize path/to/chapters.txt

# Explicit video + output
chapterize chapters.txt -v movie.mp4 -o movie.with-chapters.mp4 -y

# Dry run (show what would happen)
chapterize chapters.txt --dry-run
```

### Flags

| Flag           | Short | Description                                                        |
|----------------|-------|--------------------------------------------------------------------|
| `(positional)` |       | Path to chapters file                                              |
| `--video`      | `-v`  | Video path. Default: scan chapters file's directory                |
| `--output`     | `-o`  | Output path. Default: `<stem>.chapterized.<ext>`                   |
| `--yes`        | `-y`  | Overwrite output if it exists                                      |
| `--dry-run`    |       | Build metadata, show the command, do not run ffmpeg                |
| `--quiet`      | `-q`  | Suppress non-error output                                          |
| `--verbose`    | `-v`  | Verbose output                                                     |

Logging: `RUST_LOG=debug chapterize ...` (default: `info`)

## How It Works

1. **Parse chapters** — Read chapter lines (`H:MM:SS`, `HH:MM:SS`, or `MM:SS` + `-` + title)
2. **Resolve video** — Via `--video` flag or auto-detect in the same folder
3. **Probe duration** — `ffprobe` gets the video duration for the final chapter end
4. **Write metadata** — Create a temp `;FFMETADATA1` file with chapter markers
5. **Apply chapters** — `ffmpeg -i video -i meta -map_metadata 0 -map_chapters 1 -codec copy out`

Stream copy only, no re-encode.

## Dependencies

| Tool        | Notes                                                        |
|-------------|--------------------------------------------------------------|
| ffmpeg/ffprobe | Required at runtime. Bundled in release archives. See below. |

### Bundled FFmpeg

Release archives ship with `ffmpeg` and `ffprobe` binaries from:

- **Linux/Windows**: [BtbN/FFmpeg-Builds](https://github.com/BtbN/FFmpeg-Builds) (LGPL static)
- **macOS**: [eugeneware/ffmpeg-static](https://github.com/eugeneware/ffmpeg-static)

Build locally with:

```bash
./build/fetch-ffmpeg.sh x86_64-unknown-linux-gnu /tmp/chapterize-dist
```

## Build

```bash
# Release binary
make build          # or: cargo build --release

# Run tests
make test           # or: cargo test

# Run locally
make run            # or: cargo run -- <args>

# Fetch bundled dependencies (ffmpeg)
./build/fetch-ffmpeg.sh <target> <out-dir>
```

## Links

- [Changelog](CHANGELOG.md)
- [Issues](https://github.com/mrdwarf7/chapterize.rs/issues)

## License

MIT — see [LICENSE](LICENSE).
