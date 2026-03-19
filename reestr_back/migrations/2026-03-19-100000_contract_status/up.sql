-- 1. Таблица статусов договора
CREATE TABLE dict_contract_status (
    id    SERIAL PRIMARY KEY,
    name  TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#607d8b'
);

-- 2. Начальные статусы
INSERT INTO dict_contract_status (name, color) VALUES
    ('Подготовлен', '#9e9e9e'),
    ('Отправлен',   '#1976d2'),
    ('Подписан',    '#2e7d32'),
    ('Расторгнут',  '#c62828');

-- 3. Добавляем новую колонку (nullable, FK → dict_contract_status, SET NULL при удалении)
ALTER TABLE contract
    ADD COLUMN contract_status_id INTEGER
        REFERENCES dict_contract_status(id) ON DELETE SET NULL;

-- 4. Переносим данные из actual
UPDATE contract SET contract_status_id = 3 WHERE actual = true;
UPDATE contract SET contract_status_id = 4 WHERE actual = false;

-- 5. Удаляем старое поле actual
ALTER TABLE contract DROP COLUMN actual;
