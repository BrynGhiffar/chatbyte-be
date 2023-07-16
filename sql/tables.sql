DROP TABLE IF EXISTS "User";
-- CREATE SEQUENCE user_id_seq;

CREATE TABLE "User" (
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid()
);

-- ALTER SEQUENCE user_id_seq OWNED BY "User"."id";

INSERT INTO "User" ("id") values (default);

SELECT count("User"."id") from "User";