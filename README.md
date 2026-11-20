# Extracted

**Extracted is a desktop archive utility whose primary job is opening and unpacking archive data stored in `.bin` files.** It also opens ordinary archive files and is designed to grow into a game-asset extraction tool.

> **Important:** `.bin` is a filename extension, not a single archive format. Extracted inspects the file signature (the bytes inside the file), identifies known formats, and extracts only formats it can safely parse. A proprietary or unknown `.bin` cannot be unpacked by guessing.

## What works now

- Native macOS desktop app built with Tauri 2, React, TypeScript, and Rust.
- Select a file or drag it into the app; inspect its detected format and size.
- Choose an output folder and extract supported archives.
- Detect archive contents by signature, even when the file is named `.bin`.
- Stream archive contents to disk rather than loading the complete archive into memory.
- Preserve directory structure and refuse to overwrite existing files.
- Reject unsafe archive paths and skip TAR links and special entries.

### Supported extraction formats

| Format inside the file | Extract | Notes |
|---|---:|---|
| ZIP | Yes | Includes ZIP data named `.bin` |
| 7-Zip | Yes | Unencrypted archives; decoder allocations have a configured limit |
| TAR | Yes | Regular files and directories |
| GZIP | Yes | Produces the decompressed file |
| BZIP2 | Yes | Produces the decompressed file |
| XZ | Yes | Produces the decompressed file |
| RPF8 (Red Dead Redemption 2) | Not yet | Signature detection only |
| RAR | Not yet | Signature detection only |
| ISO 9660 | Not yet | Signature detection only |
| Unknown/proprietary BIN | No | Requires a format-specific parser |

For a compressed single-stream file such as `something.bin.gz`, the app writes the decompressed payload as one output file. A generic `something.bin` is not automatically assumed to be an archive.

## Red Dead Redemption 2 support (secondary)

RDR2's RPF8 archives are a separate format from generic `.bin` archives. Extracted currently recognizes the RPF8 signature so it can identify the file, but it does not yet browse or extract RPF8 contents. The planned RDR2 work is an optional format-specific module, kept separate from the main BIN/archive extraction path.

Encrypted RPF8 metadata may require keys that a public parser does not provide. Extracted will not claim encrypted archives are extractable, generate corrupted output, or bypass encryption/DRM. The goal is to support readable, accessible RPF8 data where technically possible.

## Roadmap

1. **Core BIN/archive reliability:** broaden real-world fixtures, test large files, improve error recovery, and add progress and cancellation.
2. **Format coverage:** add more archive formats only when their signatures, parsers, and safe extraction behavior are validated.
3. **RDR2 module:** implement RPF8 metadata inspection and extraction for readable archives, with explicit handling for encrypted TOCs.
4. **Game assets:** add optional export/conversion workflows for resource types only after their formats are documented and tested.

## Development

The desktop shell uses Tauri 2, React, TypeScript, and a Rust extraction core. See [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for setup, current support, and build instructions.

## Project status

This is an actively developed MVP, not yet a finished release. Progress reporting, mid-extraction cancellation, disk-space preflight, extraction rollback, and broad large-file integration tests are still outstanding. Check GitHub Actions for the current build status and downloadable artifacts.
