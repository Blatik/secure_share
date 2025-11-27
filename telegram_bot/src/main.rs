use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, WebAppInfo}};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting SecureShare Bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // Use Cloudflare Pages URL - update this after deployment
        let web_app_url = "https://secure-share.pages.dev/";
        
        // Create the keyboard with the Web App button
        let keyboard = InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::web_app(
                "🚀 Open SecureShare", 
                WebAppInfo { url: web_app_url.parse().unwrap() }
            ),
        ]]);

        bot.send_message(
            msg.chat.id, 
            "🔒 *SecureShare*\n\n\
            Share files securely with end\\-to\\-end encryption\\.\n\n\
            ✅ Files up to 100MB\n\
            ✅ 10 minute storage\n\
            ✅ No registration needed\n\
            ✅ Military\\-grade encryption"
        )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    })
    .await;

    Ok(())
}
