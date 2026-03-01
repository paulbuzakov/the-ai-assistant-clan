mod consts;

use crate::consts::TELEGRAM_TOKEN_ENV;
use shared::logger;
use shared::settings::Settings;

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    logger::init();

    let settings = Settings::from_env().expect("failed to init settings");

    let token = match settings.get_value(TELEGRAM_TOKEN_ENV) {
        Ok(t) => t,
        Err(e) => {
            logger::error!("{e}");
            return;
        }
    };

    logger::info!("Telegram bot is starting...");

    let bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(text) = msg.text() {
            logger::info!("Received message: {}", text);
            bot.send_message(msg.chat.id, format!("Echo: {}", text)).await?;
        }
        respond(())
    })
    .await;
}
