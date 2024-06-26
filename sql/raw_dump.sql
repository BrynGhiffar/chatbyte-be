--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1 (Ubuntu 15.1-1.pgdg20.04+1)
-- Dumped by pg_dump version 15.3

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: Blog; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Blog" (
    id text NOT NULL,
    "authorId" integer NOT NULL,
    title text NOT NULL,
    content text NOT NULL,
    "createdAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    summary text NOT NULL,
    "updatedAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    thumbnail text DEFAULT '/thumbnails/everest.jpg'::text NOT NULL
);


ALTER TABLE public."Blog" OWNER TO postgres;

--
-- Name: Message; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."Message" (
    id integer NOT NULL,
    "sentAt" timestamp(3) without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "senderId" integer NOT NULL,
    "receiverId" integer NOT NULL,
    content text NOT NULL,
    read boolean DEFAULT false NOT NULL
);


ALTER TABLE public."Message" OWNER TO postgres;

--
-- Name: Message_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."Message_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."Message_id_seq" OWNER TO postgres;

--
-- Name: Message_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."Message_id_seq" OWNED BY public."Message".id;


--
-- Name: User; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public."User" (
    id integer NOT NULL,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    role text DEFAULT 'guest'::text NOT NULL
);


ALTER TABLE public."User" OWNER TO postgres;

--
-- Name: User_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public."User_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public."User_id_seq" OWNER TO postgres;

--
-- Name: User_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public."User_id_seq" OWNED BY public."User".id;


--
-- Name: Message id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Message" ALTER COLUMN id SET DEFAULT nextval('public."Message_id_seq"'::regclass);


--
-- Name: User id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User" ALTER COLUMN id SET DEFAULT nextval('public."User_id_seq"'::regclass);


--
-- Data for Name: Blog; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Blog" (id, "authorId", title, content, "createdAt", summary, "updatedAt", thumbnail) FROM stdin;
a_blog	1	A new Blog	---\nid: 'a_blog'\ntitle: 'A new Blog'\nauthor: 'Bryn Ghiffar'\ndate: '01/01/2023'\nsummary: 'An awesome blog'\nthumbnail: '/polygon.jpg'\n---\n\nAn ampty blog.	2023-05-15 04:45:15.073	An awesome blog	2023-05-15 04:45:15.073	/thumbnails/everest.jpg
awesome_postx	1	Bloggerzzz	---\nid: awesome_postx\ntitle: 'Bloggerzzz'\nauthor: 'Bryn Ghiffar'\ndate: '01/01/2023'\nsummary: 'An awesome blog'\nthumbnail: '/polygon.jpg'\n---\n# Python\nI **love** markdown. \n\nHere is some python code\n```python\ndef main():\n    print("Hello World")\n    \n\nif __name__ == '__main__':\n    main()\n```\n# JavaScript\n**Here** is some *javascript* code.\n> *damn*\n```js\nfunction love() {\n    console.log("I love javascript and you should tooo ≡ƒÆû")}\n```\nEdit some more\n\n\n| Feature | Markdown | Html |\n| -- | --- | --- |\n| Cakes | Γ£à | Γ¥î |\n	2023-05-18 14:37:51.179	An awesome blog	2023-05-18 14:37:51.179	/polygon.jpg
python_beginner	1	A beginner's guide to 'hello world'	---\nid: 'python_beginner'\ntitle: "A beginner's guide to 'hello world'"\nauthor: 'Bryn Ghiffar'\nsummary: 'An awesome blog'\nthumbnail: '/polygon.jpg'\n---\n\n```python\n\ndef main():\n    print("Hello World")\n\nif __name__ == '__main__':\n    main()\n```	2023-05-21 08:43:08.745	An awesome blog	2023-05-21 08:43:08.745	/polygon.jpg
\.


