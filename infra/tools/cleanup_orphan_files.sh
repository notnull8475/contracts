#!/bin/bash
# =============================================================================
# Удаление файлов-сирот: *.bin в reestr_back/files без записи в contract_files.
# =============================================================================
# По умолчанию только показывает список. С --apply архивирует сирот в
# ~/orphan-files-<дата>.tar.gz и удаляет их.
# Файлы моложе суток не трогаются — это может быть загрузка в процессе.
#
#   ssh reestrdocdeploy 'bash -s' < infra/tools/cleanup_orphan_files.sh
#   ssh reestrdocdeploy 'bash -s -- --apply' < infra/tools/cleanup_orphan_files.sh
# =============================================================================
set -euo pipefail

APP_BACK_DIR="/opt/reestrdogovorov/reestr_back"
APPLY=0
[[ "${1:-}" == "--apply" ]] && APPLY=1

cd "${APP_BACK_DIR}"
set -a; . ./.env; set +a

TMP=$(mktemp -d)
trap 'rm -rf "${TMP}"' EXIT

psql "${DATABASE_URL}" -v ON_ERROR_STOP=1 -At -c "select file_name from contract_files" \
  | sort > "${TMP}/db"
# Пустой ответ БД означал бы «все файлы — сироты»: в таком случае ничего не делаем.
[[ -s "${TMP}/db" ]] || { echo "БД вернула пустой список файлов — остановка" >&2; exit 1; }

cd files
find . -maxdepth 1 -name '*.bin' -mtime +1 -printf '%f\n' | sort > "${TMP}/disk"
comm -13 "${TMP}/db" "${TMP}/disk" > "${TMP}/orphans"

COUNT=$(wc -l < "${TMP}/orphans")
echo "Записей в БД: $(wc -l < "${TMP}/db"), сирот: ${COUNT}"
[[ "${COUNT}" -eq 0 ]] && exit 0

xargs -a "${TMP}/orphans" ls -l --time-style=+%F | awk '{print $6, $5, $7}'

if [[ "${APPLY}" -eq 0 ]]; then
  echo "Пробный запуск. Для удаления: --apply"
  exit 0
fi

ARCHIVE="${HOME}/orphan-files-$(date +%F).tar.gz"
tar czf "${ARCHIVE}" -T "${TMP}/orphans"
echo "Архив: ${ARCHIVE} ($(du -h "${ARCHIVE}" | cut -f1))"
xargs -a "${TMP}/orphans" rm -f
echo "Удалено файлов: ${COUNT}"
