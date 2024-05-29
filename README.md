# vcs
![image](https://github.com/lucatsf/vcs/assets/18267941/9fe884f9-ef77-41eb-a2df-fa6e0d854d8e)

```bash
simplified_vcs/
├── src/
│   ├── commands/
│   │   ├── init.rs
│   │   ├── add.rs
│   │   ├── commit.rs
│   │   ├── log.rs
│   │   ├── status.rs
│   ├── vcs/
│   │   ├── repository.rs
│   │   ├── index.rs
│   │   └── object.rs
│   ├── main.rs
│   └── utils.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

## Commands

- **init** - Inicializa um novo repositório.
- **add** - Adiciona arquivos ao índice.
- **commit** - Comita as mudanças no índice.
- **log** - Exibe o histórico de commits.
- **status** - Mostra o estado atual dos arquivos em relação ao índice e commits.

I developed a Simplified Version Control System in Rust, featuring basic commands like init, add, commit, log, and status. This project demonstrates my skills in file manipulation, implementation of diff algorithms, and use of efficient data structures. I utilized libraries such as serde and serde_json for data management.
