-- Views
DROP VIEW IF EXISTS public.unread_message_content;
DROP VIEW IF EXISTS public.unread_message_count;
DROP VIEW IF EXISTS public.last_message;
DROP VIEW IF EXISTS public.message_sender;
DROP VIEW IF EXISTS public.last_message_group;
DROP VIEW IF EXISTS public.username_group_message;

-- Tables
DROP TABLE IF EXISTS public.group_message_read;
DROP TABLE IF EXISTS public.group_message;
DROP TABLE IF EXISTS public.group_member;
DROP TABLE IF EXISTS public.group_avatar;
DROP TABLE IF EXISTS public.group;
DROP TABLE IF EXISTS public.user_avatar;
DROP TABLE IF EXISTS public.message;
DROP TABLE IF EXISTS public.session;
DROP TABLE IF EXISTS public.user;

-- Sequences
DROP SEQUENCE IF EXISTS public.user_id_seq;
DROP SEQUENCE IF EXISTS public.message_id_seq;
DROP SEQUENCE IF EXISTS public.session_id_seq;
DROP SEQUENCE IF EXISTS public.group_id_seq;
DROP SEQUENCE IF EXISTS public.group_avatar_id_seq;
DROP SEQUENCE IF EXISTS public.group_message_id_seq;


CREATE EXTENSION IF NOT EXISTS pgcrypto;