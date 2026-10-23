#!/usr/bin/env python3
"""Подбор настоящих реквизитов для организаций с фиктивным ИНН (9000000xxx).

Для каждой такой организации спрашивает DaData (suggest/party) по названию и
пишет CSV с кандидатами на проверку человеком. В базу НИЧЕГО не пишет: совпадение
по названию неоднозначно (одноимённых ООО много), и подставлять ИНН без проверки
опасно — это реестр договоров.

Запуск из reestr_back (нужны DATABASE_URL и DADATAAPIKEY в .env):

    python3 ../infra/tools/suggest_inn.py --limit 50 --out /tmp/inn_suggestions.csv

После проверки CSV реквизиты вносятся через форму организации (кнопка
«Заполнить по ИНН» подтянет остальное из DaData).
"""

import argparse
import csv
import json
import os
import subprocess
import sys
import time
import urllib.request

DADATA_URL = "https://suggestions.dadata.ru/suggestions/api/4_1/rs/suggest/party"


def load_env(path=".env"):
    env = {}
    try:
        with open(path, encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line or line.startswith("#") or "=" not in line:
                    continue
                key, value = line.split("=", 1)
                env[key.strip()] = value.strip()
    except FileNotFoundError:
        pass
    env.update({k: v for k, v in os.environ.items() if k in ("DATABASE_URL", "DADATAAPIKEY")})
    return env


def fetch_orgs(database_url, limit):
    sql = (
        "SELECT id, short_name_with_opf, COALESCE(fact_address, '') "
        "FROM organization WHERE inn BETWEEN 9000000000 AND 9000009999 "
        f"ORDER BY id LIMIT {int(limit)}"
    )
    out = subprocess.run(
        ["psql", database_url, "-At", "-F", "\t", "-c", sql],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    rows = []
    for line in out.splitlines():
        parts = line.split("\t")
        if len(parts) == 3:
            rows.append((int(parts[0]), parts[1], parts[2]))
    return rows


def suggest(api_key, query, count=3):
    body = json.dumps({"query": query, "count": count}).encode("utf-8")
    req = urllib.request.Request(
        DADATA_URL,
        data=body,
        headers={
            "Content-Type": "application/json",
            "Accept": "application/json",
            "Authorization": f"Token {api_key}",
        },
    )
    with urllib.request.urlopen(req, timeout=15) as resp:
        return json.load(resp).get("suggestions", [])


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--limit", type=int, default=100, help="сколько организаций обработать")
    parser.add_argument("--out", default="inn_suggestions.csv", help="куда писать CSV")
    parser.add_argument("--delay", type=float, default=0.2, help="пауза между запросами, с")
    args = parser.parse_args()

    env = load_env()
    database_url = env.get("DATABASE_URL")
    api_key = env.get("DADATAAPIKEY")
    if not database_url or not api_key:
        sys.exit("Нужны DATABASE_URL и DADATAAPIKEY (в .env или окружении)")

    orgs = fetch_orgs(database_url, args.limit)
    with open(args.out, "w", newline="", encoding="utf-8") as fh:
        writer = csv.writer(fh)
        writer.writerow(
            ["org_id", "name_in_registry", "address_in_registry", "candidate_no",
             "candidate_name", "inn", "ogrn", "status", "candidate_address"]
        )
        for org_id, name, address in orgs:
            try:
                candidates = suggest(api_key, name)
            except Exception as exc:  # noqa: BLE001 — отчёт важнее одной ошибки
                writer.writerow([org_id, name, address, "", f"ОШИБКА: {exc}", "", "", "", ""])
                continue
            if not candidates:
                writer.writerow([org_id, name, address, "", "не найдено", "", "", "", ""])
            for n, c in enumerate(candidates, 1):
                data = c.get("data", {})
                writer.writerow(
                    [org_id, name, address, n, c.get("value", ""), data.get("inn", ""),
                     data.get("ogrn", ""), (data.get("state") or {}).get("status", ""),
                     (data.get("address") or {}).get("value", "")]
                )
            time.sleep(args.delay)
    print(f"Обработано организаций: {len(orgs)}, результат: {args.out}")


if __name__ == "__main__":
    main()
