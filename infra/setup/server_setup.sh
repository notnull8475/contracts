#!/bin/bash
# =============================================================================
# server_setup.sh — Разовая инициализация сервера для Реестра договоров
# =============================================================================
# Запуск:
#   ssh deploy@<SERVER_IP> "bash -s" < infra/setup/server_setup.sh
# или напрямую на сервере:
#   bash infra/setup/server_setup.sh
# =============================================================================

set -e

APP_NAME="reestrdogovorov"
APP_USER="deploy"
APP_DIR="/opt/${APP_NAME}"
GIT_BARE_DIR="/opt/${APP_NAME}.git"
FRONT_STATIC_DIR="/var/www/${APP_NAME}"
SERVICE_NAME="reestr-back"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"

log() { echo -e "\033[1;32m[setup]\033[0m $*"; }
err() { echo -e "\033[1;31m[error]\033[0m $*" >&2; exit 1; }

# -----------------------------------------------------------------------------
# 1. Проверяем, что мы root или можем sudo
# -----------------------------------------------------------------------------
if [[ $EUID -ne 0 ]]; then
  SUDO="sudo"
  $SUDO true || err "Нужны права sudo"
else
  SUDO=""
fi

# -----------------------------------------------------------------------------
# 2. Системные зависимости
# -----------------------------------------------------------------------------
log "Обновляем пакеты и устанавливаем зависимости..."
$SUDO apt-get update -q
$SUDO apt-get install -y -q \
  build-essential \
  curl \
  git \
  pkg-config \
  libssl-dev \
  libpq-dev \
  nginx \
  rsync

# Node.js LTS через NodeSource
if ! command -v node &>/dev/null; then
  log "Устанавливаем Node.js LTS..."
  curl -fsSL https://deb.nodesource.com/setup_lts.x | $SUDO bash -
  $SUDO apt-get install -y -q nodejs
fi
log "Node.js: $(node --version), npm: $(npm --version)"

# -----------------------------------------------------------------------------
# 3. Rust (для пользователя deploy)
# -----------------------------------------------------------------------------
if ! sudo -u "${APP_USER}" bash -c 'command -v cargo &>/dev/null'; then
  log "Устанавливаем Rust для пользователя ${APP_USER}..."
  sudo -u "${APP_USER}" bash -c \
    'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable'
fi
log "Rust: $(sudo -u "${APP_USER}" bash -c 'source ~/.cargo/env && rustc --version')"

# -----------------------------------------------------------------------------
# 4. diesel_cli
# -----------------------------------------------------------------------------
if ! sudo -u "${APP_USER}" bash -c 'source ~/.cargo/env && command -v diesel &>/dev/null'; then
  log "Устанавливаем diesel_cli..."
  sudo -u "${APP_USER}" bash -c \
    'source ~/.cargo/env && cargo install diesel_cli --no-default-features --features postgres'
fi
log "diesel: $(sudo -u "${APP_USER}" bash -c 'source ~/.cargo/env && diesel --version')"

# -----------------------------------------------------------------------------
# 5. Пользователь deploy (если не существует)
# -----------------------------------------------------------------------------
if ! id "${APP_USER}" &>/dev/null; then
  log "Создаём пользователя ${APP_USER}..."
  $SUDO useradd -m -s /bin/bash "${APP_USER}"
fi

# -----------------------------------------------------------------------------
# 6. Директории приложения
# -----------------------------------------------------------------------------
log "Создаём директории..."
$SUDO mkdir -p "${APP_DIR}" "${FRONT_STATIC_DIR}"
$SUDO chown -R "${APP_USER}:${APP_USER}" "${APP_DIR}" "${FRONT_STATIC_DIR}"

