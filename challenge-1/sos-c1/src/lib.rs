use serde::{Deserialize, Serialize};
use spin_sdk::http;
use spin_sdk::http::{send, IntoResponse, Request, Response};
use spin_sdk::http_component;
use std::str::from_utf8;

#[derive(Debug, Deserialize)]
pub struct DecryptResponse {
    pub response: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalResponse {
    pub encrypted_message: String,
}

/// A simple Spin HTTP component.
#[http_component]
async fn handle_sos_1(req: Request) -> anyhow::Result<impl IntoResponse> {
    let body = from_utf8(req.body())?;

    let secret_play = "4-4-2";
    let encryption_module_path = "crypto";

    let uri = format!(
        "{}{}",
        req.header("spin-full-url")
            .unwrap()
            .as_str()
            .unwrap_or("http://localhost:3000"),
        encryption_module_path
    );

    let mut decrypt_req = Request::post(&uri, body);
    decrypt_req.header("x-action", "decrypt");
    let decrypt_req_response: http::Response = send(decrypt_req).await?;

    let decrypt_response: DecryptResponse = serde_json::from_slice(decrypt_req_response.body())?;

    let mut encrypt_req = Request::post(uri, decrypt_response.response + secret_play);
    encrypt_req.header("x-action", "encrypt");
    let encrypt_req_response: http::Response = send(encrypt_req).await?;

    let encrypt_response: DecryptResponse = serde_json::from_slice(encrypt_req_response.body())?;

    let resp_str = serde_json::to_string(&FinalResponse {
        encrypted_message: encrypt_response.response,
    })?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .header("x-secret-play", secret_play)
        .header("x-encryption-module-path", encryption_module_path)
        .body(resp_str)
        .build())
}
