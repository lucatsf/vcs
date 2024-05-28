use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::vcs::repository::get_repo_path;
use crate::vcs::index::read_index;
use crate::vcs::object::{write_object, create_tree_object};

pub fn commit(message: &str) {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Uninitialized repository.");
        return;
    }

    let index = read_index(&repo_path);

    if index.is_empty() {
        eprintln!("Nothing to commit, index is empty.");
        return;
    }

    let tree_hash = create_tree_object(&repo_path, &index);

    let mut commit_data = String::new();
    commit_data.push_str(&format!("tree {}\n", tree_hash));

    // Adicionar pai (último commit) se existir
    let head_path = repo_path.join("HEAD");
    if head_path.exists() {
        let head_content = fs::read_to_string(&head_path).expect("Unable to read HEAD");
        let parent_path = repo_path.join(head_content.trim());
        if parent_path.exists() {
            let parent_hash = fs::read_to_string(parent_path).expect("Unable to read parent commit");
            commit_data.push_str(&format!("parent {}\n", parent_hash));
        }
    }

    let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting current time");
    commit_data.push_str(&format!("author You <you@example.com> {}\n", time_now.as_secs()));
    commit_data.push_str(&format!("committer You <you@example.com> {}\n", time_now.as_secs()));
    commit_data.push_str(&format!("\n{}\n", message));

    let commit_hash = write_object(&repo_path, commit_data.as_bytes());

    // Atualizar HEAD
    fs::write(repo_path.join("HEAD"), format!("{}\n", commit_hash)).expect("Unable to update HEAD");

    println!("Commit criado: {}", commit_hash);
}