--
-- Data for Name: Message; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."Message" (id, "sentAt", "senderId", "receiverId", content, read) FROM stdin;
157	2023-06-25 18:42:39.998	3	2	three two one	t
150	2023-06-25 14:34:00.301	3	2	Hi jack.	t
144	2023-06-18 21:24:27.366	3	2	Hiyaaa	t
142	2023-06-18 21:22:30.313	3	2	yooo.	t
143	2023-06-18 21:22:50.271	3	2	test123	t
141	2023-06-18 21:19:04.262	3	2	It kinda doesn't work?	t
90	2023-06-07 14:33:59.343	3	2	It looks like the server is working it's thing.	t
94	2023-06-07 14:45:24.758	3	2	Hi, person.	t
96	2023-06-07 14:46:25.835	3	2	Ooooh coool.	t
97	2023-06-17 10:56:25.392	3	2	Hello Jack	t
113	2023-06-18 00:43:50.422	1	2	Hi Jack.	t
115	2023-06-18 00:44:28.661	1	2	How's it going?	t
99	2023-06-17 11:05:51.722	3	2	Hello Jack	t
116	2023-06-18 20:48:06.561	2	1	Hi dude.	f
117	2023-06-18 20:48:46.538	2	1	bbbb	f
118	2023-06-18 20:49:39.815	2	1	three	f
119	2023-06-18 20:50:07.914	2	1	asdfasdf	f
120	2023-06-18 20:51:03.671	2	1	Hello World	f
121	2023-06-18 20:51:25.59	2	1	Hello Again.	f
122	2023-06-18 20:52:19.347	2	1	test 123	f
124	2023-06-18 20:54:14.881	2	1	asdfas	f
126	2023-06-18 20:57:02.252	2	1	hello again?	f
128	2023-06-18 20:57:51.469	2	1	Is that true?	f
102	2023-06-17 18:16:08.531	3	2	Hello Jack	t
104	2023-06-17 18:51:44.337	3	2	Ping!!!!	t
107	2023-06-17 19:00:37.149	3	2	Looks like the auto scroll is working.	t
110	2023-06-17 19:02:00.767	3	2	Yess...	t
163	2023-06-25 18:50:03.456	2	3	ewfasdfasdf	t
162	2023-06-25 18:48:07.861	2	3	grgsf	t
139	2023-06-18 21:17:27.387	3	1	Hi jack.	f
158	2023-06-25 18:43:37.241	2	3	One two three.	t
159	2023-06-25 18:43:57.955	2	3	hello2	t
156	2023-06-25 18:42:09.863	2	3	It iz the update.	t
152	2023-06-25 14:35:59.468	2	3	change	t
149	2023-06-25 14:33:06.418	2	3	Hiyaa cat.	t
145	2023-06-18 21:26:38.019	2	3	It's pretty cool, I like it.	t
137	2023-06-18 21:16:41.31	2	3	hi cat.	t
138	2023-06-18 21:17:12.348	2	3	hi cat...	t
123	2023-06-18 20:52:41.266	2	3	does it work?	t
125	2023-06-18 20:55:43.399	2	3	testt	t
127	2023-06-18 20:57:09.632	2	3	cattooo	t
131	2023-06-18 21:02:18.514	2	3	Ola cat.	t
114	2023-06-18 00:44:09.46	2	1	Hi Bryn.	f
176	2023-06-25 22:03:05.62	3	2	rgafasdf	t
88	2023-06-07 14:33:38.457	3	2	Hi jack.	t
129	2023-06-18 21:00:04.921	3	2	Yes Jack?	t
130	2023-06-18 21:02:12.075	3	2	Ola jackk!	t
132	2023-06-18 21:02:37.193	3	2	asdasdf	t
133	2023-06-18 21:02:37.853	3	2	asdf	t
134	2023-06-18 21:02:38.594	3	2	asdf	t
135	2023-06-18 21:02:39.194	3	2	asdf	t
136	2023-06-18 21:14:10.418	3	2	hello jack.	t
140	2023-06-18 21:18:03.565	3	2	hi jack..	t
146	2023-06-18 21:27:03.517	3	2	This is another test message.	t
147	2023-06-18 21:31:07.364	3	2	This is a pretty cool, chat application no?	t
153	2023-06-25 14:36:53.466	3	2	what an update.	t
155	2023-06-25 14:37:24.143	3	2	What an updatee it iz.	t
165	2023-06-25 18:53:53.788	3	2	Hi cat.	t
175	2023-06-25 22:02:59.561	3	2	How are you doing?	t
91	2023-06-07 14:34:08.343	2	3	Yeah, pretty awesome.	t
93	2023-06-07 14:35:32.233	2	3	Agreed.	t
95	2023-06-07 14:46:18.595	2	3	Now, you cannot send an empty message.	t
98	2023-06-17 11:02:48.596	2	3	Hello Cat	t
100	2023-06-17 18:07:55.97	2	3	Hello Cat	t
101	2023-06-17 18:15:56.323	2	3	Hello Cat	t
103	2023-06-17 18:51:20.602	2	3	test	t
105	2023-06-17 18:51:51.641	2	3	Ponggg!!!!	t
106	2023-06-17 19:00:01.313	2	3	Hello	t
169	2023-06-25 22:01:54.223	2	3	test	t
108	2023-06-17 19:00:53.87	2	3	Appears so.	t
111	2023-06-17 19:58:35.382	2	3	Hello world	t
112	2023-06-17 19:58:44.781	2	3	it works	t
89	2023-06-07 14:33:45.544	2	3	Well, hi cat.	t
148	2023-06-18 21:31:18.424	2	3	Yea, I guess so : ))	t
151	2023-06-25 14:34:49.831	2	3	fsdfg	t
154	2023-06-25 14:37:14.405	2	3	Yes, I know.	t
160	2023-06-25 18:45:21.59	2	3	asdfasvh	t
161	2023-06-25 18:46:00.788	2	3	hgfhgn	t
164	2023-06-25 18:53:18.966	2	3	test2	t
166	2023-06-25 22:00:37.991	2	3	test	t
167	2023-06-25 22:00:47.967	2	3	test again.	t
168	2023-06-25 22:01:14.585	2	3	hello	t
170	2023-06-25 22:02:19.342	2	3	asdf	t
171	2023-06-25 22:02:25.782	2	3	This is it.	t
172	2023-06-25 22:02:44.801	2	3	I love cakess.	t
173	2023-06-25 22:02:50.54	2	3	Yes, it's pretty fund.	t
174	2023-06-25 22:02:52.58	2	3	asdkfhsdf	t
\.


