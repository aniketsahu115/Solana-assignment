use axum::{Json, response::IntoResponse};
use bs58;
use solana_sdk::signature::{Keypair, Signer};
use crate::types::SuccessResponse;

use serde_json::json;

pub async fn generate_keypair() -> impl IntoResponse {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    let data = json!({
        "pubkey": pubkey,
        "secret": secret
    });

    Json(SuccessResponse { success: true, data }).into_response()
}
