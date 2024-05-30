use std::fs;
use crate::vcs::repository::get_repo_path;
use crate::vcs::index::write_index;

pub fn checkout(branch_name: &str) {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Repositório não inicializado.");
        return;
    }

    let branch_path = repo_path.join("refs").join("heads").join(branch_name);
    if !branch_path.exists() {
        eprintln!("Branch '{}' não existe.", branch_name);
        return;
    }

    let commit_hash = fs::read_to_string(&branch_path).expect("Não foi possível ler a branch");
    let head_path = repo_path.join("HEAD");
    fs::write(head_path, commit_hash.clone()).expect("Não foi possível atualizar o HEAD");

    // Resetando o índice para o estado do commit
    let commit_tree = crate::vcs::object::read_commit_tree(&repo_path, commit_hash.trim());
    write_index(&repo_path, &commit_tree);

    println!("Mudado para a branch '{}'.", branch_name);
}
