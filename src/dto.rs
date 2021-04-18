use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
pub struct SoundWhipRequest {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct SoundWhipResponse {
    pub name : String,
    pub artists: Vec<Artist>,
    pub url: String,
    pub links: HashMap<String, bool>,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,
}
