#![feature(async_closure)]

use reqwest;
use teloxide::prelude::*;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::sync::Arc;

mod dto;

const ACCEPTABLE_LINKS: [&str; 11] = [
    "music.amazon.com",
    "deezer.com",
    "music.apple",
    "napster.com",
    "pandora.com",
    "soundcloud.com",
    "spotify.com",
    "tidal.com",
    "music.yandex.ru",
    "youtube.com",
    "youtu.be",
];

fn extract_url(text: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(https?://)?(www\.)?([-\p{Alphabetic}\d]+\.)+([a-z]{2,63})/[-\p{Alphabetic}\d._~:/?#\[\]@!$&'()*+,;=%]+"
        ).expect("url extract regexp");
    }
    RE.find(text).map(|url| url.as_str())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting SoundWhip bot...");

    let bot = Arc::new(Bot::from_env().auto_send());

    teloxide::repl(bot.clone(), async move |message| {
        if let Some(url) = message.update.text().and_then(|text| extract_url(text)) {
            if ACCEPTABLE_LINKS.iter().find(|x| url.contains(*x)).is_none() {
                return respond(());
            }

            let mut reply_text = "not found 😕".to_string();
            if let Ok(response) = send(url).await {
                reply_text = format!(
                    "{} - {}\n{}",
                    response.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(" & "),
                    response.name,
                    response.url,
                );
            }

            message.reply_to(reply_text).await?;
        }
        respond(())
    }).await;
}

async fn send<T: ToString>(url: T) -> reqwest::Result<dto::SoundWhipResponse>
{
    let client = reqwest::Client::new();
    let response = client.post("https://songwhip.com/")
        .json(&dto::SoundWhipRequest {
            url: url.to_string(),
        })
        .send().await?;

    if response.status() != reqwest::StatusCode::OK {
        log::warn!("{} {}", response.status(), url.to_string())
    }

    response.json::<dto::SoundWhipResponse>().await
}
