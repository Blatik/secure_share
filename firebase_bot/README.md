# Firebase Functions для Telegram Bot

Цей каталог містить Firebase Functions для деплою Telegram бота.

## Налаштування

1. Встановіть Firebase CLI:
```bash
npm install -g firebase-tools
```

2. Увійдіть в Firebase:
```bash
firebase login
```

3. Ініціалізуйте проект:
```bash
firebase init functions
```

4. Встановіть залежності:
```bash
cd functions
npm install
```

5. Встановіть змінні оточення:
```bash
firebase functions:config:set telegram.token="YOUR_BOT_TOKEN"
```

## Деплой

```bash
firebase deploy --only functions
```

## Локальне тестування

```bash
cd functions
npm run serve
```
