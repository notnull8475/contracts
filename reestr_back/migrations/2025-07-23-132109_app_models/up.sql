CREATE TABLE public.contract
(
    id                    serial  NOT NULL,
    number                text    not null,
    date                  timestamp,
    organization_id       integer not null,
    type_of_validity      integer,
    responsible_person_id integer,
    address               text,
    additional_agreement  text,
    comment               text,
    CONSTRAINT contract_pk PRIMARY KEY (id)
);
CREATE TABLE public.organization
(
    id           serial NOT NULL,
    name         text   not null,
    inn          bigint not null,
    fact_address text,
    address      text,
    CONSTRAINT organization_pk PRIMARY KEY (id)
);
CREATE TABLE public.dict_type_of_validity
(
    id   serial NOT NULL,
    name text   not null,
    CONSTRAINT dict_type_of_validity_pk PRIMARY KEY (id)
);
CREATE TABLE public.responsible_person
(
    id        serial NOT NULL,
    firstname text   not null,
    name      text   not null,
    lastname  text,
    user_id   int4,
    CONSTRAINT responsible_person_pk PRIMARY KEY (id)
);
ALTER TABLE public.contract
    ADD CONSTRAINT organization_fk FOREIGN KEY (organization_id)
        REFERENCES public.organization (id) MATCH SIMPLE
        ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE public.contract
    ADD CONSTRAINT type_of_validity_fk FOREIGN KEY (type_of_validity)
        REFERENCES public.dict_type_of_validity (id) MATCH SIMPLE
        ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE public.contract
    ADD CONSTRAINT resp_person_fk FOREIGN KEY (responsible_person_id)
        REFERENCES public.responsible_person (id) MATCH SIMPLE
        ON DELETE NO ACTION ON UPDATE NO ACTION;
ALTER TABLE public.responsible_person
    ADD CONSTRAINT user_fk FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON DELETE NO ACTION ON UPDATE NO ACTION;