# -----------------------------------------------------------------------------
# 7. Bare git-репозиторий
# -----------------------------------------------------------------------------
if [[ ! -d "${GIT_BARE_DIR}" ]]; then
  log "Создаём bare git-репозиторий ${GIT_BARE_DIR}..."
  $SUDO mkdir -p "${GIT_BARE_DIR}"
  $SUDO chown -R "${APP_USER}:${APP_USER}" "${GIT_BARE_DIR}"
  sudo -u "${APP_USER}" git init --bare "${GIT_BARE_DIR}"
fi

# -----------------------------------------------------------------------------
# 8. Хук post-receive
# -----------------------------------------------------------------------------
log "Устанавливаем git-хук post-receive..."
HOOK_SRC="${REPO_ROOT}/infra/deploy/post-receive"
HOOK_DST="${GIT_BARE_DIR}/hooks/post-receive"

$SUDO cp "${HOOK_SRC}" "${HOOK_DST}"
$SUDO chown "${APP_USER}:${APP_USER}" "${HOOK_DST}"
$SUDO chmod +x "${HOOK_DST}"

# -----------------------------------------------------------------------------
# 9. Systemd unit
# -----------------------------------------------------------------------------
log "Устанавливаем systemd unit..."
$SUDO cp "${REPO_ROOT}/infra/systemd/${SERVICE_NAME}.service" "/etc/systemd/system/${SERVICE_NAME}.service"
$SUDO systemctl daemon-reload
$SUDO systemctl enable "${SERVICE_NAME}"

# -----------------------------------------------------------------------------
# 10. .env файл бэкенда (если не существует)
# -----------------------------------------------------------------------------
ENV_FILE="${APP_DIR}/reestr_back/.env"
if [[ ! -f "${ENV_FILE}" ]]; then
  log "Создаём шаблон .env для бэкенда..."
  $SUDO mkdir -p "${APP_DIR}/reestr_back"
  $SUDO tee "${ENV_FILE}" > /dev/null <<'EOF'
# ===== ОБЯЗАТЕЛЬНО ЗАПОЛНИТЬ =====
DATABASE_URL=postgres://USER:PASSWORD@HOST:5432/reestrdogovorov
DADATAAPIKEY=your_dadata_api_key
DADATASECRETKEY=your_dadata_secret_key
EOF
  $SUDO chown "${APP_USER}:${APP_USER}" "${ENV_FILE}"
  $SUDO chmod 600 "${ENV_FILE}"
  log "⚠  Заполните ${ENV_FILE} реальными данными перед первым деплоем!"
fi

# -----------------------------------------------------------------------------
# 11. Nginx конфиг приложения
# -----------------------------------------------------------------------------
log "Устанавливаем nginx конфиг..."
$SUDO cp "${REPO_ROOT}/infra/nginx/reestr.conf" "/etc/nginx/sites-available/${APP_NAME}.conf"
$SUDO ln -sf "/etc/nginx/sites-available/${APP_NAME}.conf" "/etc/nginx/sites-enabled/${APP_NAME}.conf"
$SUDO rm -f /etc/nginx/sites-enabled/default
$SUDO nginx -t && $SUDO systemctl reload nginx

# -----------------------------------------------------------------------------
# 12. Каталог для файлов договоров
# -----------------------------------------------------------------------------
log "Создаём каталог для файлов договоров..."
$SUDO mkdir -p "${APP_DIR}/reestr_back/files"
$SUDO chown "${APP_USER}:${APP_USER}" "${APP_DIR}/reestr_back/files"

# -----------------------------------------------------------------------------
# Done
# -----------------------------------------------------------------------------
log ""
log "======================================================"
log " Сервер готов к первому деплою!"
log "======================================================"
log ""
log " Добавьте remote на машине разработчика:"
log "   git remote add prodserver ssh://${APP_USER}@<SERVER_IP>${GIT_BARE_DIR}"
log ""
log " Проверьте и заполните .env:"
log "   ssh ${APP_USER}@<SERVER_IP> 'nano ${ENV_FILE}'"
log ""
log " Сделайте первый деплой:"
log "   git push prodserver master"
log ""
