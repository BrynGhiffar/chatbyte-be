\ir initial/reset.sql
\ir initial/user.sql
\ir initial/message.sql
\ir initial/user_avatar.sql
\ir initial/session.sql
\ir initial/group.sql
\ir initial/views.sql


-- Mock Users
INSERT INTO public.user (username, email, password)
    VALUES ('Bryn Ghiffar', 'bryn.ghiffar@gmail.com', crypt('bryn.ghiffar@gmail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('Jackmann', 'jack@mail.com', crypt('jack@mail.com', gen_salt('bf', 5)));
INSERT INTO public.user (username, email, password)
    VALUES ('Gato', 'cat@mail.com', crypt('cat@mail.com', gen_salt('bf', 5)));