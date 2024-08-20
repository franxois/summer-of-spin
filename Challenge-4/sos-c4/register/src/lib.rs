use serde_json::Error;
use shared::{DBRecord, RegisterRequest};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

/// A simple Spin HTTP component.
#[http_component]
fn handle_record(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let request = req.body();

    let register_request_result: Result<RegisterRequest, Error> = serde_json::from_slice(&request);

    if register_request_result.is_err() {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body("{\"error\": \"Invalid request\"}")
            .build());
    }

    let register_request = register_request_result?;

    let kv = Store::open_default()?;

    if kv
        .exists(&register_request.player_no.to_string())
        .unwrap_or(false)
    {
        return Ok(Response::builder()
            .status(409)
            .header("content-type", "application/json")
            .body("{\"error\": \"Player number already exists\"}")
            .build());
    }

    kv.set_json(
        &register_request.player_no.to_string(),
        &DBRecord {
            player_name: register_request.player_name,
            calories: 0,
        },
    )?;

    Ok(Response::builder()
        .status(201)
        .header("content-type", "application/json")
        .body(std::str::from_utf8(&request).unwrap())
        .build())
}
