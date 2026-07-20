#!/usr/bin/env bash
# Fetch platform-matched ffmpeg + ffprobe into an output directory for release zips.
#
# Usage:
#   build/fetch-ffmpeg.sh <rust-target-triple> <out-dir>
#
# Sources:
#   Linux / Windows : BtbN/FFmpeg-Builds (latest versioned nX.Y LGPL static build)
#   macOS           : eugeneware/ffmpeg-static (latest release; provides ffmpeg+ffprobe)
#
# Not legal advice. Bundled FFmpeg remains under LGPL/GPL as shipped by upstream.
# We copy their license files into out-dir/third_party/ffmpeg/ when present.
set -euo pipefail

TARGET="${1:?rust target triple required}"
OUT_DIR="${2:?output directory required}"
WORKDIR="$(mktemp -d "${TMPDIR:-/tmp}/chapterize-ffmpeg.XXXXXX")"
trap 'rm -rf "$WORKDIR"' EXIT

mkdir -p "$OUT_DIR" "$OUT_DIR/third_party/ffmpeg"
log() { printf 'fetch-ffmpeg: %s\n' "$*" >&2; }

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    log "missing required command: $1"
    exit 1
  }
}

need_cmd curl
need_cmd python3

# Map rustc target -> fetch plan
# plan fields: family(btbn|eugeneware) arch_token
case "$TARGET" in
x86_64-unknown-linux-gnu)
  FAMILY=btbn
  ARCH=linux64
  EXT=tar.xz
  ;;
aarch64-unknown-linux-gnu)
  FAMILY=btbn
  ARCH=linuxarm64
  EXT=tar.xz
  ;;
x86_64-pc-windows-msvc | x86_64-pc-windows-gnu)
  FAMILY=btbn
  ARCH=win64
  EXT=zip
  ;;
aarch64-pc-windows-msvc | aarch64-pc-windows-gnu)
  FAMILY=btbn
  ARCH=winarm64
  EXT=zip
  ;;
x86_64-apple-darwin)
  FAMILY=eugeneware
  ARCH=darwin-x64
  ;;
aarch64-apple-darwin)
  FAMILY=eugeneware
  ARCH=darwin-arm64
  ;;
*)
  log "unsupported target for bundled ffmpeg: $TARGET"
  exit 1
  ;;
esac

log "target=$TARGET family=$FAMILY arch=$ARCH"

fetch_btbn() {
  need_cmd tar
  local api json asset_name url
  api="https://api.github.com/repos/BtbN/FFmpeg-Builds/releases/latest"
  json="$WORKDIR/btbn.json"
  curl -fsSL -H "Accept: application/vnd.github+json" "$api" -o "$json"

  # Prefer newest versioned nX.Y LGPL *static* build for ARCH (skip master + shared).
  # Prints: asset_name<TAB>browser_download_url
  local asset_line asset_name url
  asset_line="$(
    python3 - "$json" "$ARCH" "$EXT" <<'PY'
import json, re, sys
path, arch, ext = sys.argv[1], sys.argv[2], sys.argv[3]
data = json.load(open(path, encoding="utf-8"))
assets = data.get("assets", [])
pat = re.compile(
    rf"^ffmpeg-n(\d+)\.(\d+)-latest-{re.escape(arch)}-lgpl(?:-\d+\.\d+)?\.{re.escape(ext)}$"
)
scored = []
for a in assets:
    name = a.get("name") or ""
    if "shared" in name or "master" in name:
        continue
    m = pat.match(name)
    if not m:
        continue
    scored.append((int(m.group(1)), int(m.group(2)), name, a.get("browser_download_url") or ""))
if not scored:
    pat2 = re.compile(
        rf"^ffmpeg-master-latest-{re.escape(arch)}-lgpl\.{re.escape(ext)}$"
    )
    for a in assets:
        name = a.get("name") or ""
        if pat2.match(name):
            print("%s\t%s" % (name, a.get("browser_download_url") or ""))
            sys.exit(0)
    sys.stderr.write("no matching BtbN asset for arch=%s ext=%s\n" % (arch, ext))
    for a in sorted(assets, key=lambda x: x.get("name") or ""):
        name = a.get("name") or ""
        if arch in name:
            sys.stderr.write("  %s\n" % name)
    sys.exit(1)
scored.sort(reverse=True)
name, url = scored[0][2], scored[0][3]
print("%s\t%s" % (name, url))
PY
  )"
  asset_name="${asset_line%%$'\t'*}"
  url="${asset_line#*$'\t'}"
  if [[ -z "$url" || "$url" == "$asset_name" ]]; then
    url="https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/${asset_name}"
  fi

  log "downloading $url"
  curl -fsSL -L -o "$WORKDIR/ffmpeg-pack.$EXT" "$url"

  local extract="$WORKDIR/extract"
  mkdir -p "$extract"
  if [[ "$EXT" == "zip" ]]; then
    need_cmd unzip
    unzip -q "$WORKDIR/ffmpeg-pack.$EXT" -d "$extract"
  else
    tar -xJf "$WORKDIR/ffmpeg-pack.$EXT" -C "$extract"
  fi

  # BtbN layout: ffmpeg-*/bin/ffmpeg[.exe]
  local bin_dir
  bin_dir="$(find "$extract" -type d -name bin | head -n1)"
  if [[ -z "$bin_dir" ]]; then
    log "could not find bin/ in BtbN archive"
    find "$extract" -maxdepth 3 -type f | head -50 >&2
    exit 1
  fi

  if [[ "$ARCH" == win* ]]; then
    cp -f "$bin_dir/ffmpeg.exe" "$OUT_DIR/ffmpeg.exe"
    cp -f "$bin_dir/ffprobe.exe" "$OUT_DIR/ffprobe.exe"
  else
    cp -f "$bin_dir/ffmpeg" "$OUT_DIR/ffmpeg"
    cp -f "$bin_dir/ffprobe" "$OUT_DIR/ffprobe"
    chmod +x "$OUT_DIR/ffmpeg" "$OUT_DIR/ffprobe"
  fi

  # License / readme crumbs if present
  find "$extract" -maxdepth 2 -type f \( \
    -iname 'LICENSE*' -o -iname 'COPYING*' -o -iname 'README*' \
    \) -exec cp -f {} "$OUT_DIR/third_party/ffmpeg/" \; 2>/dev/null || true

  printf '%s\n' "$asset_name" >"$OUT_DIR/third_party/ffmpeg/SOURCE.txt"
  printf 'https://github.com/BtbN/FFmpeg-Builds (LGPL static build)\n' \
    >>"$OUT_DIR/third_party/ffmpeg/SOURCE.txt"
}

