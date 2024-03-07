CREATE TABLE public.user (
    id integer PRIMARY KEY,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    role text DEFAULT 'guest'::text NOT NULL
);

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE ONLY public.user ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);
CREATE UNIQUE INDEX user_email_key ON public.user USING btree (email);
CREATE UNIQUE INDEX user_username_key ON public.user USING btree (username);