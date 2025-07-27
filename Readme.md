# CRUD em Rust com Tide

Este projeto é uma aplicação CRUD (Create, Read, Update, Delete) simples desenvolvida em Rust utilizando o framework web Tide.

## Funcionalidades

O sistema permite gerenciar registros de dados através de uma API RESTful. Cada registro contém duas listas: uma de strings e outra de números inteiros.

- **Criar:** Adiciona um novo registro.
- **Ler:** Consulta um registro específico ou todos os registros.
- **Atualizar:** Modifica um registro existente.
- **Deletar:** Remove um registro.

## Instalação

1.  **Instale o Rust:**
    Se você ainda não tem o Rust instalado, siga as instruções em [rustup.rs](https://rustup.rs/).

2.  **Clone o repositório:**
    ```bash
    git clone <URL_DO_REPOSITORIO>
    cd <NOME_DO_DIRETORIO>
    ```

3.  **Compile o projeto:**
    O `cargo` irá baixar e compilar todas as dependências necessárias.
    ```bash
    cargo build --release
    ```

## Execução

Para iniciar o servidor, execute o seguinte comando:

```bash
cargo run --release
```

O servidor estará disponível em `http://127.0.0.1:8080`.

## Testes

Para garantir a qualidade e o funcionamento correto da aplicação, uma suíte de testes de integração foi desenvolvida. Para executar os testes, utilize o comando:

```bash
cargo test
```

## Exemplos de Uso (com cURL)

Aqui estão alguns exemplos de como interagir com a API usando a ferramenta de linha de comando `curl`.

### 1. Criar um novo registro

Para criar um novo registro, envie uma requisição `POST` para a rota `/data` com o payload em formato JSON.

```bash
curl -X POST http://127.0.0.1:8080/data \
-H "Content-Type: application/json" \
-d '{
  "data1": ["exemplo", "de", "texto"],
  "data2": [10, 20, 30]
}'
```

O servidor responderá com o `id` do novo registro criado:

```json
{
  "id": 1
}
```

### 2. Ler um registro específico

Para ler o registro com `id=1`, envie uma requisição `GET` para `/data/1`.

```bash
curl http://127.0.0.1:8080/data/1
```

A resposta será o registro em formato JSON:

```json
{
  "data1": ["exemplo", "de", "texto"],
  "data2": [10, 20, 30]
}
```

### 3. Ler todos os registros

Para obter uma lista de todos os registros, envie uma requisição `GET` para `/data`.

```bash
curl http://127.0.0.1:8080/data
```

A resposta será um array de objetos JSON:

```json
[
  {
    "data1": ["exemplo", "de", "texto"],
    "data2": [10, 20, 30]
  }
]
```

### 4. Atualizar um registro

Para atualizar o registro com `id=1`, envie uma requisição `PUT` para `/data/1` com o novo conteúdo.

```bash
curl -X PUT http://127.0.0.1:8080/data/1 \
-H "Content-Type: application/json" \
-d '{
  "data1": ["texto", "atualizado"],
  "data2": [100, 200]
}'
```

Se a atualização for bem-sucedida, o servidor responderá com o status `200 OK`.

### 5. Deletar um registro

Para deletar o registro com `id=1`, envie uma requisição `DELETE` para `/data/1`.

```bash
curl -X DELETE http://127.0.0.1:8080/data/1
```

Se a deleção for bem-sucedida, o servidor responderá com o status `200 OK`.
