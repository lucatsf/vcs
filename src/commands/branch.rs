use std::fs;
use crate::vcs::repository::get_repo_path;

pub fn branch(branch_name: &str) {
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

    let branches_dir = repo_path.join("refs").join("heads");

    // Garantindo que o diretório de branches exista
    if !branches_dir.exists() {
        fs::create_dir_all(&branches_dir).expect("Não foi possível criar o diretório de branches");
    }

    let branch_path = branches_dir.join(branch_name);
    if branch_path.exists() {
        eprintln!("Branch '{}' já existe.", branch_name);
        return;
    }

    let commit_hash = fs::read_to_string(&head_path).expect("Não foi possível ler o HEAD").trim().to_string();

    // Atualizando o arquivo HEAD para apontar para a nova branch
    fs::write(&head_path, format!("ref: refs/heads/{}", branch_name)).expect("Não foi possível atualizar o HEAD");

    // Criando o arquivo da nova branch com o hash do último commit
    fs::write(&branch_path, commit_hash).expect("Não foi possível criar a branch");
    println!("Branch '{}' criada com sucesso.", branch_name);
}
