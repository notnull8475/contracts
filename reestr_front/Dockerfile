# Сборка приложения
FROM node:18-alpine as builder

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем файлы проекта
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Копируем исходный код
COPY . .
RUN yarn build

# Финальный образ
FROM nginx:alpine

# Копируем собранные файлы
COPY --from=builder /app/dist /usr/share/nginx/html

# Порт для Nginx
EXPOSE 3000

# Команда запуска
CMD ["nginx", "-g", "daemon off;"]