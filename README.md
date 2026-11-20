# Extracted

A native macOS-first desktop utility for inspecting and extracting supported archive formats, including archive data stored with a `.bin` extension.

> `.bin` is a filename extension, not a format. Extracted identifies formats from file signatures and only extracts formats it can safely parse.

## Status

Initial project bootstrap in progress. The first implementation targets ZIP, 7z, TAR, GZIP, BZIP2, and XZ. RAR and ISO extraction are not enabled. RDR2 RPF8 is recognized for inspection but is not yet extracted.

## Development

The desktop shell uses Tauri 2, React, TypeScript, and a Rust extraction core. See `docs/DEVELOPMENT.md` for setup and build instructions.
