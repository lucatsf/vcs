use std::path::{Path, PathBuf};

pub fn get_repo_path() -> PathBuf {
    Path::new(".vcs").to_path_buf()
}
