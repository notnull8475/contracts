CREATE TABLE public.users
(
    id            SERIAL PRIMARY KEY,
    login         VARCHAR(50) UNIQUE NOT NULL,
    username      VARCHAR(50)        NOT NULL,
    password_hash VARCHAR(255)       NOT NULL, -- Храните только хеш пароля!
    role          VARCHAR(20)        NOT NULL CHECK (role IN ('admin', 'moderator', 'user')),
    is_active     BOOLEAN DEFAULT TRUE
);