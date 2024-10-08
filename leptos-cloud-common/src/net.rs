use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckNameRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckNameResponse {
    pub available: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogResponse {
    pub log: String,
}
