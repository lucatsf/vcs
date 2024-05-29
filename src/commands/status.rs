use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::vcs::repository::get_repo_path;
use crate::vcs::index::read_index;
use crate::vcs::object::read_commit_tree;

pub fn status() {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Uninitialized repository.");
        return;
    }

    let index = read_index(&repo_path);
    let index_files: HashSet<_> = index.iter().collect();

    let head_path = repo_path.join("HEAD");
    if !head_path.exists() {
        eprintln!("No commits found.");
        return;
    }

    let head_content = fs::read_to_string(&head_path).expect("Unable to read HEAD");
    let commit_hash = head_content.trim();
    let committed_files = read_commit_tree(&repo_path, commit_hash);

    let committed_files_set: HashSet<_> = committed_files.iter().collect();

    let mut modified_files = vec![];
    let mut new_files = vec![];
    let mut deleted_files = vec![];

    for file in &index {
        let path = Path::new(file);
        if !path.exists() {
            deleted_files.push(file.clone());
            continue;
        }
        let current_hash = calculate_file_hash(path).expect("Unable to calculate file hash");
        if !committed_files_set.contains(&(file.clone(), current_hash.clone())) {
            modified_files.push(file.clone());
        }
    }

    for file in committed_files {
        if !index_files.contains(&file.0) {
            deleted_files.push(file.0);
        }
    }

    for entry in fs::read_dir(".").expect("Unable to read working directory") {
        let entry = entry.expect("Error reading directory entry");
        let path = entry.path();
        if path.is_file() {
            let file_str = path.to_str().unwrap().to_string();
            if !index_files.contains(&file_str) {
                new_files.push(file_str);
            }
        }
    }

    if !new_files.is_empty() {
        println!("New files:");
        for file in &new_files {
            println!("  {}", file);
        }
    }

    if !modified_files.is_empty() {
        println!("Modified files:");
        for file in &modified_files {
            println!("  {}", file);
        }
    }

    if !deleted_files.is_empty() {
        println!("Deleted files:");
        for file in &deleted_files {
            println!("  {}", file);
        }
    }

    if new_files.is_empty() && modified_files.is_empty() && deleted_files.is_empty() {
        println!("Nothing to commit, clean working directory.");
    }
}

fn calculate_file_hash(path: &Path) -> Option<String> {
    use sha1::{Sha1, Digest};
    use std::fs::File;
    use std::io::{Read};

    let mut file = File::open(path).ok()?;
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let hash = hasher.finalize();
    Some(format!("{:x}", hash))
}
