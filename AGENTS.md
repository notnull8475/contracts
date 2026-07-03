# AGENTS.md

Руководство для агентов, работающих с этим репозиторием. Только проверенные факты из кода.

## Структура проекта

Монорепозиторий из трёх частей:
- **`reestr_front/`** — Vue 3 + Vite + Pinia + Vuetify 3 + Tailwind CSS 4 (чистый JavaScript, без TypeScript)
- **`reestr_back/`** — Rust (edition 2024) + Actix-web + Diesel ORM + PostgreSQL
- **`infra/`** — скрипты и конфиги деплоя (см. `infra/README.md`)

## Команды

### Фронтенд (`reestr_front/`)
```bash
npm install            # установить зависимости
npm run dev            # dev-сервер Vite (по умолчанию :5173)
npm run build          # production-сборка → dist/
npm run preview        # предпросмотр собранной сборки
npm run test:unit      # Vitest (окружение jsdom)
npm run test:unit -- --run path/to/file.spec.js   # один файл, без watch
npm run format         # Prettier (только src/): без точек с запятой, одинарные кавычки, printWidth 100
```
- **Линтера и typecheck НЕТ** (это JS). Из проверок — только `npm run format` и `npm run test:unit`.
- `npm run dev` ходит за API на `http://localhost:8080` — для разработки нужен запущенный бэкенд.

### Бэкенд (`reestr_back/`)
```bash
cargo run                        # запуск сервера (слушает 0.0.0.0:8080)
cargo run --bin import           # отдельный бинарник импорта данных (src/bin/import.rs)
cargo check                      # проверка компиляции (вместо линтера)
cargo fmt                        # форматирование
cargo test                       # все тесты
cargo test имя_теста             # один тест
```
- Нужны `PostgreSQL` и файл `reestr_back/.env` (см. ниже) — иначе упадёт при старте.

### Миграции (Diesel)
```bash
diesel migration generate имя    # создать миграцию (папка с up.sql/down.sql)
diesel migration run             # применить
diesel migration revert          # откатить последнюю
diesel print-schema              # перегенерировать src/schema.rs
```
- **`src/schema.rs` генерируется** (`[print_schema]` в `diesel.toml`) — не правьте руками, обновляйте через миграции.

## Переменные окружения

Файл `reestr_back/.env` (в `.gitignore`, никогда не коммитить). Читаются в `src/conf/conf.rs`:

| Переменная | Назначение |
|------------|-----------|
| `DATABASE_URL` | строка подключения к PostgreSQL (обязательна) |
| `JWT_SECRET` | секрет для подписи JWT **и** шифрования cookie-сессии. Минимум 32 символа — **проверяется `assert!` при старте**. Смена значения инвалидирует все токены и сессии |
| `DADATAAPIKEY` | API-ключ DaData для автозаполнения по ИНН |

При первом запуске без пользователя-админа бэкенд **автоматически создаёт `admin`/`admin`** (`src/utils/create_admin_user.rs`) — смените пароль.

## Деплой (канонический поток)

Подробно — в `infra/README.md`. Кратко:

```bash
git push prodserver master
```

Push в bare-репо на сервере запускает git-хук `post-receive`, который:
1. `git checkout -f master` → `/opt/reestrdogovorov`
2. `npm ci && npm run build` (фронтенд)
3. `rsync dist/` → `/var/www/reestrdogovorov`
4. `cargo build --release` (бэкенд)
5. `diesel migration run`
6. `systemctl restart reestr-back`

Важно:
- **Деплоится только ветка `master`.** Другие ветки игнорируются хуком.
- nginx отдаёт статику фронтенда (SPA, fallback на `index.html`), `/api/` проксирует на `127.0.0.1:8080`.
- Логи на сервере: `journalctl -u reestr-back -n 50 -f` (бэкенд пишет в stdout, не в файл).
- **Гочча `diesel.toml`:** в `[migrations_directory]` лежит абсолютный путь машины разработчика. Хук переписывает его на `/opt/reestrdogovorov/reestr_back/migrations` через `sed` при каждом пуше. Локально diesel-CLI рассчитывает именно на этот абсолютный путь.
- **Docker-файлы (`Dockerfile`, `docker-compose.yml`) не используются** — на проде Docker не установлен, работает только systemd+nginx. compose-файл к тому же синтаксически сломан; не полагайтесь на него.

## Архитектура и конвенции

### Роли (авторитетный источник — `reestr_back/src/auth/roles.rs`)
Строковые значения: **`admin`**, **`moderator`**, **`user`**.

> Внимание: во фронтенде `src/router/index.js` route-meta `requiresManager` сравнивается со строкой `'manager'`, которой бэкенд **не возвращает** (он отдаёт `'moderator'`). Это рассинхрон — учитывайте при работе с ролями.

### API
- Все маршруты под `/api/v1/*`; админские — под `/api/v1/admin/*` (проводка в `src/main.rs`).
- Бэкенд слушает `0.0.0.0:8080`.

### Аутентификация (двойная)
- `/login` возвращает **JWT** (хранится во фронтенде в `localStorage`, шлётся через `Authorization: Bearer`).
- Дополнительно включена middleware `actix-session` (cookie `image-cookie`, `cookie_secure=false`). Обе части ключаются от `JWT_SECRET`.

### Фронтенд — сеть
- Центральный клиент `apiClient` в `src/axios.js`: в dev `baseURL = http://localhost:8080`, в prod — пустой (запросы идут на тот же хост, их проксирует nginx).
- При ответе 401 со словом "token" в теле — автоматический logout и редирект на `/login`.

### Структура бэкенда (`reestr_back/src/`)
```
main.rs         # точка входа, проводка маршрутов
auth/           # JWT, роли, логин
conf/           # загрузка .env (conf.rs)
handlers/       # HTTP-обработчики
services/       # бизнес-логика и работа с БД
models/         # модели и DTO
schema.rs       # генерируется Diesel (не править руками)
utils/          # db, response_fn, create_admin_user, dadata
bin/import.rs   # отдельный бинарник импорта данных
```
Модуля `middleware/` нет — middleware подключаются инлайн в `main.rs`.

### Паттерны кода (Rust)
- Сервисы возвращают `Result<T, String>` с описательной ошибкой через `.map_err(|e| format!("...: {}", e))`.
- Хендлеры оборачивают результат через `response_fn()` (`src/utils/utils.rs`): нарушение FK → `409`, остальное → `400`, успех → `200` с JSON.
- Имена: функции/переменные — `snake_case`, типы — `PascalCase`.

### Паттерны кода (фронтенд)
- `<script setup>` + Composition API; алиас `@` → `src/`.
- Состояние — Pinia (`defineStore`, опция `state: () => ({...})`).
- Имена компонентов — PascalCase; кавычки одинарные, точек с запятой нет (см. `.prettierrc.json`).
