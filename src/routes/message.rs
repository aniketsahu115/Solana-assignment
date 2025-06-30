use axum::{response::IntoResponse, Json};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::Deserialize;
use serde_json::json;

use crate::types::{ErrorResponse, SuccessResponse};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    message: String,
    secret: String,
}

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Invalid base58 secret key".to_string(),
            }).into_response();
        }
    };

    if secret_bytes.len() != 64 {
        return Json(ErrorResponse {
            success: false,
            error: format!("Expected 64 bytes secret key, got {}", secret_bytes.len()),
        })
        .into_response();
    }

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Failed to parse keypair from secret key".to_string(),
            })
            .into_response();
        }
    };

    let signature = keypair.sign(payload.message.as_bytes());
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());

    let response = json!({
        "signature": signature_b64,
        "public_key": bs58::encode(keypair.public.to_bytes()).into_string(),
        "message": payload.message,
    });

    Json(SuccessResponse {
        success: true,
        data: response,
    })
    .into_response()
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    message: String,
    signature: String,
    pubkey: String,
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    let pubkey_bytes = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Invalid base58 public key".to_string(),
            })
            .into_response();
        }
    };

    let signature_bytes = match general_purpose::STANDARD.decode(&payload.signature) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Invalid base64 signature".to_string(),
            })
            .into_response();
        }
    };

    if pubkey_bytes.len() != 32 || signature_bytes.len() != 64 {
        return Json(ErrorResponse {
            success: false,
            error: "Incorrect public key or signature length".to_string(),
        })
        .into_response();
    }

    let public_key = match PublicKey::from_bytes(&pubkey_bytes) {
        Ok(pk) => pk,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Failed to construct public key".to_string(),
            })
            .into_response();
        }
    };

    let signature = match Signature::from_bytes(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => {
            return Json(ErrorResponse {
                success: false,
                error: "Failed to parse signature".to_string(),
            })
            .into_response();
        }
    };

    let is_valid = public_key.verify(payload.message.as_bytes(), &signature).is_ok();

    let response = json!({
        "valid": is_valid,
        "message": payload.message,
        "pubkey": payload.pubkey,
    });

    Json(SuccessResponse {
        success: true,
        data: response,
    })
    .into_response()
}
