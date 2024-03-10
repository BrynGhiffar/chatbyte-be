CREATE TABLE public.user_avatar (
    id integer PRIMARY KEY,
    user_id integer UNIQUE NOT NULL,
    avatar_image bytea NOT NULL
);

CREATE SEQUENCE public.user_avatar_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.user_avatar_id_seq
    OWNED BY public.user_avatar.id;

ALTER TABLE ONLY public.user_avatar alter COLUMN id SET DEFAULT nextval('public.user_avatar_id_seq'::regclass);

ALTER TABLE ONLY public.user_avatar
    ADD CONSTRAINT user_avatar_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;
