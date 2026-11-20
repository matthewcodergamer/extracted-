# Extracted

A native macOS-first desktop utility for inspecting and extracting supported archive formats, including archive data stored with a `.bin` extension.

> `.bin` is a filename extension, not a format. Extracted identifies formats from file signatures and only extracts formats it can safely parse.

## Status

The first working implementation is in a feature branch and is undergoing CI validation. It includes a Tauri desktop interface, native file/destination pickers, drag-and-drop, signature inspection, and streaming extraction for ZIP, 7z, TAR, GZIP, BZIP2, and XZ. RAR and ISO extraction are not enabled. RDR2 RPF8 is recognized for inspection but is not yet extracted. Progress reporting, cancellation, disk-space preflight, and large-file integration tests remain follow-up work.

## Development

The desktop shell uses Tauri 2, React, TypeScript, and a Rust extraction core. See `docs/DEVELOPMENT.md` for setup and build instructions.
