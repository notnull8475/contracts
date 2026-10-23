# AGENTS.md

Руководство для агентов, работающих с репозиторием «Реестр договоров». Всё написано на основе проверенных фактов из кода и конфигурации. Если не уверены — перепроверьте по указанным файлам.

## 1. Общее описание

Монорепозиторий из трёх частей:

- **`reestr_front/`** — одностраничное приложение (SPA) на Vue 3 + Vite. Чистый JavaScript, TypeScript не используется. Стили — Tailwind CSS 4 + Vuetify 3. Состояние — Pinia.
- **`reestr_back/`** — HTTP API на Rust (edition 2024) + Actix-web 4.11. Доступ к PostgreSQL через Diesel ORM. Аутентификация — JWT в заголовке Authorization.
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
- **CORS:** actix-cors 0.7
- **БД:** PostgreSQL, Diesel ORM 2.2 с фичами `postgres`, `chrono`, `serde_json`, `numeric`
- **Аутентификация:** jsonwebtoken 9.3, bcrypt 0.17
- **Логирование:** log + simple_logger 5
- **HTTP-клиент для DaData:** reqwest 0.12
- **Прочее:** chrono, once_cell, uuid, actix-multipart, bigdecimal, calamine (импорт Excel), encoding_rs, futures-util, serde_json

### Инфраструктура

- Сервер: Ubuntu/Debian-подобная система, nginx, systemd, PostgreSQL.
- Сервер — контейнер Proxmox с диском ~8 ГБ, **расширить нельзя**. Следите за местом: при переполнении падают загрузки файлов (ENOSPC) и сборка при деплое. Docker не используется.

## 3. Структура репозитория

```
.
├── AGENTS.md                 ← этот файл
├── .gitignore                ← игнорирует .env, reestr_back/.env*, files/, .idea/
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
│       ├── smoke.spec.js     ← дымовой тест
│       └── __tests__/contractForm.spec.js ← тесты формы договора
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
    ├── nginx/reestr.conf
    ├── journald/reestr.conf      ← лимит журнала 200M
    └── tools/
        ├── cleanup_orphan_files.sh      ← удаление файлов без записи в БД
        └── suggest_inn.py               ← подбор настоящих ИНН через DaData для организаций с фиктивным 9000000xxx (CSV на проверку, в БД не пишет)
```

## 4. Переменные окружения

Файл `reestr_back/.env` в `.gitignore`; не коммитить. Загружаются через `dotenv` в `src/conf/conf.rs` с помощью `once_cell::Lazy`. Отсутствие обязательной переменной вызывает `panic` при старте.

| Переменная | Назначение |
|------------|-----------|
| `DATABASE_URL` | Строка подключения к PostgreSQL, например `postgres://USER:PASSWORD@localhost:5432/reestrdogovorov` |
| `JWT_SECRET` | Секрет для подписи JWT. Проверяется `assert!(s.len() >= 32)` при старте. При смене все выданные токены станут недействительны |
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
cargo run                        # запуск сервера на 127.0.0.1:8080
cargo run --bin import           # отдельный бинарник импорта
cargo run --bin import -- --file "../Реестр.xlsx" --dry-run
cargo check                      # проверка компиляции
cargo fmt                        # форматирование
cargo test                       # юнит-тесты (имена файлов, разбор цены)
cargo test --no-run              # скомпилировать тестовые бинари без запуска
```

- Для запуска нужны PostgreSQL и заполненный `reestr_back/.env`.
- Бэкенд пишет лог только в stdout → journald (`journalctl -u reestr-back`).

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

Канонический деплой — `git push prod master` (ремоут `prod` → `ssh://reestrdocdeploy/opt/reestrdogovorov.git`, хост `reestrdocdeploy` = `deploy@192.168.0.24` в `~/.ssh/config`). Ремоут `prodserver` указывает на несуществующий хост.

Что происходит на сервере (хук `infra/deploy/post-receive`):

0. Проверка свободного места: меньше 1 ГБ — деплой прерывается
1. `git checkout -f master` → `/opt/reestrdogovorov`
2. `npm ci && npm run build` в `reestr_front/`, затем удаление `node_modules` и кеша npm
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
- Хук не обновляется через `git push`: после правки `infra/deploy/post-receive` скопируйте его в `/opt/reestrdogovorov.git/hooks/post-receive`. То же для nginx (`/etc/nginx/sites-available/reestr`). Встроенные копии в `server_setup.sh` держите синхронными.
- БД находится на отдельном сервере `192.168.0.59`, не в контейнере.

## 7. API и аутентификация

- Все маршруты бэкенда под `/api/v1/*`. Проводка — в `src/main.rs`.
- Административные маршруты сгруппированы под `/api/v1/admin/*`.
- Бэкенд слушает `127.0.0.1:8080` — снаружи доступен только через nginx.
- Ошибки разбора JSON возвращаются как `400 {"error": "..."}` (`json_config()` в `main.rs`).

### Аутентификация — JWT

- `POST /api/v1/login` возвращает `{ "token": "..." }`.
- Фронтенд сохраняет токен в `localStorage` и отправляет в заголовке `Authorization: Bearer <token>` (`src/store/auth.js`, `src/axios.js`).
- Срок жизни токена — 24 часа (`src/auth/auth.rs`). Роль и признак активности при каждом запросе сверяются с БД.
- Защищённые обработчики вызывают `auth::verify_and_extract_claims(&req)`.
- Cookie-сессии нет: раньше подключался actix-session (`image-cookie`), но ни один обработчик его не использовал, и middleware удалён.

