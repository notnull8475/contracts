CREATE TABLE contract_files (
    id SERIAL PRIMARY KEY,
    contract_fk INTEGER NOT NULL REFERENCES contract(id) ON DELETE CASCADE,
    file_name TEXT NOT NULL,
    orig_name TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    mime_type_txt TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_contract_files_contract_fk ON contract_files(contract_fk);
