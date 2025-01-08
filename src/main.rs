use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Define a rota principal
    let app = Router::new().route("/", get(hello_world));

    // Configura o endereço e porta do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    // Inicializa o servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Define a função para a rota
async fn hello_world() -> &'static str {
    "Hello, World!"
}