use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub data: Value,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}
