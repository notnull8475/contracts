-- Your SQL goes here
alter table organization rename name to short_name_with_opf;
alter table organization rename address to legal_address;
alter table organization add column management_post varchar(256);
alter table organization add column management_name varchar(256);
alter table organization add column ogrn  varchar(256);
alter table organization add column full_name_with_opf  varchar(256);
alter table organization add column opf_full  varchar(256);
alter table organization add column opf_short  varchar(256);