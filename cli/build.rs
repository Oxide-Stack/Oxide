// Rebuild the `oxide` binary whenever starter templates change.
//
// `include_dir!` embeds `templates/` at compile time, and cargo does not watch
// files pulled in by a proc macro. Without this, editing a template and
// rebuilding produces a binary that still embeds the old content.
//
// The path has to be each *file*, not the directory. Cargo watches a directory
// entry's mtime, and on Windows (and on macOS) editing a file's contents does
// not update its parent directory's mtime, so a directory-level watch silently
// misses content edits. Emitting one line per file is correct on every
// platform and costs nothing beyond the template count.

use std::path::Path;

fn main() {
    let templates = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates");
    println!("cargo:rerun-if-changed={}", templates.display());

    // A missing or removed template tree should not fail the build; the
    // `include_dir!` expansion in create.rs will report it more usefully.
    let Ok(entries) = std::fs::read_dir(&templates) else {
        return;
    };
    let mut files: Vec<_> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
    }
}
