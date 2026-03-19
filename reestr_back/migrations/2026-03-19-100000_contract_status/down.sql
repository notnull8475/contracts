-- Возвращаем actual, переносим данные обратно, удаляем contract_status_id
ALTER TABLE contract ADD COLUMN actual BOOLEAN;
UPDATE contract SET actual = true
    WHERE contract_status_id IN (SELECT id FROM dict_contract_status WHERE name NOT IN ('Расторгнут'));
UPDATE contract SET actual = false
    WHERE contract_status_id IN (SELECT id FROM dict_contract_status WHERE name = 'Расторгнут');
ALTER TABLE contract DROP COLUMN contract_status_id;
DROP TABLE dict_contract_status;
