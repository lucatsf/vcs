use std::collections::HashSet;
use std::fs;
use std::path::Path;
use crate::vcs::repository::get_repo_path;
use crate::vcs::index::read_index;
use crate::vcs::object::read_commit_tree;

pub fn status() {
    let repo_path = get_repo_path();
    if !repo_path.exists() {
        eprintln!("Repositório não inicializado.");
        return;
    }

    let index = read_index(&repo_path);
    let index_files: HashSet<_> = index.iter().collect();

    let head_path = repo_path.join("HEAD");
    if !head_path.exists() {
        eprintln!("Nenhum commit encontrado.");
        return;
    }

    let head_content = fs::read_to_string(&head_path).expect("Não foi possível ler o HEAD");
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
        let current_hash = calculate_file_hash(path).expect("Não foi possível calcular o hash do arquivo");
        if !committed_files_set.contains(&(file.clone(), current_hash.clone())) {
            modified_files.push(file.clone());
        }
    }

    for file in committed_files {
        if !index_files.contains(&file.0) {
            deleted_files.push(file.0);
        }
    }

    for entry in fs::read_dir(".").expect("Não foi possível ler o diretório de trabalho") {
        let entry = entry.expect("Erro ao ler a entrada do diretório");
        let path = entry.path();
        if path.is_file() {
            let file_str = path.to_str().unwrap().to_string();
            if !index_files.contains(&file_str) {
                new_files.push(file_str);
            }
        }
    }

    if !new_files.is_empty() {
        println!("Novos arquivos:");
        for file in new_files {
            println!("  {}", file);
        }
    }

    if !modified_files.is_empty() {
        println!("Arquivos modificados:");
        for file in modified_files {
            println!("  {}", file);
        }
    }

    if !deleted_files.is_empty() {
        println!("Arquivos deletados:");
        for file in deleted_files {
            println!("  {}", file);
        }
    }

    if new_files.is_empty() && modified_files.is_empty() && deleted_files.is_empty() {
        println!("Nada a commitar, diretório de trabalho limpo.");
    }
}

fn calculate_file_hash(path: &Path) -> Option<String> {
    use sha1::{Sha1, Digest};
    use std::fs::File;
    use std::io::{self, Read};

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
