use std::fs;
use std::path::Path;

pub fn add(file_path: &str) {
    let path = Path::new(file_path);
    if !path.exists() {
        eprintln!("File not found: {}", file_path);
        return;
    }

    let repo_path = Path::new(".vcs");
    if !repo_path.exists() {
        eprintln!("Uninitialized repository.");
        return;
    }

    let index_path = repo_path.join("index");
    let mut index = fs::read_to_string(&index_path).unwrap_or_else(|_| String::new());
    index.push_str(&format!("{}\n", file_path));
    fs::write(index_path, index).expect("Unable to update index");

    println!("File added to index: {}", file_path);
}
