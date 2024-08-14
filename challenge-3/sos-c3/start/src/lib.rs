use rand::Rng;
use shared::{Game, NextResponse, StartResponse, MAX_ATTEMPTS};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;
use uuid::Uuid;

/// A simple Spin HTTP component.
#[http_component]
fn handle_start(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let word_list = shared::get_word_list();

    let message = "The game has started, start guessing the word";

    let game_id = Uuid::new_v4();

    let kv = Store::open_default()?;

    let resp = StartResponse {
        message: message.to_string(),
        game_id: game_id.to_string(),
        current_row: 0,
        solved: false,
        grid: [[' '; 5]; MAX_ATTEMPTS],
    };

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..word_list.len());

    let new_game = Game {
        response: NextResponse {
            message: message.to_string(),
            game_id: game_id.to_string(),
            current_row: 0,
            solved: false,
            grid: [[' '; 5]; MAX_ATTEMPTS],
            correct_letters: [' '; 5],
        },
        solution: word_list[idx].to_string(),
    };

    println!("Game: {:?}", new_game);

    kv.set(
        &game_id.to_string(),
        serde_json::to_string(&new_game)?.as_bytes(),
    )?;

    let resp_str = serde_json::to_string(&resp)?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(resp_str)
        .build())
}
