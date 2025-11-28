# 🚀 Deployment Guide: SecureShare Telegram Bot на Firebase

## Покрокова інструкція

### 1. Встановлення Firebase CLI

```bash
npm install -g firebase-tools
```

### 2. Авторизація в Firebase

```bash
firebase login
```

### 3. Створення Firebase проекту

1. Відкрийте [Firebase Console](https://console.firebase.google.com/)
2. Натисніть "Add project"
3. Введіть назву проекту (наприклад, `secure-share-bot`)
4. Увімкніть Google Analytics (опціонально)
5. Створіть проект

### 4. Ініціалізація проекту

```bash
cd /Users/blatik/Documents/rust_apps/secure_share/firebase_bot
firebase init
```

Виберіть:
- ✅ Functions
- ✅ Use an existing project → виберіть ваш проект
- ✅ TypeScript
- ✅ ESLint (опціонально)
- ✅ Install dependencies now

### 5. Встановлення залежностей

```bash
cd functions
npm install
```

### 6. Налаштування змінних оточення

Встановіть ваш Telegram Bot Token:

```bash
firebase functions:config:set telegram.token="YOUR_BOT_TOKEN_HERE"
```

Замініть `YOUR_BOT_TOKEN_HERE` на ваш реальний токен від [@BotFather](https://t.me/BotFather).

### 7. Деплой бота

```bash
firebase deploy --only functions
```

### 8. Налаштування Webhook

Після деплою Firebase надасть вам URL функції. Відкрийте в браузері:

```
https://YOUR_REGION-YOUR_PROJECT_ID.cloudfunctions.net/setWebhook
```

Замініть `YOUR_REGION` та `YOUR_PROJECT_ID` на ваші значення.

Ви побачите повідомлення: "Webhook set to: ..."

### 9. Тестування

Відкрийте Telegram і напишіть `/start` вашому боту. Ви повинні побачити повідомлення з кнопкою "🚀 Open SecureShare".

## 🔧 Локальне тестування (опціонально)

```bash
cd functions
npm run serve
```

## 📊 Моніторинг логів

```bash
firebase functions:log
```

## 💰 Вартість

Firebase Functions має безкоштовний tier:
- ✅ 2 мільйони викликів/місяць
- ✅ 400,000 GB-секунд
- ✅ 200,000 CPU-секунд

Для Telegram бота цього більш ніж достатньо!

## 🔄 Оновлення бота

Після змін у коді:

```bash
cd /Users/blatik/Documents/rust_apps/secure_share/firebase_bot
firebase deploy --only functions
```

## ⚠️ Важливо

- Токен бота зберігається в Firebase Functions Config (безпечно)
- Webhook автоматично обробляє всі повідомлення
- Бот працює 24/7 без засинання
- Масштабується автоматично

## 🆘 Troubleshooting

### Бот не відповідає
1. Перевірте логи: `firebase functions:log`
2. Перевірте webhook: відкрийте `/setWebhook` URL
3. Перевірте токен: `firebase functions:config:get`

### Помилка деплою
1. Перевірте, чи встановлені залежності: `cd functions && npm install`
2. Перевірте TypeScript: `npm run build`
3. Перевірте Firebase проект: `firebase use --add`
