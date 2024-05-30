use std::fs;
use crate::vcs::repository::get_repo_path;

pub fn list_branches() {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Repositório não inicializado.");
        return;
    }

    let branches_dir = repo_path.join("refs").join("heads");
    if !branches_dir.exists() {
        println!("Nenhuma branch encontrada.");
        return;
    }

    println!("Branches existentes:");
    if let Ok(entries) = fs::read_dir(branches_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("  {}", file_name);
                }
            }
        }
    } else {
        eprintln!("Erro ao listar as branches.");
    }
}
