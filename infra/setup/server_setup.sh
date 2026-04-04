#!/bin/bash
# =============================================================================
# server_setup.sh — Разовая инициализация сервера для Реестра договоров
# =============================================================================
# Запуск С МАШИНЫ РАЗРАБОТЧИКА (репозиторий НЕ нужен на сервере):
#
#   ssh root@<SERVER_IP> "bash -s" < infra/setup/server_setup.sh
#   или если есть sudo:
#   ssh <USER>@<SERVER_IP> "sudo bash -s" < infra/setup/server_setup.sh
#
# Скрипт полностью самодостаточен — все конфиги встроены через heredoc.
# =============================================================================

set -euo pipefail

# =============================================================================
# ПЕРЕМЕННЫЕ — можно изменить под своё окружение
# =============================================================================
APP_NAME="reestrdogovorov"
APP_USER="deploy"
APP_DIR="/opt/${APP_NAME}"
GIT_BARE_DIR="/opt/${APP_NAME}.git"
FRONT_STATIC_DIR="/var/www/${APP_NAME}"
SERVICE_NAME="reestr-back"
DEPLOY_BRANCH="master"

# =============================================================================
# Вспомогательные функции
# =============================================================================
log()  { echo -e "\033[1;32m[setup]\033[0m $*"; }
info() { echo -e "\033[1;34m[ info]\033[0m $*"; }
ok()   { echo -e "\033[1;32m[  ok ]\033[0m $*"; }
err()  { echo -e "\033[1;31m[error]\033[0m $*" >&2; exit 1; }
sep()  { echo "----------------------------------------------"; }

# Убеждаемся что запущены как root
if [[ $EUID -ne 0 ]]; then
  err "Запустите скрипт от root: ssh root@SERVER 'bash -s' < server_setup.sh"
fi

log "Начало инициализации сервера"
log "APP_DIR     = ${APP_DIR}"
log "GIT_BARE    = ${GIT_BARE_DIR}"
log "STATIC_DIR  = ${FRONT_STATIC_DIR}"
log "SERVICE     = ${SERVICE_NAME}"
sep

# =============================================================================
# 1. Создаём пользователя deploy (ПЕРВЫМ — остальные шаги от него зависят)
# =============================================================================
log "[1/12] Пользователь ${APP_USER}..."
if ! id "${APP_USER}" &>/dev/null; then
  useradd -m -s /bin/bash "${APP_USER}"
  ok "Пользователь ${APP_USER} создан"
else
  ok "Пользователь ${APP_USER} уже существует"
fi
DEPLOY_HOME="$(getent passwd "${APP_USER}" | cut -d: -f6)"
log "HOME пользователя ${APP_USER}: ${DEPLOY_HOME}"

# =============================================================================
# 2. Системные зависимости
# =============================================================================
log "[2/12] Системные пакеты..."
apt-get update -q
DEBIAN_FRONTEND=noninteractive apt-get install -y -q \
  build-essential \
  curl \
  git \
  pkg-config \
  libssl-dev \
  libpq-dev \
  nginx \
  rsync \
  ca-certificates \
  gnupg
ok "Базовые пакеты установлены"

# =============================================================================
# 3. Node.js LTS
# =============================================================================
log "[3/12] Node.js LTS..."
if ! command -v node &>/dev/null; then
  curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -
  DEBIAN_FRONTEND=noninteractive apt-get install -y -q nodejs
  ok "Node.js $(node --version) установлен"
else
  ok "Node.js уже установлен: $(node --version)"
fi

# =============================================================================
# 4. Rust для пользователя deploy
# =============================================================================
log "[4/12] Rust..."
if ! sudo -u "${APP_USER}" HOME="${DEPLOY_HOME}" bash -c 'command -v cargo &>/dev/null'; then
  sudo -u "${APP_USER}" HOME="${DEPLOY_HOME}" bash -c \
    'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --no-modify-path'
  ok "Rust установлен"
else
  ok "Rust уже установлен"
fi

# Добавляем cargo в PATH для deploy
CARGO_ENV="${DEPLOY_HOME}/.cargo/env"
if [[ ! -f "${DEPLOY_HOME}/.bashrc" ]] || ! grep -q 'cargo/env' "${DEPLOY_HOME}/.bashrc"; then
  echo "source \"${CARGO_ENV}\"" >> "${DEPLOY_HOME}/.bashrc"
