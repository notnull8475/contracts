# Деплой Реестра договоров

Схема: `git push prodserver master` → автоматическая сборка и перезапуск на сервере.

```
[Машина разработчика]
  git push prodserver master
          │
          ▼
[Сервер: /opt/reestrdogovorov.git — bare repo]
  └── hooks/post-receive
          │
          ├── 1. git checkout → /opt/reestrdogovorov
          ├── 2. npm ci && npm run build
          ├── 3. rsync dist/ → /var/www/reestrdogovorov
          ├── 4. cargo build --release
          ├── 5. diesel migration run
          └── 6. systemctl restart reestr-back

[nginx на сервере]
  ├── /api/ → 127.0.0.1:8080  (Actix-web)
  └── /     → /var/www/reestrdogovorov  (Vue dist)

[Внешний nginx-прокси]
  └── reestr.domain.local → SERVER_IP:80
```

---

## Первичная настройка сервера (один раз)

### 1. Настройте SSH-доступ к серверу

На сервере создайте пользователя `deploy` (если нет):
```bash
sudo useradd -m -s /bin/bash deploy
sudo mkdir -p /home/deploy/.ssh
# Добавьте свой публичный ключ
sudo bash -c 'cat >> /home/deploy/.ssh/authorized_keys' < ~/.ssh/id_rsa.pub
sudo chown -R deploy:deploy /home/deploy/.ssh
sudo chmod 700 /home/deploy/.ssh
sudo chmod 600 /home/deploy/.ssh/authorized_keys
```

Разрешите пользователю `deploy` перезапускать systemd-сервис без пароля:
```bash
sudo bash -c 'echo "deploy ALL=(ALL) NOPASSWD: /usr/bin/systemctl restart reestr-back, /usr/bin/systemctl start reestr-back, /usr/bin/systemctl stop reestr-back, /usr/bin/systemctl is-active reestr-back" > /etc/sudoers.d/deploy-reestr'
```

### 2. Запустите скрипт инициализации

С машины разработчика (из корня репозитория):
```bash
ssh deploy@<SERVER_IP> "bash -s" < infra/setup/server_setup.sh
```

> Скрипт установит: `build-essential`, `libpq-dev`, `libssl-dev`, `nginx`, `Node.js LTS`, `Rust`, `diesel_cli`

### 3. Заполните `.env` на сервере

```bash
ssh deploy@<SERVER_IP> "nano /opt/reestrdogovorov/reestr_back/.env"
```

Содержимое:
```env
DATABASE_URL=postgres://USER:PASSWORD@DB_HOST:5432/reestrdogovorov
DADATAAPIKEY=ваш_ключ_dadata
DADATASECRETKEY=ваш_секрет_dadata
```

### 4. Настройте nginx на сервере

Отредактируйте `server_name` в конфиге:
```bash
ssh deploy@<SERVER_IP> "sudo nano /etc/nginx/sites-available/reestrdogovorov.conf"
# Замените reestr.your-domain.local на реальное имя
sudo nginx -t && sudo systemctl reload nginx
```

### 5. Добавьте remote на машине разработчика

```bash
git remote add prodserver ssh://deploy@<SERVER_IP>/opt/reestrdogovorov.git
```

Проверьте подключение:
```bash
ssh deploy@<SERVER_IP> "echo ok"
```

---

## Деплой

Стандартный деплой:
```bash
git push prodserver master
```

Деплой текущей ветки в master:
```bash
git push prodserver HEAD:master
```

---

## Просмотр логов на сервере

```bash
# Логи backend-сервиса
ssh deploy@<SERVER_IP> "journalctl -u reestr-back -n 50 -f"

# Статус сервиса
ssh deploy@<SERVER_IP> "systemctl status reestr-back"

# Логи nginx
ssh deploy@<SERVER_IP> "sudo tail -f /var/log/nginx/reestrdogovorov.error.log"
```

---

## Ручное управление сервисом

```bash
# Перезапуск
ssh deploy@<SERVER_IP> "sudo systemctl restart reestr-back"

# Остановка
ssh deploy@<SERVER_IP> "sudo systemctl stop reestr-back"

# Статус
ssh deploy@<SERVER_IP> "systemctl status reestr-back"
```

---

## Настройка внешнего nginx-прокси

На вашем прокси-сервере добавьте виртуальный хост:
```nginx
server {
    listen 80;
    server_name reestr.your-domain.local;

    location / {
        proxy_pass         http://<APP_SERVER_IP>:80;
        proxy_http_version 1.1;
        proxy_set_header   Host              $host;
        proxy_set_header   X-Real-IP         $remote_addr;
        proxy_set_header   X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header   X-Forwarded-Proto $scheme;
    }
}
```

---

## Структура файлов infra/

```
infra/
├── README.md                    ← этот файл
├── setup/
│   └── server_setup.sh          ← разовая инициализация сервера
├── deploy/
│   └── post-receive             ← git-хук (копируется в bare repo автоматически)
├── systemd/
│   └── reestr-back.service      ← systemd unit для backend
└── nginx/
    └── reestr.conf              ← nginx конфиг приложения
```

---

## Переменные окружения

| Переменная | Описание |
|------------|----------|
| `DATABASE_URL` | Строка подключения к PostgreSQL |
| `DADATAAPIKEY` | API-ключ Dadata (для автозаполнения по ИНН) |
| `DADATASECRETKEY` | Секретный ключ Dadata |

> `.env` файл создаётся скриптом `server_setup.sh` как шаблон.  
> Никогда не коммитьте `.env` с реальными данными в репозиторий.
