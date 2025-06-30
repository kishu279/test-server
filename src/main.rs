use axum::{Json, Router, extract::Query, response::Html, routing::get};
use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct BalanceResponse {
    wallet: String,
    balance_sol: f64,
}

async fn root() -> Html<&'static str> {
    Html("<h2>✅ Server is running! Use /balance?walletAdd=...</h2>")
}

async fn balance_Request(Query(params): Query<HashMap<String, String>>) -> Json<BalanceResponse> {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());
    if let Some(wallet) = params.get("walletAdd") {
        let pubkey = Pubkey::from_str(wallet).unwrap();
        let balance = client.get_balance(&pubkey).unwrap();

        Json(BalanceResponse {
            wallet: wallet.to_string(),
            balance_sol: balance as f64 / 1_000_000_000.0,
        })
    } else {
        Json(BalanceResponse {
            wallet: "No wallet provided".to_string(),
            balance_sol: 0.0,
        })
    }
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/balance", get(balance_Request))
        .layer(cors);

    println!("🚀 Server running at http://localhost:3000");

    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
