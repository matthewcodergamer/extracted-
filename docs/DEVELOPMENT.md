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
- Extraction streams data rather than loading entire archives into memory.
- This first version does not yet show byte-level progress or support mid-extraction cancellation.
- Disk-space preflight and a full extraction rollback are not yet implemented.

## Red Dead Redemption 2
RPF8 signatures are recognized for inspection, but RPF extraction is not implemented. RPF is a distinct archive format and is not equivalent to .bin. A current Rust library advertises read support for RPF8, but the integration and its key/encryption requirements still need validation against lawful sample fixtures. No DRM, encryption, or access-control bypass is included.
