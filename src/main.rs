#![feature(async_closure)]

mod dto;

use reqwest;

use teloxide::prelude::*;

const ACCEPTABLE_LINKS: [&str; 10] = [
    "amazon",
    "deezer",
    "itunes",
    "napster",
    "pandora",
    "soundcloud",
    "spotify",
    "tidal",
    "yandex",
    "youtube",
];

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting SoundWhip bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        if let Some(text) = message.update.text() {
            if !(text.starts_with("http") || ACCEPTABLE_LINKS.iter().find(|x| text.contains(*x)).is_none()) {
                return respond(());
            }

            if let Ok(response) = send(text).await {
                let formatted = format!(
                    "{} - {}\n{}",
                    response.artists.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(" "),
                    response.name,
                    response.url,
                );

                message.reply_to(formatted).await?;
            }
        }
        respond(())
    })
        .await;
}


async fn send<T: ToString>(url: T)
                           -> reqwest::Result<dto::SoundWhipResponse>
{
    let client = reqwest::Client::new();
    let response = client.post("https://songwhip.com/")
        .json(&dto::SoundWhipRequest {
            url: url.to_string(),
        })
        .send().await?;

    response.json::<dto::SoundWhipResponse>().await
}
