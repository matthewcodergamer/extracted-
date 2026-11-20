# Development

## Requirements
- macOS 12 or newer (recommended for native development)
- Node.js 20+
- Rust stable
- Tauri 2 platform prerequisites: https://v2.tauri.app/start/prerequisites/

## Run locally
    npm install
    npm run tauri dev

## Build macOS app and DMG
    npm install
    npm run icons
    npm run tauri build

Artifacts are written under src-tauri/target/release/bundle/.

## Product focus: BIN and archive extraction

The main user workflow is selecting a file whose name may end in `.bin`, inspecting its actual signature, choosing an output directory, and extracting it if the payload matches a supported format. Never infer a parser from `.bin` alone. Unknown signatures are intentionally rejected instead of being unpacked as arbitrary bytes. The extraction engine should remain streaming/file-backed for large inputs; avoid reading an entire archive into a `Vec<u8>`.

## Current format support

| Format signature | Inspect | Extract |
|---|---:|---:|
| ZIP | Yes | Yes |
| TAR | Yes | Yes |
| GZIP | Yes | Yes, as one decompressed file |
| BZIP2 | Yes | Yes, as one decompressed file |
| XZ | Yes | Yes, as one decompressed file |
| 7-Zip | Yes | Yes (unencrypted archives) |
| RPF8 (RDR2) | Yes | Not yet |
| RAR | Yes | Not yet |
| ISO 9660 | Yes | Not yet |
| Unknown or proprietary BIN | Unknown classification | No |

The .bin extension does not identify a payload format. The app uses file signatures. Compressed TAR variants are not yet unpacked as nested archives.

## Safety and known limitations
- ZIP paths are checked using the archive library's enclosed-path validation.
- TAR extraction only writes regular files and directories; links and special entries are skipped.
- Existing files are not overwritten.
- ZIP, TAR, and compressor extraction stream data rather than loading whole archives into memory.
- 7-Zip uses a decoder memory limit of 256 MiB for header and dictionary/model allocations separately; this is not a hard cap on total process memory or extracted output.
- This first version does not yet show byte-level progress or support mid-extraction cancellation.
- Disk-space preflight and a full extraction rollback are not yet implemented.

## Red Dead Redemption 2 (secondary module)
RPF8 signatures are recognized for inspection, but RPF extraction is not implemented. RPF is a distinct archive format and is not equivalent to .bin. Keep RDR2 support optional and isolated from the core BIN/archive extraction path. The public rpf-archive Rust library advertises RPF8 read support, but its parse/extract API accepts the full archive as a byte slice, which is not suitable for our large-file goal without a file-backed streaming adapter. Its current parser marks encrypted RPF8 TOCs as TFIT-encrypted and reports that RDR2 keys are not supported. Extracted therefore only identifies RPF8 and does not try to decrypt or bypass it. No DRM, encryption, or access-control bypass is included.

## Test expectations
- Include small synthetic fixtures for each supported signature and extraction path.
- Include at least one ZIP and one 7-Zip fixture whose filename ends in `.bin`.
- Test large archives using generated data in CI; do not commit copyrighted game archives or huge test binaries to the repository.
- Verify extracted paths remain within the chosen destination and existing files are never overwritten.
- Treat an unknown `.bin` as unsupported, not as a failed generic extraction.
