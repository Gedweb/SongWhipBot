use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct SoundWhipRequest {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct SoundWhipResponse {
    name : String,
    artists: Vec<Artist>,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    name: String,
}
