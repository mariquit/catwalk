#!/bin/sh
# SPDX-License-Identifier: Apache-2.0
# Copyright (c) 2026 John Anthony Mariquit <john@mariquit.com>

# Download the latest Catwalk release for this platform and run `catwalk setup`.
set -eu

repo="${CATWALK_REPO:-mariquit/catwalk}"
os=$(uname -s)
arch=$(uname -m)

case "$os-$arch" in
  Linux-x86_64) asset=catwalk-x86_64-unknown-linux-gnu ;;
  Linux-aarch64 | Linux-arm64) asset=catwalk-aarch64-unknown-linux-gnu ;;
  Darwin-x86_64) asset=catwalk-x86_64-apple-darwin ;;
  Darwin-arm64) asset=catwalk-aarch64-apple-darwin ;;
  MINGW* | MSYS* | CYGWIN*) asset=catwalk-x86_64-pc-windows-msvc.exe ;;
  *)
    echo "catwalk: unsupported platform $os $arch" >&2
    echo "download a release asset from https://github.com/${repo}/releases/latest and run it with setup" >&2
    exit 1
    ;;
esac

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT
url="https://github.com/${repo}/releases/latest/download/${asset}"
echo "downloading ${url}"
curl -fsSL -o "$tmpdir/catwalk" "$url"
curl -fsSL -o "$tmpdir/catwalk.sha256" "${url}.sha256"
if command -v sha256sum >/dev/null 2>&1; then
  actual=$(sha256sum "$tmpdir/catwalk" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
  actual=$(shasum -a 256 "$tmpdir/catwalk" | awk '{print $1}')
else
  echo "catwalk: need sha256sum or shasum to verify ${asset}" >&2
  exit 1
fi
expected=$(awk '{print $1}' "$tmpdir/catwalk.sha256")
if [ -z "$expected" ] || [ "$expected" != "$actual" ]; then
  echo "catwalk: checksum mismatch for ${asset}" >&2
  echo "expected ${expected:-<empty>}" >&2
  echo "actual   ${actual}" >&2
  exit 1
fi
echo "checksum ok"
chmod +x "$tmpdir/catwalk"
"$tmpdir/catwalk" setup
