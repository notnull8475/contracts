-- Чистка справочника ответственных лиц после импорта из Excel.
--
-- Два вида мусора:
--   1) опечатки одной и той же фамилии (Газетдиноьв, Гзетдинов, Газктдинов → Газетдинов);
--   2) комментарии, попавшие в колонку ответственного («ссуда», «РАСТОРЖЕНИЕ с 13.04.2026г»,
--      «на Перешли другую платформу», e-mail) — текст переносится в комментарий договора,
--      ответственный снимается.
--
-- Сопоставление сделано по фамилии, имени и отчеству; записи, где совпадение
-- неочевидно (Шарипова Н.В., Газетдинова, Юсупова Ф.С., Чемоданов А.В.), не тронуты.
-- Каждая строка проверяется по точному ФИО, поэтому на БД без таких записей миграция
-- ничего не меняет.

CREATE TEMP TABLE person_merge (variant_id INT PRIMARY KEY, canonical_id INT NOT NULL);

-- Вариант включается в склейку только если его ФИО совпадает с тем, что было при импорте.
INSERT INTO person_merge
SELECT v.id, canon.id
FROM (VALUES
    -- Газетдинов И.И. → id 11 («И Газетдинов И»)
    ('И.И.', 'Газетденов',     '',    'И',  'Газетдинов', 'И'),
    ('',     'ГазетдиновИ.И.', '',    'И',  'Газетдинов', 'И'),
    ('И.И.', 'Газктдинов',     '',    'И',  'Газетдинов', 'И'),
    ('И.И.', 'Гавзетдинов',    '',    'И',  'Газетдинов', 'И'),
    ('И.И.', 'Газетдиноав',    '',    'И',  'Газетдинов', 'И'),
    ('И',    'Газединов',      'И',   'И',  'Газетдинов', 'И'),
    ('И.И.', 'Гзетдинов',      '',    'И',  'Газетдинов', 'И'),
    ('И.И.', 'Газетдиноьв',    '',    'И',  'Газетдинов', 'И'),
    -- Гиззатулин Динар Радикович → id 7
    ('Динар', 'Гизатдинов',   'Радикович', 'Динар', 'Гиззатулин', 'Радикович'),
    ('Д',     'Гизатуллин',   '.Р.',       'Динар', 'Гиззатулин', 'Радикович'),
    ('Д.Р.',  'Гиззатуллин',  '',          'Динар', 'Гиззатулин', 'Радикович'),
    ('Д.Р.',  'Гиззатулдин',  '',          'Динар', 'Гиззатулин', 'Радикович'),
    -- Вареник В.А. → id 6
    ('В.А.', 'Варенк',       '', '', 'Вареник', 'В.А.'),
    ('В.А.', 'Вареникк',     '', '', 'Вареник', 'В.А.'),
    ('',     'ВареникВ.А.',  '', '', 'Вареник', 'В.А.'),
    -- Шакиров Ф.С. → id 16
    ('Ф.С.', 'Шакирпов', '', 'Ф.С.', 'Шакиров', ''),
    -- Еникеев Камиль Шамилевич → id 8
    ('К.Ш.', 'Енгикеев', '', 'Камиль', 'Еникеев', 'Шамилевич'),
    -- Еникеева С.К. → id 13
    ('С.К.', 'Еникева', '', 'С.', 'Еникеева', 'К.')
) AS m(v_first, v_last, v_name, c_first, c_last, c_name)
JOIN responsible_person v
  ON v.firstname = m.v_first AND COALESCE(v.lastname, '') = m.v_last AND v.name = m.v_name
JOIN responsible_person canon
  ON canon.firstname = m.c_first AND COALESCE(canon.lastname, '') = m.c_last AND canon.name = m.c_name
WHERE v.id <> canon.id;

UPDATE contract c SET responsible_person_id = m.canonical_id
FROM person_merge m WHERE c.responsible_person_id = m.variant_id;

DELETE FROM responsible_person WHERE id IN (SELECT variant_id FROM person_merge);

-- Комментарии вместо ответственного: текст сохраняем в договоре, запись удаляем.
CREATE TEMP TABLE person_note (person_id INT PRIMARY KEY, note TEXT NOT NULL);

INSERT INTO person_note
SELECT p.id, m.note
FROM (VALUES
    ('',     'ссуда',              '',                         'ссуда'),
    ('',     'дактелоскопирование','',                         'дактелоскопирование'),
    ('было', 'не',                 'нашла  Наталья Шарипова',  'было: не нашла, Наталья Шарипова'),
    ('',     'РАСТОРЖЕНИЕ',        'с 13.04.2026г',            'РАСТОРЖЕНИЕ с 13.04.2026г'),
    ('на',   'Перешли',            'другую платформу',         'Перешли на другую платформу'),
    ('',     'nela60@mail.ru',     '',                         'nela60@mail.ru')
) AS m(p_first, p_last, p_name, note)
JOIN responsible_person p
  ON p.firstname = m.p_first AND COALESCE(p.lastname, '') = m.p_last AND p.name = m.p_name;

UPDATE contract c
SET comment = NULLIF(CONCAT_WS('; ', NULLIF(c.comment, ''), n.note), ''),
    responsible_person_id = NULL
FROM person_note n WHERE c.responsible_person_id = n.person_id;

DELETE FROM responsible_person WHERE id IN (SELECT person_id FROM person_note);

DROP TABLE person_merge;
DROP TABLE person_note;
