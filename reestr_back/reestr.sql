--
-- PostgreSQL database dump
--

\restrict NjSoXP4qxfWF8FvIlgQeF8JjGiXNJgEojzXDaD3RsN8Zz3Dc2PVPSIydYTyMBkZ

-- Dumped from database version 16.6
-- Dumped by pg_dump version 17.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: reestrdogovorov
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO reestrdogovorov;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: reestrdogovorov
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO reestrdogovorov;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO reestrdogovorov;

--
-- Name: contract; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.contract (
    id integer NOT NULL,
    number text NOT NULL,
    date_from timestamp without time zone,
    organization_id integer NOT NULL,
    type_of_validity integer,
    responsible_person_id integer,
    address text,
    additional_agreement text,
    comment text,
    date_to timestamp with time zone,
    contract_period integer,
    created_time timestamp with time zone DEFAULT now(),
    updated_time timestamp with time zone DEFAULT now(),
    actual boolean
);


ALTER TABLE public.contract OWNER TO reestrdogovorov;

--
-- Name: contract_id_seq; Type: SEQUENCE; Schema: public; Owner: reestrdogovorov
--

CREATE SEQUENCE public.contract_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.contract_id_seq OWNER TO reestrdogovorov;

--
-- Name: contract_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: reestrdogovorov
--

ALTER SEQUENCE public.contract_id_seq OWNED BY public.contract.id;


--
-- Name: dict_type_of_validity; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.dict_type_of_validity (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public.dict_type_of_validity OWNER TO reestrdogovorov;

--
-- Name: dict_type_of_validity_id_seq; Type: SEQUENCE; Schema: public; Owner: reestrdogovorov
--

CREATE SEQUENCE public.dict_type_of_validity_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.dict_type_of_validity_id_seq OWNER TO reestrdogovorov;

--
-- Name: dict_type_of_validity_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: reestrdogovorov
--

ALTER SEQUENCE public.dict_type_of_validity_id_seq OWNED BY public.dict_type_of_validity.id;


--
-- Name: organization; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.organization (
    id integer NOT NULL,
    short_name_with_opf text NOT NULL,
    inn bigint NOT NULL,
    fact_address text,
    legal_address text,
    management_post character varying(256),
    management_name character varying(256),
    ogrn character varying(256),
    full_name_with_opf character varying(256),
    opf_full character varying(256),
    opf_short character varying(256)
);


ALTER TABLE public.organization OWNER TO reestrdogovorov;

--
-- Name: organization_id_seq; Type: SEQUENCE; Schema: public; Owner: reestrdogovorov
--

CREATE SEQUENCE public.organization_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.organization_id_seq OWNER TO reestrdogovorov;

--
-- Name: organization_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: reestrdogovorov
--

ALTER SEQUENCE public.organization_id_seq OWNED BY public.organization.id;


--
-- Name: responsible_person; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.responsible_person (
    id integer NOT NULL,
    firstname text NOT NULL,
    name text NOT NULL,
    lastname text,
    user_id integer
);


ALTER TABLE public.responsible_person OWNER TO reestrdogovorov;

--
-- Name: responsible_person_id_seq; Type: SEQUENCE; Schema: public; Owner: reestrdogovorov
--

CREATE SEQUENCE public.responsible_person_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.responsible_person_id_seq OWNER TO reestrdogovorov;

--
-- Name: responsible_person_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: reestrdogovorov
--

ALTER SEQUENCE public.responsible_person_id_seq OWNED BY public.responsible_person.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: reestrdogovorov
--

CREATE TABLE public.users (
    id integer NOT NULL,
    login character varying(50) NOT NULL,
    username character varying(50) NOT NULL,
    password_hash character varying(255) NOT NULL,
    role character varying(20) NOT NULL,
    is_active boolean DEFAULT true,
    CONSTRAINT users_role_check CHECK (((role)::text = ANY ((ARRAY['admin'::character varying, 'moderator'::character varying, 'user'::character varying])::text[])))
);


ALTER TABLE public.users OWNER TO reestrdogovorov;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: reestrdogovorov
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO reestrdogovorov;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: reestrdogovorov
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: contract id; Type: DEFAULT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.contract ALTER COLUMN id SET DEFAULT nextval('public.contract_id_seq'::regclass);


--
-- Name: dict_type_of_validity id; Type: DEFAULT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.dict_type_of_validity ALTER COLUMN id SET DEFAULT nextval('public.dict_type_of_validity_id_seq'::regclass);


--
-- Name: organization id; Type: DEFAULT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.organization ALTER COLUMN id SET DEFAULT nextval('public.organization_id_seq'::regclass);


--
-- Name: responsible_person id; Type: DEFAULT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.responsible_person ALTER COLUMN id SET DEFAULT nextval('public.responsible_person_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: contract contract_pk; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.contract
    ADD CONSTRAINT contract_pk PRIMARY KEY (id);


--
-- Name: dict_type_of_validity dict_type_of_validity_pk; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.dict_type_of_validity
    ADD CONSTRAINT dict_type_of_validity_pk PRIMARY KEY (id);


--
-- Name: organization organization_pk; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.organization
    ADD CONSTRAINT organization_pk PRIMARY KEY (id);


--
-- Name: responsible_person responsible_person_pk; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.responsible_person
    ADD CONSTRAINT responsible_person_pk PRIMARY KEY (id);


--
-- Name: users users_login_key; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_login_key UNIQUE (login);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: contract organization_fk; Type: FK CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.contract
    ADD CONSTRAINT organization_fk FOREIGN KEY (organization_id) REFERENCES public.organization(id);


--
-- Name: contract resp_person_fk; Type: FK CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.contract
    ADD CONSTRAINT resp_person_fk FOREIGN KEY (responsible_person_id) REFERENCES public.responsible_person(id);


--
-- Name: contract type_of_validity_fk; Type: FK CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.contract
    ADD CONSTRAINT type_of_validity_fk FOREIGN KEY (type_of_validity) REFERENCES public.dict_type_of_validity(id);


--
-- Name: responsible_person user_fk; Type: FK CONSTRAINT; Schema: public; Owner: reestrdogovorov
--

ALTER TABLE ONLY public.responsible_person
    ADD CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- PostgreSQL database dump complete
--

\unrestrict NjSoXP4qxfWF8FvIlgQeF8JjGiXNJgEojzXDaD3RsN8Zz3Dc2PVPSIydYTyMBkZ

