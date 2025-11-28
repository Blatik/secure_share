export default {
    async fetch(request, env, ctx) {
        // Handle Webhook setup
        const url = new URL(request.url);
        if (url.pathname === "/setWebhook") {
            const webhookUrl = `https://${url.hostname}/webhook`;
            const token = env.TELEGRAM_TOKEN;
            if (!token) return new Response("TELEGRAM_TOKEN not set", { status: 500 });

            const response = await fetch(`https://api.telegram.org/bot${token}/setWebhook?url=${webhookUrl}`);
            const data = await response.json();
            return new Response(JSON.stringify(data, null, 2), {
                headers: { "content-type": "application/json" }
            });
        }

        // Handle incoming updates
        if (request.method === "POST" && url.pathname === "/webhook") {
            try {
                const update = await request.json();
                if (update.message) {
                    await handleMessage(update.message, env);
                }
                return new Response("OK");
            } catch (e) {
                return new Response("Error processing update", { status: 500 });
            }
        }

        return new Response("SecureShare Bot is running! 🚀\nGo to /setWebhook to configure.", { status: 200 });
    },
};

async function handleMessage(message, env) {
    const chatId = message.chat.id;
    const text = message.text || "";
    const token = env.TELEGRAM_TOKEN;

    if (text.startsWith("/start")) {
        const webAppUrl = "https://secure-share-78e.pages.dev/";

        const replyMarkup = {
            inline_keyboard: [[
                {
                    text: "🚀 Open SecureShare",
                    web_app: { url: webAppUrl }
                }
            ]]
        };

        const welcomeText = `🔒 *SecureShare*

Share files securely with end\\-to\\-end encryption\\.

✅ Files up to 100MB
✅ 10 minute storage
✅ No registration needed
✅ Military\\-grade encryption`;

        await sendMessage(chatId, welcomeText, token, replyMarkup);
    } else {
        await sendMessage(chatId, "Use /start to open SecureShare and share files securely! 🔒", token);
    }
}

async function sendMessage(chatId, text, token, replyMarkup = null) {
    const url = `https://api.telegram.org/bot${token}/sendMessage`;
    const body = {
        chat_id: chatId,
        text: text,
        parse_mode: "MarkdownV2"
    };

    if (replyMarkup) {
        body.reply_markup = replyMarkup;
    }

    await fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
    });
}
