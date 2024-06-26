use std::fs;
use std::path::Path;
use sha1::{Sha1, Digest};

pub fn write_object(repo_path: &Path, data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let object_dir = repo_path.join("objects").join(&hash_hex[..2]);
    fs::create_dir_all(&object_dir).expect("Não foi possível criar o diretório de objetos");

    let object_path = object_dir.join(&hash_hex[2..]);
    fs::write(&object_path, data).expect("Não foi possível escrever o objeto");

    hash_hex
}


pub fn create_tree_object(repo_path: &Path, index: &[String]) -> String {
    let mut tree_data = Vec::new();
    for file_path in index {
        let path = Path::new(file_path);
        let data = fs::read(path).expect("Não foi possível ler o arquivo do índice");
        let file_hash = write_object(repo_path, &data);
        tree_data.extend_from_slice(format!("100644 {} {}\n", file_hash, file_path).as_bytes());
    }

    write_object(repo_path, &tree_data)
}

pub fn read_commit_tree(repo_path: &Path, commit_hash: &str) -> Vec<(String, String)> {
    let commit_path = repo_path.join("objects").join(&commit_hash[..2]).join(&commit_hash[2..]);
    if !commit_path.exists() {
        panic!("Commit não encontrado: {}", commit_hash);
    }

    let commit_content = fs::read_to_string(&commit_path).expect("Não foi possível ler o commit");
    let mut lines = commit_content.lines();

    let tree_line = lines.find(|line| line.starts_with("tree ")).expect("Commit sem árvore");
    let tree_hash = &tree_line[5..];

    read_tree(repo_path, tree_hash)
}


fn read_tree(repo_path: &Path, tree_hash: &str) -> Vec<(String, String)> {
    let tree_path = repo_path.join("objects").join(&tree_hash[..2]).join(&tree_hash[2..]);
    let tree_content = fs::read_to_string(tree_path).expect("Não foi possível ler a árvore");
    
    tree_content.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hash = parts[0].to_string();
        let path = parts[1].to_string();
        (path, hash)
    }).collect()
}

pub fn read_commit(repo_path: &Path, commit_hash: &str) -> (String, Option<String>) {
    let commit_path = repo_path.join("objects").join(&commit_hash[..2]).join(&commit_hash[2..]);
    if !commit_path.exists() {
        panic!("Commit não encontrado: {}", commit_hash);
    }

    let commit_content = fs::read_to_string(&commit_path).expect("Não foi possível ler o commit");
    
    // Se o conteúdo do commit estiver vazio, retorna uma mensagem indicando que não há nada para ler
    if commit_content.trim().is_empty() {
        return ("Commit vazio".to_string(), None);
    }

    let mut lines = commit_content.lines();
    let mut message = String::new();
    let mut parent = None;

    for line in lines.by_ref() {
        if line.starts_with("parent ") {
            parent = Some(line[7..].to_string());
        } else if line.trim().is_empty() {
            break;
        }
    }

    for line in lines {
        message.push_str(line);
        message.push('\n');
    }

    (message.trim().to_string(), parent)
}
