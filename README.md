# Hello World Aplicação Web em Rust

Este é um aplicativo web simples "Hello, World!" criado usando o framework Axum em Rust. O aplicativo demonstra como configurar um servidor web 
básico e lidar com solicitações usando programação assíncrona Rust moderna com Tokio.

## Características

Servidor web leve e rápido
Lida com solicitações HTTP GET
Construído com o poderoso framework Axum
Usa Tokio para execução assíncrona

## Pré-requisitos
Certifique-se de ter o seguinte instalado Rust.

Etapa 1: instalar dependências
Verifique as dependências Cargo.tomle certifique-se de que estejam corretas:

[dependencies]
axum = "0.6"
tokio = { version = "1.33", features = ["full"] }
hyper = "0.14"
tower = "0.4"

Uso
Abra seu navegador da web.

Navegue até : http://127.0.0.1:3000

Você deverá ver a mensagem:

Hello, World!
