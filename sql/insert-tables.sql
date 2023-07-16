DROP VIEW IF EXISTS public.unread_message_content;
DROP VIEW IF EXISTS public.unread_message_count;
DROP VIEW IF EXISTS public.unread_message;
DROP VIEW IF EXISTS public.last_message;
DROP VIEW IF EXISTS public.message_sender;
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

COPY public.message (sent_at, sender_id, receiver_id, content, read) FROM stdin;
2023-06-25 18:42:39.998	3	2	three two one	t
2023-06-25 14:34:00.301	3	2	Hi jack.	t
2023-06-18 21:24:27.366	3	2	Hiyaaa	t
2023-06-18 21:22:30.313	3	2	yooo.	t
2023-06-18 21:22:50.271	3	2	test123	t
2023-06-18 21:19:04.262	3	2	It kinda doesn't work?	t
2023-06-07 14:33:59.343	3	2	It looks like the server is working it's thing.	t
2023-06-07 14:45:24.758	3	2	Hi, person.	t
2023-06-07 14:46:25.835	3	2	Ooooh coool.	t
2023-06-17 10:56:25.392	3	2	Hello Jack	t
2023-06-18 00:43:50.422	1	2	Hi Jack.	t
2023-06-18 00:44:28.661	1	2	How's it going?	t
2023-06-17 11:05:51.722	3	2	Hello Jack	t
2023-06-18 20:48:06.561	2	1	Hi dude.	f
2023-06-18 20:48:46.538	2	1	bbbb	f
2023-06-18 20:49:39.815	2	1	three	f
2023-06-18 20:50:07.914	2	1	asdfasdf	f
2023-06-18 20:51:03.671	2	1	Hello World	f
2023-06-18 20:51:25.59	2	1	Hello Again.	f
2023-06-18 20:52:19.347	2	1	test 123	f
2023-06-18 20:54:14.881	2	1	asdfas	f
2023-06-18 20:57:02.252	2	1	hello again?	f
2023-06-18 20:57:51.469	2	1	Is that true?	f
2023-06-17 18:16:08.531	3	2	Hello Jack	t
2023-06-17 18:51:44.337	3	2	Ping!!!!	t
2023-06-17 19:00:37.149	3	2	Looks like the auto scroll is working.	t
2023-06-17 19:02:00.767	3	2	Yess...	t
2023-06-25 18:50:03.456	2	3	ewfasdfasdf	t
2023-06-25 18:48:07.861	2	3	grgsf	t
2023-06-18 21:17:27.387	3	1	Hi jack.	f
2023-06-25 18:43:37.241	2	3	One two three.	t
2023-06-25 18:43:57.955	2	3	hello2	t
2023-06-25 18:42:09.863	2	3	It iz the update.	t
2023-06-25 14:35:59.468	2	3	change	t
2023-06-25 14:33:06.418	2	3	Hiyaa cat.	t
2023-06-18 21:26:38.019	2	3	It's pretty cool, I like it.	t
2023-06-18 21:16:41.31	2	3	hi cat.	t
2023-06-18 21:17:12.348	2	3	hi cat...	t
2023-06-18 20:52:41.266	2	3	does it work?	t
2023-06-18 20:55:43.399	2	3	testt	t
2023-06-18 20:57:09.632	2	3	cattooo	t
2023-06-18 21:02:18.514	2	3	Ola cat.	t
2023-06-18 00:44:09.46	2	1	Hi Bryn.	f
2023-06-25 22:03:05.62	3	2	rgafasdf	t
2023-06-07 14:33:38.457	3	2	Hi jack.	t
2023-06-18 21:00:04.921	3	2	Yes Jack?	t
2023-06-18 21:02:12.075	3	2	Ola jackk!	t
2023-06-18 21:02:37.193	3	2	asdasdf	t
2023-06-18 21:02:37.853	3	2	asdf	t
2023-06-18 21:02:38.594	3	2	asdf	t
2023-06-18 21:02:39.194	3	2	asdf	t
2023-06-18 21:14:10.418	3	2	hello jack.	t
2023-06-18 21:18:03.565	3	2	hi jack..	t
2023-06-18 21:27:03.517	3	2	This is another test message.	t
2023-06-18 21:31:07.364	3	2	This is a pretty cool, chat application no?	t
2023-06-25 14:36:53.466	3	2	what an update.	t
2023-06-25 14:37:24.143	3	2	What an updatee it iz.	t
2023-06-25 18:53:53.788	3	2	Hi cat.	t
2023-06-25 22:02:59.561	3	2	How are you doing?	t
2023-06-07 14:34:08.343	2	3	Yeah, pretty awesome.	t
2023-06-07 14:35:32.233	2	3	Agreed.	t
2023-06-07 14:46:18.595	2	3	Now, you cannot send an empty message.	t
2023-06-17 11:02:48.596	2	3	Hello Cat	t
2023-06-17 18:07:55.97	2	3	Hello Cat	t
2023-06-17 18:15:56.323	2	3	Hello Cat	t
2023-06-17 18:51:20.602	2	3	test	t
2023-06-17 18:51:51.641	2	3	Ponggg!!!!	t
2023-06-17 19:00:01.313	2	3	Hello	t
2023-06-25 22:01:54.223	2	3	test	t
2023-06-17 19:00:53.87	2	3	Appears so.	t
2023-06-17 19:58:35.382	2	3	Hello world	t
2023-06-17 19:58:44.781	2	3	it works	t
2023-06-07 14:33:45.544	2	3	Well, hi cat.	t
2023-06-18 21:31:18.424	2	3	Yea, I guess so : ))	t
2023-06-25 14:34:49.831	2	3	fsdfg	t
2023-06-25 14:37:14.405	2	3	Yes, I know.	t
2023-06-25 18:45:21.59	2	3	asdfasvh	t
2023-06-25 18:46:00.788	2	3	hgfhgn	t
2023-06-25 18:53:18.966	2	3	test2	t
2023-06-25 22:00:37.991	2	3	test	t
2023-06-25 22:00:47.967	2	3	test again.	t
2023-06-25 22:01:14.585	2	3	hello	t
2023-06-25 22:02:19.342	2	3	asdf	t
2023-06-25 22:02:25.782	2	3	This is it.	t
2023-06-25 22:02:44.801	2	3	I love cakess.	t
2023-06-25 22:02:50.54	2	3	Yes, it's pretty fund.	t
2023-06-25 22:02:52.58	2	3	asdkfhsdf	t
\.
;
