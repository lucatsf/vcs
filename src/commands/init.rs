use std::fs;
use std::path::Path;

pub fn init() {
    let repo_path = Path::new(".vcs");
    if repo_path.exists() {
        eprintln!("Repository already exists.");
        return;
    }

    // Create the repository directory and subdirectories
    fs::create_dir(repo_path).expect("Unable to create repository directory");
    fs::create_dir(repo_path.join("objects")).expect("Unable to create object directory");
    fs::create_dir(repo_path.join("refs")).expect("Unable to create reference directory");

    // Create the HEAD file
    fs::write(repo_path.join("HEAD"), "ref: refs/heads/master\n").expect("Unable to create HEAD file");

    println!("Repository initialized successfully!");
}
