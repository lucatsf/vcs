use std::fs;
use std::path::Path;

pub fn read_index(repo_path: &Path) -> Vec<String> {
    let index_path = repo_path.join("index");
    if !index_path.exists() {
        return vec![];
    }

    let index_content = fs::read_to_string(index_path).expect("Unable to read index");
    index_content.lines().map(String::from).collect()
}
