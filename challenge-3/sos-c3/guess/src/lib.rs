use serde::{Deserialize, Serialize};
use shared::{Game, NextResponse};
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueryParams {
    pub game_id: String,
    pub guess: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_guess(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let rec_params: QueryParams = serde_qs::from_str(req.query())?;

    println!("query: {:?}", req.query());
    println!("rec_params: {:?}", rec_params);

    if !shared::get_word_list().contains(&rec_params.guess.as_str()) {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body("{\"error\": \"Invalid guess\"}")
            .build());
    }

    let kv = Store::open_default()?;

    if !kv.exists(&rec_params.game_id.as_str()).unwrap_or(false) {
        return Ok(Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body("{\"error\": \"Game not found\"}")
            .build());
    }

    let game: Game = kv.get_json(&rec_params.game_id)?.expect("Game not found");

    println!("game: {:?}", game);
    //let game: Game = serde_json::from_slice(&kv.get(&rec_params.game_id.as_str()?)?)?;

    if game.response.solved {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body("{\"error\": \"Game is already solved\"}")
            .build());
    }

    if game.response.current_row >= shared::MAX_ATTEMPTS as u8 {
        return Ok(Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body("{\"error\": \"Too much retry\"}")
            .build());
    }

    if &rec_params.guess == &game.solution {
        let response: NextResponse = NextResponse {
            message: "You have guessed the word".to_string(),
            game_id: rec_params.game_id,
            current_row: game.response.current_row + 1,
            solved: true,
            grid: game.response.grid,
            correct_letters: rec_params
                .guess
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
        };

        kv.set_json(
            &response.game_id,
            &Game {
                response: response.clone(),
                solution: game.solution.clone(),
            },
        )?;

        return Ok(Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&response)?)
            .build());
    }

    let current_row = game.response.current_row as usize;
    let mut current_grid = game.response.grid;

    current_grid[current_row] = rec_params
        .guess
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();

    let response: NextResponse = NextResponse {
        message: "Try again".to_string(),
        game_id: rec_params.game_id,
        current_row: current_row as u8 + 1,
        solved: false,
        grid: current_grid,
        correct_letters: get_correct_letters(
            &rec_params.guess,
            &game.solution,
            game.response.correct_letters,
        ),
    };

    println!("fail response: {:?}", response);

    kv.set_json(
        &response.game_id,
        &Game {
            response: response.clone(),
            solution: game.solution.clone(),
        },
    )?;

    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string(&response)?)
        .build())
}

fn get_correct_letters(guess: &str, solution: &str, mut correct_letters: [char; 5]) -> [char; 5] {
    for (i, c) in guess.chars().enumerate() {
        if solution.chars().collect::<Vec<char>>()[i] == c {
            correct_letters[i] = c;
        }
    }

    correct_letters
}
