CREATE TABLE public.message (
    id integer PRIMARY KEY,
    sent_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    sender_id integer NOT NULL,
    receiver_id integer NOT NULL,
    content text NOT NULL,
    read boolean DEFAULT false NOT NULL
);

CREATE SEQUENCE public.message_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE ONLY public.message ALTER COLUMN id SET DEFAULT nextval('public.message_id_seq'::regclass);

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_receiver_id_fkey FOREIGN KEY (receiver_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_sender_id_fkey FOREIGN KEY (sender_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;