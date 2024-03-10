FROM rust:1.76.0-slim-buster


WORKDIR /usr/src/chatbyte-be

COPY . .

RUN apt update
RUN apt install libssl-dev pkg-config -y

RUN cargo install --path .

CMD [ "chatbyte-be" ]