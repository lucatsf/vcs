use std::fs;
use std::path::Path;
use sha1::{Sha1, Digest};

pub fn write_object(repo_path: &Path, data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let object_path = repo_path.join("objects").join(&hash_hex[..2]).join(&hash_hex[2..]);
    fs::create_dir_all(object_path.parent().unwrap()).expect("Unable to create object directory");
    fs::write(object_path, data).expect("Unable to write object");

    hash_hex
}

pub fn create_tree_object(repo_path: &Path, index: &[String]) -> String {
    let mut tree_data = Vec::new();
    for file_path in index {
        let path = Path::new(file_path);
        let data = fs::read(path).expect("Unable to read index file");
        let file_hash = write_object(repo_path, &data);
        tree_data.extend_from_slice(format!("100644 {} {}\n", file_hash, file_path).as_bytes());
    }

    write_object(repo_path, &tree_data)
}
