ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS organization_fk CASCADE;
ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS type_of_validity_fk CASCADE;
ALTER TABLE public.contract DROP CONSTRAINT IF EXISTS resp_person_fk CASCADE;
ALTER TABLE public.responsible_person DROP CONSTRAINT IF EXISTS user_fk CASCADE;
DROP TABLE IF EXISTS public.responsible_person CASCADE;
DROP TABLE IF EXISTS public.organization CASCADE;
DROP TABLE IF EXISTS public.dict_type_of_validity CASCADE;
DROP TABLE IF EXISTS public.contract CASCADE;
