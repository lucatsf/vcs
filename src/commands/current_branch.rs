use std::fs;
use crate::vcs::repository::get_repo_path;

pub fn current_branch() {
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

    let head_content = fs::read_to_string(&head_path).expect("Não foi possível ler o HEAD");
    let head_content_trimmed = head_content.trim();

    if head_content_trimmed.starts_with("ref: refs/heads/") {
        let branch_name = &head_content_trimmed[16..]; // Correção do índice
        println!("Você está na branch: {}", branch_name);
    } else {
        println!("HEAD aponta diretamente para um commit.");
    }
}
