#![feature(async_closure)]

mod dto;
use reqwest;
use serde_json;

use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;
use teloxide::prelude::*;
use tokio::io::AsyncReadExt;
use reqwest::Response;


#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        if let Some(text) = message.update.text() {
            message.reply_to(text).await?;
        }
        respond(())
    })
        .await;
}


async fn send<T: ToString>(url: T) -> Result<dto::SoundWhipResponse, reqwest::Error>
{
    // curl --request POST --data '{"url":"MY_SOURCE_MUSIC_LINK"}' https://songwhip.com/
    let body = serde_json::to_string(&dto::SoundWhipRequest {
        url: url.to_string(),
    });


    let client = reqwest::Client::new();
    client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await
        .map(async move |x| serde_json::from_str(x.text().await.map(|y| y.as_str())?))
}
