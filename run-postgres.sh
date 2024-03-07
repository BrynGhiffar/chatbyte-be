#!/usr/bin/bash

docker container stop chat-app-db
docker container rm chat-app-db
docker run \
    --name chat-app-db \
    -e POSTGRES_PASSWORD=password \
    -e PGDATA=/var/lib/postgresql/data/pgdata \
    -v chat-app-db:/var/lib/postgresql/data \
    -v ./database:/docker-entrypoint-initdb.d \
    -d postgres

# Exec into postgres
# docker exec -it chat-app-db psql -U postgres