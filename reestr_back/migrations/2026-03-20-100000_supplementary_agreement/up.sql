CREATE TABLE supplementary_agreement (
    id          SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL REFERENCES contract(id) ON DELETE CASCADE,
    number      TEXT,
    date_from   TIMESTAMPTZ,
    description TEXT,
    file_link   TEXT,
    price       DECIMAL(15,2),
    created_at  TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_supp_agreement_contract_id ON supplementary_agreement(contract_id);
