import * as functions from 'firebase-functions';
import TelegramBot from 'node-telegram-bot-api';

// Initialize bot with webhook mode (required for Firebase Functions)
const bot = new TelegramBot(functions.config().telegram.token);

// Webhook handler
export const telegramWebhook = functions.https.onRequest(async (req, res) => {
    try {
        if (req.method === 'POST') {
            await bot.processUpdate(req.body);
            res.sendStatus(200);
        } else {
            res.sendStatus(405); // Method Not Allowed
        }
    } catch (error) {
        console.error('Error processing update:', error);
        res.sendStatus(500);
    }
});

// Set up webhook URL
export const setWebhook = functions.https.onRequest(async (req, res) => {
    const webhookUrl = `https://${req.hostname}/telegramWebhook`;
    try {
        await bot.setWebHook(webhookUrl);
        res.send(`Webhook set to: ${webhookUrl}`);
    } catch (error) {
        console.error('Error setting webhook:', error);
        res.status(500).send('Error setting webhook');
    }
});

// Bot message handlers
bot.onText(/\/start/, (msg) => {
    const chatId = msg.chat.id;
    const webAppUrl = 'https://secure-share-78e.pages.dev/';

    const keyboard = {
        inline_keyboard: [[
            {
                text: '🚀 Open SecureShare',
                web_app: { url: webAppUrl }
            }
        ]]
    };

    const message = `🔒 *SecureShare*

Share files securely with end\\-to\\-end encryption\\.

✅ Files up to 100MB
✅ 10 minute storage
✅ No registration needed
✅ Military\\-grade encryption`;

    bot.sendMessage(chatId, message, {
        parse_mode: 'MarkdownV2',
        reply_markup: keyboard
    });
});

// Handle all other messages
bot.on('message', (msg) => {
    if (!msg.text?.startsWith('/')) {
        const chatId = msg.chat.id;
        bot.sendMessage(
            chatId,
            'Use /start to open SecureShare and share files securely! 🔒'
        );
    }
});
