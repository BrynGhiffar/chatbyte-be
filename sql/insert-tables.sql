\i sql/initial/reset.sql
\i sql/initial/user.sql
\i sql/initial/message.sql
\i sql/initial/user_avatar.sql
\i sql/initial/session.sql
\i sql/initial/group.sql
\i sql/initial/views.sql


-- Mock Users
INSERT INTO public.user (username, email, password)
    VALUES ('Bryn Ghiffar', 'bryn.ghiffar@gmail.com', crypt('bryn.ghiffar@gmail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('Jackmann', 'jack@mail.com', crypt('jack@mail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('Gato', 'cat@mail.com', crypt('cat@mail.com', gen_salt('bf', 5)));