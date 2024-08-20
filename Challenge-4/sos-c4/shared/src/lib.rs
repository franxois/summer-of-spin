use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterRequest {
    pub player_name: String,
    pub player_no: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RecordRequest {
    pub player_no: u8,
    pub calories: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DBRecord {
    pub player_name: String,
    pub calories: u64,
}