fi

CARGO_BIN="${DEPLOY_HOME}/.cargo/bin"

# =============================================================================
# 5. diesel_cli
# =============================================================================
log "[5/12] diesel_cli..."
if ! sudo -u "${APP_USER}" HOME="${DEPLOY_HOME}" PATH="${CARGO_BIN}:${PATH}" \
    bash -c 'command -v diesel &>/dev/null'; then
  sudo -u "${APP_USER}" HOME="${DEPLOY_HOME}" PATH="${CARGO_BIN}:${PATH}" \
    bash -c 'cargo install diesel_cli --no-default-features --features postgres'
  ok "diesel_cli установлен"
else
  ok "diesel_cli уже установлен"
fi

# =============================================================================
# 6. Директории приложения
# =============================================================================
log "[6/12] Директории..."
mkdir -p "${APP_DIR}" "${APP_DIR}/reestr_back/files" "${FRONT_STATIC_DIR}"
chown -R "${APP_USER}:${APP_USER}" "${APP_DIR}" "${FRONT_STATIC_DIR}"
ok "Директории созданы"

# =============================================================================
# 7. Bare git-репозиторий
# =============================================================================
log "[7/12] Bare git-репозиторий ${GIT_BARE_DIR}..."
if [[ ! -d "${GIT_BARE_DIR}" ]]; then
  mkdir -p "${GIT_BARE_DIR}"
  chown -R "${APP_USER}:${APP_USER}" "${GIT_BARE_DIR}"
  sudo -u "${APP_USER}" HOME="${DEPLOY_HOME}" git init --bare "${GIT_BARE_DIR}"
  ok "Bare репозиторий создан"
else
  ok "Bare репозиторий уже существует"
fi

# =============================================================================
# 8. Git-хук post-receive (встроен в скрипт через heredoc)
# =============================================================================
log "[8/12] Git-хук post-receive..."
HOOK_FILE="${GIT_BARE_DIR}/hooks/post-receive"

cat > "${HOOK_FILE}" << HOOK_SCRIPT
#!/bin/bash
# =============================================================================
# post-receive — автодеплой при git push prodserver master
# =============================================================================
set -euo pipefail

APP_DIR="${APP_DIR}"
GIT_BARE_DIR="${GIT_BARE_DIR}"
FRONT_STATIC_DIR="${FRONT_STATIC_DIR}"
SERVICE_NAME="${SERVICE_NAME}"
DEPLOY_BRANCH="${DEPLOY_BRANCH}"
CARGO_BIN="${CARGO_BIN}"

export PATH="\${CARGO_BIN}:\${PATH}"
export HOME="${DEPLOY_HOME}"

log()  { echo -e "\033[1;34m[deploy]\033[0m \$*"; }
ok()   { echo -e "\033[1;32m[  ok  ]\033[0m \$*"; }
fail() { echo -e "\033[1;31m[ fail ]\033[0m \$*" >&2; exit 1; }

