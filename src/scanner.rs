use std::path::Path;
use jwalk::{WalkDir, WalkDirGeneric};

/// Discovers files in subdirectories and creates batches.
pub struct Scanner {
    // Absolute path where scanning started.
    //root: Path,
    /// Paths to discovered files relative to origin.
    pub files: WalkDirGeneric<((),())>,
}

impl Scanner {
    pub fn scan(root: &str) -> Self {
        let files = WalkDir::new(&root)
            .skip_hidden(false)
            .process_read_dir(|depth, path, state, children| {
                if path.ends_with(".repo")
                    || path.ends_with(".git")
                    || path.ends_with("prebuilt")
                    || path.ends_with("prebuilts")
                    || path.ends_with("out") {
                    children.clear();
                }
                //println!("{}", &path.as_os_str().to_string_lossy());
            });
        Scanner {
            //root: files.root().canonicalize().expect("Invalid root directory").copy(),
            files,
        }
    }
}