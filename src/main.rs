// Marca a função main com asyncrona e usa o runtime async-std
#[async_std::main]
async fn main() -> tide::Result<()> {
    // Define o endereço a porta onde a API vai rodar
    let addr = "127.0.0.1:8080";

    println!("Servidor Tide Rodando em: http://{}", addr);

    // Cria uma nova aplicação Tide
    let mut app = tide::new();

    // Define uma Rota Get para o caminho raiz ("/")
    // Quando a requisição Get chega em "/", ela responde com "Hello World!"
    app.at("/").get(|_| async {
        Ok("Hello, world!")
    });

    // Inicia o servidor Tide e o faz escutar as requisições no endereço definido
    app.listen(addr).await?;

    // Retorna vazio (sucesso)
    Ok(())
}