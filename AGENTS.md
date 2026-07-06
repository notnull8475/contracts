# AGENTS.md

Руководство для агентов, работающих с репозиторием «Реестр договоров». Всё написано на основе проверенных фактов из кода и конфигурации. Если не уверены — перепроверьте по указанным файлам.

## 1. Общее описание

Монорепозиторий из трёх частей:

- **`reestr_front/`** — одностраничное приложение (SPA) на Vue 3 + Vite. Чистый JavaScript, TypeScript не используется. Стили — Tailwind CSS 4 + Vuetify 3. Состояние — Pinia.
- **`reestr_back/`** — HTTP API на Rust (edition 2024) + Actix-web 4.11. Доступ к PostgreSQL через Diesel ORM. Аутентификация — JWT + cookie-сессия Actix.
- **`infra/`** — скрипты и конфиги для развёртывания на сервере через `git push`. Подробности в `infra/README.md`.

Приложение — реестр договоров с организациями, ответственными лицами, статусами, дополнительными соглашениями, историей изменений статусов, загрузкой файлов и импортом из Excel.

## 2. Технологический стек

### Фронтенд (`reestr_front/`)

- **Сборка:** Vite 7 (`vite.config.js`)
- **Фреймворк:** Vue 3.5, Vue Router 4
- **Состояние:** Pinia 3
- **UI:** Vuetify 3, @mdi/font, @heroicons/vue
- **Стили:** Tailwind CSS 4 + `@tailwindcss/vite`, `src/index.css`, `src/assets/main.css`
- **Сетевой клиент:** Axios (`src/axios.js`)
- **Даты:** dayjs, @vuepic/vue-datepicker, flatpickr
- **Тесты:** Vitest 3.2 + jsdom + @vue/test-utils
- **Форматирование:** Prettier 3.5 (конфиг `.prettierrc.json`)

### Бэкенд (`reestr_back/`)

- **Язык:** Rust, edition 2024
- **Веб-фреймворк:** actix-web 4.11
- **CORS / сессии:** actix-cors 0.7, actix-session 0.10 (cookie-session)
- **БД:** PostgreSQL, Diesel ORM 2.2 с фичами `postgres`, `chrono`, `serde_json`, `numeric`
- **Аутентификация:** jsonwebtoken 9.3, bcrypt 0.17, sha2
- **Логирование:** log + simple_logger 5
- **HTTP-клиент для DaData:** reqwest 0.12
- **Прочее:** chrono, once_cell, uuid, actix-multipart, bigdecimal, calamine (импорт Excel), encoding_rs, futures-util, serde_json

### Инфраструктура

- Сервер: Ubuntu/Debian-подобная система, nginx, systemd, PostgreSQL.
- Docker-файлы (`reestr_front/Dockerfile`, `reestr_back/Dockerfile`, `reestr_back/docker-compose.yml`) **не используются** в каноническом деплое. Frontend-Dockerfile ссылается на несуществующий `yarn.lock` и использует команду `npm build` вместо `npm run build`; `docker-compose.yml` имеет синтаксические проблемы. Не полагайтесь на них.

## 3. Структура репозитория