fetch_eugeneware() {
  # macOS static builds; ships ffmpeg + ffprobe per arch.
  local api json tag base
  api="https://api.github.com/repos/eugeneware/ffmpeg-static/releases/latest"
  json="$WORKDIR/eu.json"
  curl -fsSL -H "Accept: application/vnd.github+json" "$api" -o "$json"
  tag="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1],encoding="utf-8"))["tag_name"])' "$json")"
  base="https://github.com/eugeneware/ffmpeg-static/releases/download/${tag}"

  log "eugeneware tag=$tag arch=$ARCH"
  curl -fsSL -o "$WORKDIR/ffmpeg.gz" "${base}/ffmpeg-${ARCH}.gz"
  curl -fsSL -o "$WORKDIR/ffprobe.gz" "${base}/ffprobe-${ARCH}.gz"
  # optional license
  curl -fsSL -o "$OUT_DIR/third_party/ffmpeg/LICENSE.gz" \
    "${base}/${ARCH}.LICENSE.gz" 2>/dev/null || true
  curl -fsSL -o "$OUT_DIR/third_party/ffmpeg/README.gz" \
    "${base}/${ARCH}.README.gz" 2>/dev/null || true

  if command -v gzip >/dev/null 2>&1; then
    gzip -dc "$WORKDIR/ffmpeg.gz" >"$OUT_DIR/ffmpeg"
    gzip -dc "$WORKDIR/ffprobe.gz" >"$OUT_DIR/ffprobe"
    if [[ -f "$OUT_DIR/third_party/ffmpeg/LICENSE.gz" ]]; then
      gzip -dc "$OUT_DIR/third_party/ffmpeg/LICENSE.gz" \
        >"$OUT_DIR/third_party/ffmpeg/LICENSE" || true
    fi
  else
    python3 - "$WORKDIR/ffmpeg.gz" "$OUT_DIR/ffmpeg" <<'PY'
import gzip, shutil, sys
with gzip.open(sys.argv[1], "rb") as src, open(sys.argv[2], "wb") as dst:
    shutil.copyfileobj(src, dst)
PY
    python3 - "$WORKDIR/ffprobe.gz" "$OUT_DIR/ffprobe" <<'PY'
import gzip, shutil, sys
with gzip.open(sys.argv[1], "rb") as src, open(sys.argv[2], "wb") as dst:
    shutil.copyfileobj(src, dst)
PY
  fi
  chmod +x "$OUT_DIR/ffmpeg" "$OUT_DIR/ffprobe"

  printf 'eugeneware/ffmpeg-static %s (%s)\n' "$tag" "$ARCH" \
    >"$OUT_DIR/third_party/ffmpeg/SOURCE.txt"
  printf 'https://github.com/eugeneware/ffmpeg-static\n' \
    >>"$OUT_DIR/third_party/ffmpeg/SOURCE.txt"
}

case "$FAMILY" in
btbn) fetch_btbn ;;
eugeneware) fetch_eugeneware ;;
esac

# Sanity: binaries exist and are non-empty
if [[ "$ARCH" == win* ]]; then
  test -s "$OUT_DIR/ffmpeg.exe"
  test -s "$OUT_DIR/ffprobe.exe"
else
  test -s "$OUT_DIR/ffmpeg"
  test -s "$OUT_DIR/ffprobe"
fi

# Write a short notice for the zip root
cat >"$OUT_DIR/FFMPEG_NOTICE.txt" <<'EOF'
This archive may include ffmpeg and ffprobe binaries for convenience.

- chapterize itself is MIT (see LICENSE).
- ffmpeg/ffprobe are separate programs redistributed under their upstream
  licenses (typically LGPL 2.1+; GPL if the build enabled GPL components).
- See third_party/ffmpeg/ for upstream source attribution and license texts.
- You may delete ffmpeg/ffprobe and use system installs on PATH instead.
- Official project: https://ffmpeg.org/  Legal: https://ffmpeg.org/legal.html

Build sources used by our release script:
  Linux/Windows: https://github.com/BtbN/FFmpeg-Builds (LGPL static)
  macOS:         https://github.com/eugeneware/ffmpeg-static
EOF

log "ok -> $OUT_DIR"
ls -lah "$OUT_DIR" >&2 || true
