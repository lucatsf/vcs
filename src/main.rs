mod commands;
mod vcs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: vcs <command>");
        return;
    }

    match args[1].as_str() {
        "init" => commands::init::init(),
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: vcs add <file>");
                return;
            }
            commands::add::add(&args[2]);
        },
        "commit" => {
            if args.len() < 3 {
                eprintln!("Usage: vcs commit <message>");
                return;
            }
            commands::commit::commit(&args[2]);
        },
        "status" => commands::status::status(),
        "log" => commands::log::log(),
        "branch" => {
            if args.len() < 3 {
                eprintln!("Usage: vcs branch <branch_name>");
                return;
            }
            commands::branch::branch(&args[2]);
        },
        "checkout" => {
            if args.len() < 3 {
                eprintln!("Usage: vcs checkout <branch_name>");
                return;
            }
            commands::checkout::checkout(&args[2]);
        },
        _ => eprintln!("Comando não reconhecido."),
    }
}
