FROM debian:11.3

WORKDIR /usr/share

RUN apt-get update && apt-get install curl -y

COPY ./target/release/api /usr/share/dziro/api

CMD ["dziro/api"]