--
-- Data for Name: User; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public."User" (id, username, email, password, role) FROM stdin;
1	Bryn Ghiffar	bryn.ghiffar@gmail.com	$2b$10$6BOnJu9k0155CMirJG4sc.vPWlO7VMvzEWxFhclSctKjqxHigt2OK	guest
2	jack@mail.com	jack@mail.com	$2b$10$ygaBk/XPO5/nsVdeFD0IXOUT3IqZk5RX7ncZalHGWj6wq.MUknvZ6	guest
3	aCat	cat@mail.com	$2b$10$MkLTKI63FiVX/ZZjA0PcH.A/Ex44lcx2x8J4U0hs8BBWSJ13fsfwO	guest
\.


--
-- Name: Message_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."Message_id_seq"', 176, true);


--
-- Name: User_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public."User_id_seq"', 3, true);


--
-- Name: Blog Blog_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Blog"
    ADD CONSTRAINT "Blog_pkey" PRIMARY KEY (id);


--
-- Name: Message Message_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Message"
    ADD CONSTRAINT "Message_pkey" PRIMARY KEY (id);


--
-- Name: User User_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."User"
    ADD CONSTRAINT "User_pkey" PRIMARY KEY (id);


--
-- Name: User_email_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "User_email_key" ON public."User" USING btree (email);


--
-- Name: User_username_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE UNIQUE INDEX "User_username_key" ON public."User" USING btree (username);


--
-- Name: Blog Blog_authorId_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Blog"
    ADD CONSTRAINT "Blog_authorId_fkey" FOREIGN KEY ("authorId") REFERENCES public."User"(id) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- Name: Message Message_receiverId_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Message"
    ADD CONSTRAINT "Message_receiverId_fkey" FOREIGN KEY ("receiverId") REFERENCES public."User"(id) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- Name: Message Message_senderId_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public."Message"
    ADD CONSTRAINT "Message_senderId_fkey" FOREIGN KEY ("senderId") REFERENCES public."User"(id) ON UPDATE CASCADE ON DELETE RESTRICT;


--
-- PostgreSQL database dump complete
--

