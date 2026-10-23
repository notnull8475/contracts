-- Возврат трёх договоров на организацию-заглушку «Не указано».

UPDATE contract c
SET organization_id = placeholder.id
FROM organization placeholder, organization o
WHERE placeholder.short_name_with_opf ILIKE 'не указано%'
  AND o.id = c.organization_id
  AND (
        (c.number = '55/15Г'  AND o.short_name_with_opf = 'ООО " Коршай"')
     OR (c.number = '59/1-15' AND o.short_name_with_opf = 'ООО "ГЕМОНТ"')
     OR (c.number = '3159262' AND o.short_name_with_opf = 'ОАО "Российские железные дороги"')
  );
