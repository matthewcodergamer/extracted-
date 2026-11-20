# Extracted

A native macOS-first desktop utility for inspecting and extracting supported archive formats, including archive data stored with a `.bin` extension.

> `.bin` is a filename extension, not a format. Extracted identifies formats from file signatures and only extracts formats it can safely parse.

## Status

Initial project bootstrap in progress. The first implementation targets ZIP, TAR, GZIP, BZIP2, and XZ. 7z, RAR, ISO, and RDR2 RPF support are tracked as separate compatibility work and are not claimed until implemented and tested.

## Development

The desktop shell uses Tauri 2, React, TypeScript, and a Rust extraction core. See `docs/DEVELOPMENT.md` for setup and build instructions.
