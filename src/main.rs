mod routes;
mod types;
mod utils;

use axum::{routing::post, Router};
use routes::keypair::generate_keypair;
use routes::message::{sign_message, verify_message};
use routes::token::{create_token, mint_token, send_sol, send_token};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!("✅ Server running at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