```
.
├── AGENTS.md                 ← этот файл
├── .gitignore                ← игнорирует .env, reestr_back/.env, files/, application.log
├── reestr_front/
│   ├── package.json
│   ├── vite.config.js
│   ├── .prettierrc.json
│   ├── jsconfig.json         ← алиас @ → ./src
│   └── src/
│       ├── main.js           ← инициализация Vue/Pinia/Vuetify/axios
│       ├── App.vue
│       ├── axios.js          ← центральный axios-клиент
│       ├── router/index.js   ← маршруты и guards
│       ├── store/            ← Pinia-модули: auth.js, contracts.js, organizations.js, ...
│       ├── views/            ← страницы
│       ├── components/       ← компоненты: forms/, lists/, admin/
│       ├── layouts/AppLayout.vue
│       ├── composables/useNotify.js
│       ├── plugins/vuetify.js, plugins/markdown.js
│       └── smoke.spec.js     ← единственный тест
├── reestr_back/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── diesel.toml           ← [print_schema] и абсолютный путь к migrations
│   ├── migrations/           ← миграции Diesel
│   └── src/
│       ├── main.rs           ← точка входа, маршрутизация
│       ├── schema.rs         ← сгенерирован Diesel (не править руками)
│       ├── auth/             ← JWT, роли, логин
│       ├── conf/             ← загрузка .env
│       ├── handlers/         ← HTTP-обработчики
│       ├── services/         ← бизнес-логика и запросы к БД
│       ├── models/           ← модели и DTO
│       ├── utils/            ─ db, response_fn, create_admin_user, dadata
│       └── bin/import.rs     ─ отдельный бинарник импорта из Excel
└── infra/
    ├── README.md
    ├── setup/server_setup.sh
    ├── deploy/post-receive
    ├── systemd/reestr-back.service
    └── nginx/reestr.conf
```

## 4. Переменные окружения

Файл `reestr_back/.env` в `.gitignore`; не коммитить. Загружаются через `dotenv` в `src/conf/conf.rs` с помощью `once_cell::Lazy`. Отсутствие обязательной переменной вызывает `panic` при старте.

| Переменная | Назначение |
|------------|-----------|
| `DATABASE_URL` | Строка подключения к PostgreSQL, например `postgres://USER:PASSWORD@localhost:5432/reestrdogovorov` |
| `JWT_SECRET` | Секрет для подписи JWT и производных ключей. Проверяется `assert!(s.len() >= 32)` при старте. При смене все выданные токены и cookie-сессии станут недействительны |
| `DADATAAPIKEY` | API-ключ DaData (`https://suggestions.dadata.ru`) для автозаполнения организации по ИНН |

При первом запуске, если в БД нет пользователя с ролью `admin`, бэкенд автоматически создаёт пользователя `admin`/`admin` (`src/utils/create_admin_user.rs`). Смените пароль.

## 5. Команды разработки, сборки и тестирования

### Фронтенд

Все команды выполняются из `reestr_front/`:

```bash
npm install            # установка зависимостей
npm run dev            # dev-сервер Vite, по умолчанию :5173
npm run build          # production-сборка → dist/
npm run preview        # предпросмотр собранной сборки
npm run test:unit      # Vitest в watch-режиме
npm run test:unit -- --run path/to/file.spec.js   # один раз, один файл
npm run format         # Prettier --write src/
```

- Линтера и typecheck нет (чистый JS).
- В dev-режиме фронт ходит напрямую на `http://localhost:8080` (`src/axios.js`). Для разработки нужен запущенный бэкенд.
- В production `baseURL` пустой: запросы идут на тот же хост, nginx проксирует `/api/` на бэкенд.

### Бэкенд

Все команды выполняются из `reestr_back/`:

```bash
cargo run                        # запуск сервера на 0.0.0.0:8080
cargo run --bin import           # отдельный бинарник импорта
cargo run --bin import -- --file "../Реестр.xlsx" --dry-run
cargo check                      # проверка компиляции
cargo fmt                        # форматирование
cargo test                       # запуск тестов (сейчас тестов нет)
cargo test --no-run              # скомпилировать тестовые бинари без запуска
```

- Для запуска нужны PostgreSQL и заполненный `reestr_back/.env`.
- Бэкенд пишет лог в `application.log` и в stdout/journal.

### Миграции Diesel

```bash
diesel migration generate имя    # создать миграцию (up.sql/down.sql)
diesel migration run             # применить
diesel migration revert          # откатить последнюю
diesel print-schema              # перегенерировать src/schema.rs
```

- `src/schema.rs` генерируется автоматически. Не редактировать вручную.
- В `diesel.toml` поле `[migrations_directory] dir` содержит абсолютный путь локальной машины. На сервере хук `post-receive` перезаписывает его на `/opt/reestrdogovorov/reestr_back/migrations`.

