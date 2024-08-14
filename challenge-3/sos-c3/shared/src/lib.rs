use serde::{Deserialize, Serialize};

const WORDS: &str = include_str!("words");

pub fn get_word_list() -> Vec<&'static str> {
    WORDS.split("\n").filter(|w| w.len() > 0).collect()
}

pub const MAX_ATTEMPTS: usize = 5;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartResponse {
    pub message: String,
    pub game_id: String,
    pub current_row: u8,
    pub solved: bool,
    pub grid: [[char; 5]; MAX_ATTEMPTS],
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NextResponse {
    pub message: String,
    pub game_id: String,
    pub current_row: u8,
    pub solved: bool,
    pub grid: [[char; 5]; MAX_ATTEMPTS],
    pub correct_letters: [char; 5],
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub response: NextResponse,
    pub solution: String,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
