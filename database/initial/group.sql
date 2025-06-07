CREATE TABLE public.group (
    id INTEGER PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    disbanded BOOLEAN DEFAULT FALSE NOT NULL
);

CREATE TABLE public.group_avatar (
    id INTEGER PRIMARY KEY,
    group_id INTEGER UNIQUE NOT NULL,
    group_image BYTEA NOT NULL,
    CONSTRAINT fk_group_avatar_group_id FOREIGN KEY (group_id) REFERENCES public.group(id)
);

CREATE TABLE public.group_member (
    group_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    CONSTRAINT group_member_group_id FOREIGN KEY (group_id) REFERENCES public.group(id),
    CONSTRAINT group_member_user_id FOREIGN KEY (user_id) REFERENCES public.user(id),
    UNIQUE(group_id, user_id)
);

CREATE TABLE public.group_message(
    id INTEGER PRIMARY KEY,
    sent_at TIMESTAMP(3) WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    sender_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    edited BOOLEAN DEFAULT FALSE NOT NULL,
    deleted BOOLEAN DEFAULT FALSE NOT NULL,
    CONSTRAINT group_message_sender_id FOREIGN KEY(sender_id) REFERENCES public.user(id),
    CONSTRAINT group_message_group_id FOREIGN KEY(group_id) REFERENCES public.group(id),
    UNIQUE(id, group_id)
);

CREATE TABLE public.group_message_read(
    group_id INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    reader_id INTEGER NOT NULL,
    CONSTRAINT fk_group_message_read_member FOREIGN KEY (group_id, reader_id) REFERENCES public.group_member(group_id, user_id),
    CONSTRAINT fk_group_message_read_group_message FOREIGN KEY (message_id, group_id) REFERENCES public.group_message(id, group_id),
    UNIQUE(message_id, reader_id)
);

-- create view user_id, group_id, unread_count
-- create view last message in a group
-- make these views when we have more data to work with.

CREATE VIEW public.last_message_group AS
SELECT 
    G.ID GROUP_ID,
    G.NAME GROUP_NAME,
    U.USERNAME USERNAME,
    M.SENT_AT SENT_AT,
    M.CONTENT CONTENT,
    M.DELETED DELETED
FROM PUBLIC.GROUP_MESSAGE M
    FULL JOIN PUBLIC.GROUP G ON G.ID = M.GROUP_ID
    FULL JOIN PUBLIC.USER U ON U.ID = M.SENDER_ID
WHERE G.ID IS NOT NULL
AND 
    CASE
        WHEN M.SENT_AT IS NOT NULL THEN (M.SENT_AT IN (SELECT MAX(sent_at) as id FROM PUBLIC.GROUP_MESSAGE GROUP BY PUBLIC.GROUP_MESSAGE.GROUP_ID)) ELSE TRUE
    END
;

CREATE VIEW public.username_group_message AS
SELECT 
    GM.ID as ID,
    GM.GROUP_ID,
    GM.sender_id, 
    U.username, 
    GM.sent_at,
    GM.deleted,
    GM.content
FROM PUBLIC.GROUP_MESSAGE GM
    JOIN PUBLIC.USER U ON GM.SENDER_ID = U.ID
WHERE GM.GROUP_ID = 4 AND DELETED = FALSE
ORDER BY GM.SENT_AT
;

CREATE SEQUENCE public.group_id_seq AS INTEGER
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE SEQUENCE public.group_avatar_id_seq AS INTEGER
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE SEQUENCE public.group_message_id_seq AS INTEGER
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE ONLY public.group
    ALTER COLUMN id
    SET DEFAULT nextval('public.group_id_seq'::regclass);

ALTER TABLE ONLY public.group_avatar 
    ALTER COLUMN id 
    SET DEFAULT nextval('public.group_avatar_id_seq'::regclass);

ALTER TABLE ONLY public.group_message
    ALTER COLUMN id
    SET DEFAULT nextval('public.group_message_id_seq'::regclass);
