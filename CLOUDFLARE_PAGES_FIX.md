# Правильне налаштування Cloudflare Pages

## Проблема
Cloudflare намагається використати Wrangler (Workers), а потрібен Pages (статичний хостинг).

## Рішення

### Крок 1: Видалити поточний проект
1. Перейдіть на https://dash.cloudflare.com/
2. **Pages** → знайдіть ваш проект
3. **Settings** → **Delete project**

### Крок 2: Створити новий проект ПРАВИЛЬНО

1. **Pages** → **Create a project**
2. **Connect to Git** → виберіть `Blatik/secure_share`
3. **Set up builds and deployments:**

```
Framework preset: None
Build command: ./build.sh
Build output directory: frontend/dist
Root directory: (залишити порожнім)
```

4. **Environment variables** (додайте):
```
NODE_VERSION = 18
```

5. **Save and Deploy**

### Важливо!
НЕ вибирайте "Workers" або "Wrangler" - тільки **Pages**!

## Альтернатива: Ручний deploy

Якщо автоматичний build не працює:

```bash
# Локально
cd /Users/blatik/Documents/rust_apps/secure_share
trunk build --release --public-url / -d frontend

# Встановіть Wrangler
npm install -g wrangler

# Deploy вручну
npx wrangler pages deploy frontend/dist --project-name=secure-share
```

## Після успішного deploy

URL буде: `https://secure-share.pages.dev`

Оновіть в `telegram_bot/src/main.rs` якщо URL інший.
