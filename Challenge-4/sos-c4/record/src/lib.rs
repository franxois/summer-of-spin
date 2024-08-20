use serde_json::Error;
use shared::{DBRecord, RecordRequest};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

/// A simple Spin HTTP component.
#[http_component]
fn handle_record(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let request = req.body();

    let record_request_result: Result<RecordRequest, Error> = serde_json::from_slice(&request);

    if record_request_result.is_err() {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body("{\"error\": \"Invalid request\"}")
            .build());
    }

    let record_request = record_request_result?;

    let kv = Store::open_default()?;

    if !kv
        .exists(&record_request.player_no.to_string())
        .unwrap_or(false)
    {
        return Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body("{\"error\": \"Player not found\"}")
            .build());
    }

    let player: DBRecord = kv
        .get_json(&record_request.player_no.to_string())?
        .expect("Player not found");

    println!("player: {:?}", player);

    let new_record = DBRecord {
        player_name: player.player_name,
        calories: player.calories + record_request.calories,
    };

    kv.set_json(&record_request.player_no.to_string(), &new_record)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&new_record)?)
        .build())
}
