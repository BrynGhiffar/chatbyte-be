DROP VIEW IF EXISTS public.unread_message_content;
DROP VIEW IF EXISTS public.unread_message_count;
-- DROP VIEW IF EXISTS public.unread_message;
DROP VIEW IF EXISTS public.last_message;
DROP VIEW IF EXISTS public.message_sender;
DROP TABLE IF EXISTS public.user_avatar;
DROP TABLE IF EXISTS public.blog;
DROP TABLE IF EXISTS public.message;
DROP TABLE IF EXISTS public.user;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE public.blog (
    id text PRIMARY KEY,
    author_id integer NOT NULL,
    title text NOT NULL,
    content text NOT NULL,
    created_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    summary text NOT NULL,
    updated_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    thumbnail text
);

ALTER TABLE public.blog OWNER TO postgres;

CREATE TABLE public.message (
    id integer PRIMARY KEY,
    sent_at timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    sender_id integer NOT NULL,
    receiver_id integer NOT NULL,
    content text NOT NULL,
    read boolean DEFAULT false NOT NULL
);

ALTER TABLE public.message OWNER TO postgres;

CREATE SEQUENCE public.message_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.message_id_seq OWNER TO postgres;
ALTER SEQUENCE public.message_id_seq OWNED BY public.message.id;

CREATE TABLE public.user (
    id integer PRIMARY KEY,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    role text DEFAULT 'guest'::text NOT NULL
);

ALTER TABLE public.user OWNER TO postgres;

CREATE TABLE public.user_avatar (
    user_id integer,
    avatar_image bytea
);

ALTER TABLE public.user OWNER TO postgres;

ALTER TABLE ONLY public.user_avatar
    ADD CONSTRAINT user_avatar_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.user_id_seq OWNER TO postgres;

ALTER SEQUENCE public.user_id_seq OWNED BY public.user.id;

ALTER TABLE ONLY public.message ALTER COLUMN id SET DEFAULT nextval('public.message_id_seq'::regclass);

ALTER TABLE ONLY public.user ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);

CREATE UNIQUE INDEX user_email_key ON public.user USING btree (email);
CREATE UNIQUE INDEX user_username_key ON public.user USING btree (username);

ALTER TABLE ONLY public.blog
    ADD CONSTRAINT blog_author_id_fkey FOREIGN KEY (author_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_receiver_id_fkey FOREIGN KEY (receiver_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;

ALTER TABLE ONLY public.message
    ADD CONSTRAINT message_sender_id_fkey FOREIGN KEY (sender_id) REFERENCES public.user(id) ON UPDATE CASCADE ON DELETE RESTRICT;

CREATE VIEW message_sender as 
    select
        message.id,
        message.sent_at,
        message.read,
        message.content,
        case when message.sender_id < message.receiver_id then message.sender_id else message.receiver_id end as min_sender,
        case when message.sender_id < Message.receiver_id then message.receiver_id else message.sender_id end as max_sender
    from message;

CREATE VIEW last_message as 
    select
        max(id) as last_msg_id, 
        min_sender, 
        max_sender 
    from message_sender
    -- where read = false 
    group by min_sender, max_sender;

CREATE VIEW unread_message_count as
    select count(*) as unread_count, receiver_id, sender_id from message where read = false group by receiver_id, sender_id;

CREATE VIEW unread_message_content as
    select
        message.id,
        message.receiver_id,
        message.sender_id,
        message.sent_at,
        (case when unread_message_count.unread_count is null then 0 else unread_message_count.unread_count end) as unread_count,
        message.content as last_message
    from message
    join last_message on message.id = last_message.last_msg_id
    left join unread_message_count on message.sender_id = unread_message_count.sender_id and message.receiver_id = unread_message_count.receiver_id
    ;

-- Mock Users
INSERT INTO public.user (username, email, password)
    VALUES ('Bryn Ghiffar', 'bryn.ghiffar@gmail.com', crypt('bryn.ghiffar@gmail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('jack@mail.com', 'jack@mail.com', crypt('jack@mail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('Gato', 'cat@mail.com', crypt('cat@mail.com', gen_salt('bf', 5)));