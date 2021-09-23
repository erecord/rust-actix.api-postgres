use serde::Serialize;

#[derive(Serialize)]
pub struct Status{
    pub status: String,
    pub number: u8
}