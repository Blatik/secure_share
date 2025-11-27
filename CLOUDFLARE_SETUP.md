# Налаштування Cloudflare Pages

## Крок 1: Створення проекту на Cloudflare Pages

1. Перейдіть на https://dash.cloudflare.com/
2. Виберіть **Pages** в лівому меню
3. Натисніть **Create a project**
4. Виберіть **Connect to Git**
5. Авторизуйте Cloudflare для доступу до вашого GitHub
6. Виберіть репозиторій `Blatik/secure_share`

## Крок 2: Налаштування Build

Використайте такі параметри:

### Framework preset
- **None** (або Custom)

### Build command
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xz && \
chmod +x trunk && \
rustup target add wasm32-unknown-unknown && \
cd frontend && \
../trunk build --release
```

### Build output directory
```
frontend/dist
```

### Root directory (optional)
Залишіть порожнім або `/`

### Environment variables
Додайте:
- `RUST_VERSION` = `1.75.0` (або новіша)

## Крок 3: Deploy

1. Натисніть **Save and Deploy**
2. Cloudflare автоматично:
   - Встановить Rust
   - Встановить Trunk
   - Зібере проект
   - Опублікує на Cloudflare Pages

## Крок 4: Налаштування домену (опціонально)

Після успішного deploy ви отримаєте URL типу:
```
https://secure-share.pages.dev
```

Ви можете:
1. Використовувати цей URL
2. Або налаштувати власний домен в **Custom domains**

## Переваги Cloudflare Pages

✅ **Безкоштовно** - без обмежень на Actions
✅ **Швидше** - глобальна CDN
✅ **Автоматичні деплої** - при кожному push
✅ **Preview deployments** - для кожного PR
✅ **Кращий DDoS захист**
✅ **Аналітика** - вбудована

## Оновлення посилань

Після deploy оновіть посилання в коді:

### В `telegram_bot/src/main.rs`:
```rust
let web_app_url = "https://secure-share.pages.dev/"; // або ваш custom domain
```

### В `frontend/src/main.rs` (для Telegram посилань):
Посилання генеруються динамічно, тому автоматично працюватимуть з новим доменом.

## Troubleshooting

Якщо build не вдається:

1. **Перевірте логи** в Cloudflare Dashboard
2. **Збільште timeout** в налаштуваннях проекту
3. **Використайте Docker** (альтернативний метод):

Створіть `Dockerfile.cloudflare`:
```dockerfile
FROM rust:1.75

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cd frontend && trunk build --release
```

## Автоматичні деплої

Після налаштування, кожен push в `main` автоматично тригерить новий deploy. Не потрібно нічого додатково налаштовувати!
