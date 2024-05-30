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

pub fn write_index(repo_path: &Path, tree: &[(String, String)]) {
    let index_path = repo_path.join("index");
    let mut index_content = String::new();
    for (path, hash) in tree {
        index_content.push_str(&format!("{} {}\n", path, hash));
    }
    fs::write(index_path, index_content).expect("Unable to write index");
}