DEPLOYED=0
while read -r OLD_REV NEW_REV REF_NAME; do
  BRANCH="\${REF_NAME##refs/heads/}"
  if [[ "\${BRANCH}" != "\${DEPLOY_BRANCH}" ]]; then
    log "Ветка '\${BRANCH}' пропускается (деплоим только '\${DEPLOY_BRANCH}')"
    continue
  fi

  DEPLOYED=1
  log "========================================"
  log "Деплой ветки '\${BRANCH}' (\${NEW_REV:0:8})"
  log "\$(date '+%Y-%m-%d %H:%M:%S')"
  log "========================================"

  # 1. Обновляем код
  log "[1/6] Обновляем исходный код..."
  export GIT_WORK_TREE="\${APP_DIR}"
  export GIT_DIR="\${GIT_BARE_DIR}"
  git checkout -f "\${DEPLOY_BRANCH}"
  unset GIT_DIR GIT_WORK_TREE
  ok "Код обновлён"

  # 2. Сборка фронтенда
  log "[2/6] Сборка фронтенда..."
  cd "\${APP_DIR}/reestr_front"
  npm ci --silent              || fail "npm ci завершился с ошибкой"
  npm run build                || fail "npm run build завершился с ошибкой"
  ok "Фронтенд собран"

  # 3. Публикация статики
  log "[3/6] Публикация статики → \${FRONT_STATIC_DIR}..."
  mkdir -p "\${FRONT_STATIC_DIR}"
  rsync -a --delete "\${APP_DIR}/reestr_front/dist/" "\${FRONT_STATIC_DIR}/"
  ok "Статика опубликована"

  # 4. Сборка бэкенда
  log "[4/6] Сборка бэкенда (cargo build --release)..."
  cd "\${APP_DIR}/reestr_back"
  cargo build --release        || fail "cargo build завершился с ошибкой"
  ok "Бэкенд собран"

  # 5. Миграции БД
  log "[5/6] Применение миграций..."
  cd "\${APP_DIR}/reestr_back"
  # Фиксим diesel.toml (перезатирается git checkout)
  sed -i 's|dir = "/home/mechanicus/.*"|dir = "/opt/reestrdogovorov/reestr_back/migrations"|' diesel.toml
  diesel migration run         || fail "diesel migration run завершился с ошибкой"
  ok "Миграции применены"

  # 6. Перезапуск сервиса
  log "[6/6] Перезапуск сервиса \${SERVICE_NAME}..."
  sudo systemctl restart "\${SERVICE_NAME}" || fail "Не удалось перезапустить \${SERVICE_NAME}"
  sleep 2
  if systemctl is-active --quiet "\${SERVICE_NAME}"; then
    ok "Сервис \${SERVICE_NAME} запущен"
  else
    fail "\${SERVICE_NAME} не запустился. Лог: journalctl -u \${SERVICE_NAME} -n 30"
  fi

  log "========================================"
  ok "Деплой завершён! \$(date '+%Y-%m-%d %H:%M:%S')"
  log "========================================"
done

if [[ \$DEPLOYED -eq 0 ]]; then
  log "Ветка '\${DEPLOY_BRANCH}' не найдена в push — деплой не выполнялся"
fi
HOOK_SCRIPT

chown "${APP_USER}:${APP_USER}" "${HOOK_FILE}"
chmod +x "${HOOK_FILE}"
ok "Хук post-receive создан"

# =============================================================================
# 9. Systemd unit (встроен через heredoc)
# =============================================================================
log "[9/12] Systemd unit ${SERVICE_NAME}..."

cat > "/etc/systemd/system/${SERVICE_NAME}.service" << SYSTEMD_UNIT
[Unit]
Description=Reestr Dogovorov — Actix-web Backend
After=network.target
Wants=network.target

[Service]
Type=simple
User=${APP_USER}
Group=${APP_USER}
WorkingDirectory=${APP_DIR}/reestr_back
EnvironmentFile=${APP_DIR}/reestr_back/.env
ExecStart=${APP_DIR}/reestr_back/target/release/reestr_back
Restart=on-failure
RestartSec=5s
NoNewPrivileges=true
PrivateTmp=true
StandardOutput=journal
StandardError=journal
SyslogIdentifier=${SERVICE_NAME}

[Install]
WantedBy=multi-user.target
SYSTEMD_UNIT

systemctl daemon-reload
systemctl enable "${SERVICE_NAME}"
ok "Systemd unit создан и включён"

# =============================================================================
# 10. Nginx конфиг приложения (встроен через heredoc)
# =============================================================================
log "[10/12] Nginx конфиг..."

NGINX_CONF="/etc/nginx/sites-available/${APP_NAME}.conf"

cat > "${NGINX_CONF}" << NGINX_CONF_CONTENT
# Реестр договоров — nginx конфиг
# Внешний прокси → этот сервер :80
#   /api/ → 127.0.0.1:8080 (Actix-web)
#   /     → /var/www/${APP_NAME} (Vue dist)

server {
    listen 80;

    # Замените на реальное имя хоста (то, что настроено на внешнем прокси)
    server_name reestr.your-domain.local;

    root ${FRONT_STATIC_DIR};
    index index.html;

    # Frontend — SPA
    location / {
        try_files \$uri \$uri/ /index.html;

        location ~* \.(js|css|woff2?|ttf|eot|svg|png|jpg|ico)\$ {
            expires 30d;
            add_header Cache-Control "public, immutable";
            access_log off;
        }
    }

    # Backend API
    location /api/ {
        proxy_pass         http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header   Host              \$host;
        proxy_set_header   X-Real-IP         \$remote_addr;
        proxy_set_header   X-Forwarded-For   \$proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto \$scheme;
        proxy_connect_timeout 10s;
        proxy_read_timeout    60s;
        proxy_send_timeout    60s;
        client_max_body_size  50M;
    }

    access_log /var/log/nginx/${APP_NAME}.access.log;
    error_log  /var/log/nginx/${APP_NAME}.error.log warn;
}
NGINX_CONF_CONTENT

