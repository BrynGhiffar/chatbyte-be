
CREATE TABLE public.session (
    id integer PRIMARY KEY,
    created_at timestamp(3) without time zone default CURRENT_TIMESTAMP NOT NULL,
    last_active timestamp(3) without time zone default CURRENT_TIMESTAMP NOT NULL,
    operating_system text CONSTRAINT os_chk CHECK (char_length(operating_system) >= 2),
    agent text CONSTRAINT agent_chk CHECK (char_length(agent) >= 2),
    user_id integer NOT NULL,
    CONSTRAINT fk_session_user_id FOREIGN KEY (user_id) REFERENCES public.user(id)
);

CREATE SEQUENCE public.session_id_seq 
    AS INTEGER
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
    ;

ALTER TABLE ONLY public.session ALTER COLUMN id SET DEFAULT nextval('public.session_id_seq'::regclass);