## 6. Развёртывание

Канонический деплой — `git push prodserver master` (или `git push prodserver HEAD:master`).

Что происходит на сервере (хук `infra/deploy/post-receive`):

1. `git checkout -f master` → `/opt/reestrdogovorov`
2. `npm ci && npm run build` в `reestr_front/`
3. `rsync -a --delete reestr_front/dist/` → `/var/www/reestrdogovorov`
4. `cargo build --release` в `reestr_back/`
5. `diesel migration run` (с предварительным `sed`-фиксом пути в `diesel.toml`)
6. `sudo systemctl restart reestr-back`

Важно:

- Деплоится **только ветка `master`**. Другие ветки игнорируются.
- Первичная настройка сервера делается скриптом `infra/setup/server_setup.sh` (один раз от root).
- systemd-юнит `reestr-back` загружает `.env` через `EnvironmentFile` и запускает `/opt/reestrdogovorov/reestr_back/target/release/reestr_back`.
- nginx (конфиг `infra/nginx/reestr.conf`) отдаёт статику фронтенда (`/var/www/reestrdogovorov`) в режиме SPA (fallback на `index.html`) и проксирует `/api/` на `127.0.0.1:8080`.
- Логи: `journalctl -u reestr-back -n 50 -f` и `/var/log/nginx/reestrdogovorov.*.log`.

## 7. API и аутентификация

- Все маршруты бэкенда под `/api/v1/*`. Проводка — в `src/main.rs`.
- Административные маршруты сгруппированы под `/api/v1/admin/*`.
- Бэкенд слушает `0.0.0.0:8080`.

### Аутентификация двойная

1. **JWT:**
   - `POST /api/v1/login` возвращает `{ "token": "..." }`.
   - Фронтенд сохраняет токен в `localStorage` и отправляет в заголовке `Authorization: Bearer <token>` (`src/store/auth.js`, `src/axios.js`).
   - Срок жизни токена — 24 часа (`src/auth/auth.rs`).
   - Защищённые обработчики вызывают `auth::verify_and_extract_claims(&req)`.

2. **Cookie-сессия Actix:**
   - Имя куки: `image-cookie`.
   - Настройки: `cookie_secure(false)`, `SameSite::Strict`, `HttpOnly`, `CookieContentSecurity::Private`.
   - Ключ шифрования куки производится от `JWT_SECRET` через SHA512 (`src/auth/auth.rs::cookie_session_key`).

### Роли

Авторитетный источник — `src/auth/roles.rs`:

- Строковые значения ролей: `admin`, `moderator`, `user`.
- Административные обработчики (`src/handlers/users.rs` и другие под `/admin`) используют `check_admin_token`, которая требует роль `admin`.
- Остальные защищённые маршруты (договоры, организации и т.д.) требуют только валидный JWT без проверки роли.

### Важная деталь фронтенда

В `src/router/index.js` есть guard `requiresManager`, который сравнивает роль со строкой `'manager'`. Бэкенд такую роль не возвращает (он возвращает `moderator`), и сейчас ни один маршрут не использует `requiresManager`. Это потенциальное рассогласование — имейте в виду при добавлении ролевых проверок.

## 8. База данных

Основные таблицы (см. сгенерированный `src/schema.rs`):

- `users` — пользователи с ролью и хешем пароля.
- `organization` — организации (с ИНН, адресами, руководителем).
- `responsible_person` — ответственные лица.
- `contract` — договоры.
- `supplementary_agreement` — дополнительные соглашения к договорам.
- `contract_files` — загруженные файлы (хранятся в `reestr_back/files/`, в БД — метаданные).
- `contract_history` — история изменений статусов договоров.
- `dict_contract_status`, `dict_pricelist`, `dict_type_of_validity` — справочники.

Связи описаны в `src/schema.rs` через `diesel::joinable!`.

## 9. Загрузка файлов

