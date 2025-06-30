use axum::{Json, response::IntoResponse};
use serde::Deserialize;
use spl_token::instruction::initialize_mint;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::Instruction;
use solana_sdk::system_instruction;
use spl_associated_token_account::get_associated_token_address;
use base64::engine::general_purpose;
use base64::Engine;





use crate::types::{SuccessResponse, ErrorResponse};
use serde_json::json;
use bs58;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    let mint_pubkey = bs58::decode(&payload.mint).into_vec();
    let authority_pubkey = bs58::decode(&payload.mintAuthority).into_vec();

    if mint_pubkey.is_err() || authority_pubkey.is_err() {
        return Json(ErrorResponse {
            success: false,
            error: "Invalid base58-encoded pubkey".to_string(),
        }).into_response();
    }

    let mint_pubkey = Pubkey::new(&mint_pubkey.unwrap());
    let authority_pubkey = Pubkey::new(&authority_pubkey.unwrap());

    let instr: Instruction = match initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &authority_pubkey,
        None,
        payload.decimals,
    ) {
        Ok(i) => i,
        Err(e) => {
            return Json(ErrorResponse {
                success: false,
                error: format!("Failed to build instruction: {}", e),
            }).into_response();
        }
    };

    let accounts: Vec<_> = instr.accounts.iter().map(|meta| {
        json!({
            "pubkey": meta.pubkey.to_string(),
            "is_signer": meta.is_signer,
            "is_writable": meta.is_writable
        })
    }).collect();

    let response = json!({
        "program_id": instr.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": general_purpose::STANDARD.encode(&instr.data)

,
    });

    Json(SuccessResponse {
        success: true,
        data: response,
    }).into_response()
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    let mint = bs58::decode(&payload.mint).into_vec();
    let dest = bs58::decode(&payload.destination).into_vec();
    let auth = bs58::decode(&payload.authority).into_vec();

    if mint.is_err() || dest.is_err() || auth.is_err() {
        return Json(ErrorResponse {
            success: false,
            error: "Invalid base58 pubkey in mint, destination or authority".to_string(),
        }).into_response();
    }

    let mint = Pubkey::new(&mint.unwrap());
    let dest = Pubkey::new(&dest.unwrap());
    let auth = Pubkey::new(&auth.unwrap());

    let instr = match spl_token::instruction::mint_to(
        &spl_token::id(),
        &mint,
        &dest,
        &auth,
        &[],
        payload.amount,
    ) {
        Ok(i) => i,
        Err(e) => {
            return Json(ErrorResponse {
                success: false,
                error: format!("Failed to build mint_to instruction: {}", e),
            }).into_response();
        }
    };

    let accounts: Vec<_> = instr.accounts.iter().map(|meta| {
        json!({
            "pubkey": meta.pubkey.to_string(),
            "is_signer": meta.is_signer,
            "is_writable": meta.is_writable
        })
    }).collect();

    let response = json!({
        "program_id": instr.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": general_purpose::STANDARD.encode(&instr.data),


    });

    Json(SuccessResponse {
        success: true,
        data: response,
    }).into_response()
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    let from_bytes = bs58::decode(&payload.from).into_vec();
    let to_bytes = bs58::decode(&payload.to).into_vec();

    if from_bytes.is_err() || to_bytes.is_err() {
        return Json(ErrorResponse {
            success: false,
            error: "Invalid base58 address in 'from' or 'to'".to_string(),
        }).into_response();
    }

    let from_pubkey = Pubkey::new(&from_bytes.unwrap());
    let to_pubkey = Pubkey::new(&to_bytes.unwrap());

 let instr = system_instruction::transfer(&from_pubkey, &to_pubkey, payload.lamports);


    let response = json!({
        "program_id": instr.program_id.to_string(),
        "accounts": instr.accounts.iter().map(|a| a.pubkey.to_string()).collect::<Vec<_>>(),
        "instruction_data": general_purpose::STANDARD.encode(&instr.data)

,
    });

    Json(SuccessResponse {
        success: true,
        data: response,
    }).into_response()
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    destination: String,
    mint: String,
    owner: String,
    amount: u64,
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> impl IntoResponse {
    let mint_bytes = bs58::decode(&payload.mint).into_vec();
    let dest_bytes = bs58::decode(&payload.destination).into_vec();
    let owner_bytes = bs58::decode(&payload.owner).into_vec();

    if mint_bytes.is_err() || dest_bytes.is_err() || owner_bytes.is_err() {
        return Json(ErrorResponse {
            success: false,
            error: "Invalid base58 address".to_string(),
        }).into_response();
    }

    let mint = Pubkey::new(&mint_bytes.unwrap());
    let dest = Pubkey::new(&dest_bytes.unwrap());
    let owner = Pubkey::new(&owner_bytes.unwrap());

    let source_ata = get_associated_token_address(&owner, &mint);
    let dest_ata = get_associated_token_address(&dest, &mint);

    let instr = match spl_token::instruction::transfer(
        &spl_token::id(),
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        payload.amount,
    ) {
        Ok(i) => i,
        Err(e) => {
            return Json(ErrorResponse {
                success: false,
                error: format!("Failed to create instruction: {}", e),
            }).into_response();
        }
    };

    let accounts: Vec<_> = instr.accounts.iter().map(|meta| {
        json!({
            "pubkey": meta.pubkey.to_string(),
            "isSigner": meta.is_signer,
        })
    }).collect();

    let response = json!({
        "program_id": instr.program_id.to_string(),
        "accounts": accounts,
        "instruction_data": general_purpose::STANDARD.encode(&instr.data),
    });

    Json(SuccessResponse {
        success: true,
        data: response,
    }).into_response()
}
