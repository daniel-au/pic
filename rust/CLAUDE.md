# pic (Rust)

CLI tool for batch photo/video file operations — rename or copy files. Targets common media extensions: JPG, CR2, GIF, JPEG, MOV, MP4, NEF, PNG.

## Commands

```bash
cargo build                 # debug build
cargo run -- rename         # rename images with a prefix + sequential numbering
cargo run -- copy           # copy selected images from a "Good Ones.txt" index file
```

## Code conventions
- Internal helpers are prefixed with `_` (e.g., `_get_images`, `_read_input`)
- Errors use `panic!` / `unwrap` — this is intentional for a personal CLI tool
- Rename uses a two-pass approach (random temp prefix → final prefix) to avoid collisions
- Filenames must match `(?<prefix>.+)_(?<index>\d+)\.(?<extension>\w+)$` — panics otherwise
