# Criar o projeto com Library
cargo new nome_do_projeto --lib

# Ativação do Ambiente Virtual: No Linux
python3 -m venv .venv
source .venv/bin/activate
deactivate

# Intalação Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificação da instalação
rustc --version
cargo --version

# Compilar com rustc
rustc main.rs

# Executar binário
./main

# Tabela de comandos de execução

| Comando         | Compila? | Cria Executável? | Executa? | Caso de Uso Principal                               |
|-----------------|----------|------------------|----------|-----------------------------------------------------|
| `cargo check`   | Sim      | Não              | Não      | Verificar erros rapidamente enquanto codifica.      |
| `cargo build`   | Sim      | Sim              | Não      | Preparar o programa para uso ou distribuição.       |
| `cargo run`     | Sim      | Sim              | Sim      | Compilar e testar o resultado do programa na hora.  |

# Criar conta no Crates.io
- Acesse https://crates.io e crie uma conta
- Gere um token de API em Account Settings > API Tokens
- Autentique se localmente (cargo login <seu-token>)

# Publicar no Crates.io
- verificar antes de publicar
cargo package
- gPublicar no Crates.io
cargo publish

Meu repositório no Crates.io:
https://crates.io/crates/CRUD-henriquecapi