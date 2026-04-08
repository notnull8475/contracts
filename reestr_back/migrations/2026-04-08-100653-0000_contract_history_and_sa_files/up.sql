-- Add supplementary_agreement_id to contract_files for file attachments on supplementary agreements
ALTER TABLE contract_files
  ADD COLUMN supplementary_agreement_id INTEGER REFERENCES supplementary_agreement(id) ON DELETE CASCADE;

CREATE INDEX idx_contract_files_sa_id ON contract_files(supplementary_agreement_id) WHERE supplementary_agreement_id IS NOT NULL;

-- Contract history / audit log
CREATE TABLE contract_history (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL REFERENCES contract(id) ON DELETE CASCADE,
    action TEXT NOT NULL,
    old_value TEXT,
    new_value TEXT,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_contract_history_contract_id ON contract_history(contract_id);
CREATE INDEX idx_contract_history_created_at ON contract_history(created_at DESC);
