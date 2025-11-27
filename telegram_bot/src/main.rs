use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo}};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting SecureShare Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let web_app_url = "https://blatik.github.io/secure_share/";
        
        // Create the keyboard with the Web App button
        let keyboard = InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::web_app("🚀 Open SecureShare", WebAppInfo { url: web_app_url.parse().unwrap() }),
        ]]);

        bot.send_message(msg.chat.id, "Welcome to SecureShare! 🔒\n\nShare files securely with end-to-end encryption directly in Telegram.")
            .reply_markup(keyboard)
            .await?;

        Ok(())
    })
    .await;

    Ok(())
}
