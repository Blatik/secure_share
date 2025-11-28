# Cloudflare Workers Bot Deployment

## Setup

1. **Directory**: `cloudflare_bot/`
2. **Worker**: `secure-share-bot`
3. **URL**: `https://secure-share-bot.blatik-short.workers.dev`

## Commands Used

```bash
# Deploy
npx wrangler deploy

# Set Secret
echo "YOUR_TOKEN" | npx wrangler secret put TELEGRAM_TOKEN

# Set Webhook
curl https://secure-share-bot.blatik-short.workers.dev/setWebhook
```

## Status
✅ Deployed
✅ Secret Set
✅ Webhook Configured
