use crud_henriquecapi::setup_app;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let app = setup_app();
    let addr = "127.0.0.1:8080";
    println!("Servidor CRUD rodando em: http://{addr}");
    app.listen(addr).await?;
    Ok(())
}
