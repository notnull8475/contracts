alter table contract rename column date to date_from;
alter table contract add column date_to TIMESTAMPTZ;
alter table contract add column contract_period integer;
alter table contract add column created_time TIMESTAMPTZ default now();
alter table contract add column updated_time TIMESTAMPTZ default now();
alter table contract add column actual bool;