- Директория для файлов: `./files` относительно рабочей директории бэкенда (`src/services/contract_files.rs`).
- При старте вызывается `contract_files::init_files_dir()`; если папки нет, она создаётся.
- Файлы сохраняются под именем `{contract_id}_{uuid}.bin`; оригинальное имя хранится в `orig_name`.
- Поле `file_type` в `contract_files` различает файлы договора и дополнительных соглашений.
- Загрузка ограничена в nginx (`client_max_body_size 50M`).

## 10. Импорт из Excel

Отдельный бинарник `src/bin/import.rs`:

```bash
cargo run --bin import -- --file "../Реестр.xlsx"
cargo run --bin import -- --file "../Реестр.xlsx" --dry-run
```

- Читает `.xlsx` через `calamine`.
- Создаёт или находит организации и ответственных лиц.
- Лист с названием `Лист3` считается закрытыми договорами и получает `contract_status_id = 4`.
- Лист `Доп соглашения  к лиц договору` пропускается.
- При дублировании номера договору добавляется пометка `(дубл)`.

## 11. Организация кода и соглашения

### Rust

- Функции/переменные — `snake_case`, типы — `PascalCase`.
- Сервисы возвращают `Result<T, String>` с описательной ошибкой через `.map_err(|e| format!("...: {}", e))`.
- Большинство хендлеров оборачивают результат в `response_fn()` (`src/utils/utils.rs`):
  - успех → `200 OK` + JSON;
  - ошибки с подстроками `foreign key`, `violates`, `is still referenced` → `409 Conflict` с фиксированным сообщением об использовании в связанных данных;
  - остальные ошибки → `400 Bad Request` + JSON `{"error": "..."}`.
- Исключение: `src/handlers/users.rs` и некоторые админ-обработчики не используют `response_fn` и возвращают текстовые тела, а ошибки `.expect` приводят к панике.
- Модуля `middleware/` нет; middleware (CORS, логгер, сессия) подключаются инлайн в `src/main.rs`.

### Фронтенд

- Компоненты используют `<script setup>` и Composition API.
- Алиас `@` → `src/` (настроен в `vite.config.js` и `jsconfig.json`).
- Pinia-сторы в `src/store/`, определены через `defineStore` с опцией `state: () => ({...})`.
- Имена компонентов — PascalCase.
- Prettier: без точек с запятой, одинарные кавычки, `printWidth: 100`.
- Глобальное свойство `$axios` указывает на `apiClient` (`src/main.js`).

## 12. Тестирование

- **Фронтенд:** Vitest. Сейчас есть только дымовой тест `src/smoke.spec.js`.
- **Бэкенд:** юнит- и интеграционных тестов нет. `cargo test` компилирует тестовые бинари, но не запускает реальных проверок. Для компиляции без запуска используйте `cargo test --no-run`.
- Проверки качества: `npm run format` (Prettier) + `cargo fmt`.

## 13. Безопасность и важные замечания

- `.env` и реальные секреты не должны попадать в git (см. `.gitignore`).
- `JWT_SECRET` должен быть не короче 32 символов; иначе сервер падает на старте.
- Пароли хешируются через bcrypt.
- Cookie-сессия шифруется, имеет флаг `HttpOnly` и `SameSite=Strict`, но `cookie_secure(false)` — в production желательно включить HTTPS и перевести в `true`.
- CORS настроен крайне разрешительно (`allow_any_origin`, `allow_any_method`, `allow_any_header`) — только для разработки/внутренних сетей; на публичном продакшене нужно ограничить.
- DaData-запросы выполняются с ключом `DADATAAPIKEY` и внешним вызовом на `suggestions.dadata.ru`.
- Админ-эндпоинты защищены ролью `admin`; остальные защищённые эндпоинты требуют только валидный JWT. Если появится необходимость разграничивать `moderator`/`user`, потребуется дополнительная проверка.
- Docker- и compose-файлы не используются в текущем деплое и содержат ошибки; не опирайтесь на них.
