-- ** Database generated with pgModeler (PostgreSQL Database Modeler).
-- ** pgModeler version: 1.2.0
-- ** PostgreSQL version: 16.0
-- ** Project Site: pgmodeler.io
-- ** Model Author: ---

-- ** Database creation must be performed outside a multi lined SQL file. 
-- ** These commands were put in this file only as a convenience.

-- object: new_database | type: DATABASE --
-- DROP DATABASE IF EXISTS new_database;
CREATE DATABASE new_database;
-- ddl-end --


SET search_path TO pg_catalog,public;
-- ddl-end --

-- object: public.users | type: TABLE --
-- DROP TABLE IF EXISTS public.users CASCADE;
CREATE TABLE public.users (
	id serial NOT NULL,
	name text NOT NULL,
	login text NOT NULL,
	password text NOT NULL,
	CONSTRAINT users_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.users OWNER TO postgres;
-- ddl-end --

-- object: public.app_role | type: TABLE --
-- DROP TABLE IF EXISTS public.app_role CASCADE;
CREATE TABLE public.app_role (
	id serial NOT NULL,
	name text NOT NULL,
	CONSTRAINT app_role_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.app_role OWNER TO postgres;
-- ddl-end --

-- object: public.user_roles | type: TABLE --
-- DROP TABLE IF EXISTS public.user_roles CASCADE;
CREATE TABLE public.user_roles (
	user_id integer,
	app_role_id integer

);
-- ddl-end --
ALTER TABLE public.user_roles OWNER TO postgres;
-- ddl-end --

-- object: public.contract | type: TABLE --
-- DROP TABLE IF EXISTS public.contract CASCADE;
CREATE TABLE public.contract (
	id serial NOT NULL,
	number text,
	date timestamp,
	organization_id integer,
	type_of_validity integer,
	responsible_person_id integer,
	address text,
	additional_agreement text,
	comment text,
	CONSTRAINT contract_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.contract OWNER TO postgres;
-- ddl-end --

-- object: public.organization | type: TABLE --
-- DROP TABLE IF EXISTS public.organization CASCADE;
CREATE TABLE public.organization (
	id serial NOT NULL,
	name text,
	inn bigint,
	fact_address text,
	address text,
	CONSTRAINT organization_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.organization OWNER TO postgres;
-- ddl-end --

-- object: public.dict_type_of_validity | type: TABLE --
-- DROP TABLE IF EXISTS public.dict_type_of_validity CASCADE;
CREATE TABLE public.dict_type_of_validity (
	id serial NOT NULL,
	name text,
	CONSTRAINT dict_type_of_validity_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.dict_type_of_validity OWNER TO postgres;
-- ddl-end --

-- object: public.responsible_person | type: TABLE --
-- DROP TABLE IF EXISTS public.responsible_person CASCADE;
CREATE TABLE public.responsible_person (
	id serial NOT NULL,
	firstname text,
	name text,
	lastname text,
	user_id smallint,
	CONSTRAINT responsible_person_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.responsible_person OWNER TO postgres;
-- ddl-end --

-- object: user_id | type: CONSTRAINT --
-- ALTER TABLE public.user_roles DROP CONSTRAINT IF EXISTS user_id CASCADE;
ALTER TABLE public.user_roles ADD CONSTRAINT user_id FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: app_role_id | type: CONSTRAINT --
-- ALTER TABLE public.user_roles DROP CONSTRAINT IF EXISTS app_role_id CASCADE;
ALTER TABLE public.user_roles ADD CONSTRAINT app_role_id FOREIGN KEY (app_role_id)
REFERENCES public.app_role (id) MATCH SIMPLE
ON DELETE CASCADE ON UPDATE CASCADE;
-- ddl-end --

-- object: organization_fk | type: CONSTRAINT --
-- ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS organization_fk CASCADE;
ALTER TABLE public.contract ADD CONSTRAINT organization_fk FOREIGN KEY (organization_id)
REFERENCES public.organization (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: type_of_validity_fk | type: CONSTRAINT --
-- ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS type_of_validity_fk CASCADE;
ALTER TABLE public.contract ADD CONSTRAINT type_of_validity_fk FOREIGN KEY (type_of_validity)
REFERENCES public.dict_type_of_validity (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: resp_person_fk | type: CONSTRAINT --
-- ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS resp_person_fk CASCADE;
ALTER TABLE public.contract ADD CONSTRAINT resp_person_fk FOREIGN KEY (responsible_person_id)
REFERENCES public.responsible_person (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_fk | type: CONSTRAINT --
-- ALTER TABLE public.responsible_person DROP CONSTRAINT IF EXISTS user_fk CASCADE;
ALTER TABLE public.responsible_person ADD CONSTRAINT user_fk FOREIGN KEY (user_id)
REFERENCES public.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --


