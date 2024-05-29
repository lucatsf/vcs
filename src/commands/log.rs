use std::fs;
use crate::vcs::repository::get_repo_path;
use crate::vcs::object::read_commit;

pub fn log() {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Repositório não inicializado.");
        return;
    }

    let head_path = repo_path.join("HEAD");
    if !head_path.exists() {
        eprintln!("Nenhum commit encontrado.");
        return;
    }

    let mut commit_hash = fs::read_to_string(head_path).expect("Não foi possível ler o HEAD").trim().to_string();
    while !commit_hash.is_empty() {
        let (commit_message, parent_hash) = read_commit(&repo_path, &commit_hash);
        println!("commit {}\n\n{}\n", commit_hash, commit_message);

        commit_hash = parent_hash.unwrap_or_default();
    }
}