ln -sf "${NGINX_CONF}" "/etc/nginx/sites-enabled/${APP_NAME}.conf"
rm -f /etc/nginx/sites-enabled/default

nginx -t && systemctl reload nginx
ok "Nginx конфиг установлен"

# =============================================================================
# 11. .env шаблон для бэкенда
# =============================================================================
log "[11/12] .env шаблон..."
ENV_FILE="${APP_DIR}/reestr_back/.env"

if [[ ! -f "${ENV_FILE}" ]]; then
  mkdir -p "${APP_DIR}/reestr_back"
  cat > "${ENV_FILE}" << ENV_TEMPLATE
# =====================================================
# ОБЯЗАТЕЛЬНО ЗАПОЛНИТЬ ПЕРЕД ПЕРВЫМ ДЕПЛОЕМ
# Редактировать: nano ${ENV_FILE}
# =====================================================
DATABASE_URL=postgres://USER:PASSWORD@DB_HOST:5432/reestrdogovorov
JWT_SECRET=replace_with_random_string_at_least_32_chars_long
DADATAAPIKEY=your_dadata_api_key
ENV_TEMPLATE

  chown "${APP_USER}:${APP_USER}" "${ENV_FILE}"
  chmod 600 "${ENV_FILE}"
  ok ".env шаблон создан (заполните перед первым деплоем)"
else
  ok ".env уже существует — не перезаписываем"
fi

# =============================================================================
# 12. Права sudoers для deploy (перезапуск сервиса без пароля)
# =============================================================================
log "[12/12] Sudoers для ${APP_USER}..."
SUDOERS_FILE="/etc/sudoers.d/${APP_USER}-reestr"

cat > "${SUDOERS_FILE}" << SUDOERS_CONTENT
# Позволяет пользователю ${APP_USER} управлять сервисом без пароля
${APP_USER} ALL=(ALL) NOPASSWD: /usr/bin/systemctl restart ${SERVICE_NAME}
${APP_USER} ALL=(ALL) NOPASSWD: /usr/bin/systemctl start ${SERVICE_NAME}
${APP_USER} ALL=(ALL) NOPASSWD: /usr/bin/systemctl stop ${SERVICE_NAME}
${APP_USER} ALL=(ALL) NOPASSWD: /usr/bin/systemctl is-active ${SERVICE_NAME}
${APP_USER} ALL=(ALL) NOPASSWD: /usr/bin/systemctl status ${SERVICE_NAME}
SUDOERS_CONTENT

chmod 440 "${SUDOERS_FILE}"
visudo -cf "${SUDOERS_FILE}" && ok "Sudoers настроен" || err "Ошибка в sudoers — проверьте ${SUDOERS_FILE}"

# =============================================================================
# Готово
# =============================================================================
echo ""
log "============================================================"
log " Сервер инициализирован!"
log "============================================================"
echo ""
info " Следующие шаги:"
echo ""
info " 1. Заполните .env на сервере:"
info "    nano ${ENV_FILE}"
echo ""
info " 2. Замените server_name в nginx конфиге:"
info "    nano /etc/nginx/sites-available/${APP_NAME}.conf"
info "    (замените 'reestr.your-domain.local' на реальное имя)"
info "    nginx -t && systemctl reload nginx"
echo ""
info " 3. Добавьте SSH-ключ для пользователя ${APP_USER} (если нужен):"
info "    mkdir -p ${DEPLOY_HOME}/.ssh"
info "    echo 'ВАШ_ПУБЛИЧНЫЙ_КЛЮЧ' >> ${DEPLOY_HOME}/.ssh/authorized_keys"
info "    chown -R ${APP_USER}:${APP_USER} ${DEPLOY_HOME}/.ssh"
info "    chmod 700 ${DEPLOY_HOME}/.ssh && chmod 600 ${DEPLOY_HOME}/.ssh/authorized_keys"
echo ""
info " 4. Добавьте remote на машине разработчика:"
info "    git remote add prodserver ssh://${APP_USER}@<SERVER_IP>${GIT_BARE_DIR}"
echo ""
info " 5. Первый деплой:"
info "    git push prodserver master"
echo ""
