CREATE TABLE dict_pricelist (
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL,
    price      DECIMAL(15,2) NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO dict_pricelist (name, price) VALUES
    ('Базовое', 30000.00),
    ('Стандартное', 50000.00),
    ('Расширенное', 80000.00);

ALTER TABLE contract ADD COLUMN price DECIMAL(15,2);
ALTER TABLE contract ADD COLUMN pricelist_id INTEGER
    REFERENCES dict_pricelist(id) ON DELETE SET NULL;
