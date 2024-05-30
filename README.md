# vcs
![vcs](https://github.com/lucatsf/vcs/assets/18267941/5679172a-e196-450c-91eb-c7ce84fcc45e)


Um sistema de controle de versão simplificado, escrito em Rust, que suporta operações básicas como `init`, `add`, `commit`, `log`, `status`, `branch`, `checkout`, `current_branch` e `list_branches`.

## Funcionalidades

- **init**: Inicializa um novo repositório.
- **add**: Adiciona arquivos ao índice.
- **commit**: Cria um commit com as mudanças do índice.
- **log**: Exibe o histórico de commits.
- **status**: Mostra o estado atual dos arquivos em relação ao índice e commits.
- **branch**: Cria uma nova branch.
- **checkout**: Altera para uma branch específica.
- **current_branch**: Exibe a branch atual.
- **list_branches**: Lista todas as branches.

## Instalação

1. Certifique-se de ter o [Rust](https://www.rust-lang.org/) instalado.
2. Clone este repositório:
    ```sh
    git clone https://github.com/seu-usuario/simplified_vcs.git
    cd simplified_vcs
    ```
3. Compile o projeto:
    ```sh
    cargo build --release
    ```

## Uso

### Inicializar um Repositório

Para inicializar um novo repositório, use o comando `init`:

```sh
cargo run -- init
```

### Criar um Commit

Para criar um commit com as mudanças no índice, use o comando commit:
```sh
cargo run -- add <arquivo>
```
### Exibir o Histórico de Commits

Para exibir o histórico de commits, use o comando log:

```sh
cargo run -- log
```

### Mostrar o Status dos Arquivos

Para mostrar o estado atual dos arquivos em relação ao índice e commits, use o comando status:

```sh
cargo run -- status
```

### Criar uma Nova Branch

Para criar uma nova branch, use o comando branch:

```sh
cargo run -- branch <nome-da-branch>
```

### Mudar para uma Branch Específica

Para mudar para uma branch específica, use o comando checkout:

```sh
cargo run -- checkout <nome-da-branch>
```

### Exibir a Branch Atual

Para exibir a branch atual, use o comando current_branch:

```sh
cargo run -- current_branch
```

### Listar Todas as Branches

Para listar todas as branches, use o comando list_branches:

```sh
cargo run -- list_branches
```

### Contribuição

1. Faça um fork do projeto.
2. Crie uma nova branch para sua feature (git checkout -b feature/nome-da-feature).
3. Commit suas alterações (git commit -am 'Adicionei uma nova feature').
4. Faça push para a branch (git push origin feature/nome-da-feature).
5. Crie um novo Pull Request.

### Licença

Este projeto está licenciado sob a Licença MIT. Veja o arquivo LICENSE para mais detalhes.

### Agradecimentos

Este README fornece uma visão geral clara e concisa de como usar seu Sistema de Controle de Versão Simplificado, incluindo exemplos de comandos e instruções para instalação e contribuição. Você pode personalizá-lo ainda mais conforme necessário. Boa sorte com seu projeto!