### Роли

Авторитетный источник — `src/auth/roles.rs`:

- Строковые значения ролей: `admin`, `moderator`, `user`.
- Административные обработчики (`src/handlers/users.rs` и другие под `/admin`) используют `check_admin_token`, которая требует роль `admin`.
- Остальные защищённые маршруты (договоры, организации и т.д.) требуют только валидный JWT без проверки роли.

### Важная деталь фронтенда

В `src/router/index.js` есть guard `requiresModerator` (пускает `admin` и `moderator`), но сейчас ни один маршрут его не использует. Роли сверяйте с `src/auth/roles.rs`.

### Валидация на бэкенде

Сервисы проверяют входные данные независимо от формы (API доступен любому клиенту с токеном): непустой номер договора, дата окончания не раньше начала, неотрицательные цена и срок, непустое название организации и ИНН из 10 или 12 цифр, хотя бы одно заполненное поле ФИО ответственного. Загрузка файлов принимает только поле формы `file` с именем файла и отвечает 404, если договора нет.

`GET /api/v1/contracts/summary` отдаёт счётчики для главной страницы (договоры, актуальные, организации, ответственные). Актуальный договор — не расторгнутый и с датой окончания в будущем либо без неё.

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
- Известные дефекты импортированных данных: 1367 из 1368 организаций получили фиктивные ИНН `9000000xxx` (настоящих в Excel не было), часть договоров висит на организации-заглушке «Не указано», в ответственных лицах — опечатки и комментарии вместо фамилий. Что восстанавливалось однозначно, исправлено миграциями от 2026-10-22: даты вида `0019-…` (`fix_import_years`), опечатки и комментарии в ответственных лицах (`merge_responsible_persons`), три договора с заглушки на организации с тем же адресом (`fix_placeholder_organizations`). Фиктивные ИНН автоматически не подбираются: `infra/tools/suggest_inn.py` даёт CSV с кандидатами из DaData, выбор делает человек.
- Лист `Доп соглашения  к лиц договору` пропускается.
- При дублировании номера договору добавляется пометка `(дубл)`.

## 11. Организация кода и соглашения

### Rust

- Функции/переменные — `snake_case`, типы — `PascalCase`.
- Сервисы возвращают `Result<T, String>` с описательной ошибкой через `.map_err(|e| format!("...: {}", e))`.
- Большинство хендлеров оборачивают результат в `response_fn()` (`src/utils/utils.rs`), которая разбирает текст ошибки Diesel (`error_response`):
  - успех → `200 OK` + JSON;
  - удаление записи, на которую есть ссылки (`is still referenced`, `update or delete … foreign key`) → `409 Conflict`;
  - вставка/обновление со ссылкой на несуществующую запись (`insert or update … foreign key`) → `400` с названием поля (организация, статус, договор…);
  - `duplicate key` → `409`; `Record not found` → `404`;
  - остальные ошибки → `400 Bad Request` + JSON `{"error": "..."}`.
- Запись в историю договора — через `services::contract_history::record(conn, &dto)`; ошибки логируются, основную операцию не откатывают.
- Исключение: `src/handlers/users.rs` и некоторые админ-обработчики не используют `response_fn` и возвращают текстовые тела, а ошибки `.expect` приводят к панике.
- Модуля `middleware/` нет; middleware (CORS, логгер) подключаются инлайн в `src/main.rs`.

### Фронтенд

- Компоненты используют `<script setup>` и Composition API.
- Алиас `@` → `src/` (настроен в `vite.config.js` и `jsconfig.json`).
- Pinia-сторы в `src/store/`, определены через `defineStore` с опцией `state: () => ({...})`.
- Имена компонентов — PascalCase.
- Prettier: без точек с запятой, одинарные кавычки, `printWidth: 100`.
- Глобальное свойство `$axios` указывает на `apiClient` (`src/main.js`).

## 12. Тестирование

- **Фронтенд:** Vitest. `src/smoke.spec.js` и `src/__tests__/contractForm.spec.js` (обязательные поля, защита от потери данных, сброс состояния между договорами).
- **Бэкенд:** юнит-тесты в `contract_files_handler.rs` (имена файлов), `utils/utils.rs` (разбор цены, классификация ошибок БД), `services/organization.rs` (ИНН); интеграционных нет. `cargo test --no-run` — компиляция без запуска.
- Проверки качества: `npm run format` (Prettier) + `cargo fmt`.

## 13. Безопасность и важные замечания

- `.env` и реальные секреты не должны попадать в git (см. `.gitignore`).
- `JWT_SECRET` должен быть не короче 32 символов; иначе сервер падает на старте.
- Пароли хешируются через bcrypt.
- Токен хранится в `localStorage` и передаётся заголовком; в production желательно включить HTTPS, чтобы он не ходил открытым текстом.
- CORS настроен крайне разрешительно (`allow_any_origin`, `allow_any_method`, `allow_any_header`); снаружи бэкенд закрыт привязкой к 127.0.0.1.
- DaData-запросы выполняются с ключом `DADATAAPIKEY` и внешним вызовом на `suggestions.dadata.ru`.
- Админ-эндпоинты защищены ролью `admin`; остальные защищённые эндпоинты требуют только валидный JWT. Если появится необходимость разграничивать `moderator`/`user`, потребуется дополнительная проверка.
- Не логируйте `DATABASE_URL`: в нём пароль, а журнал читают при отладке.
