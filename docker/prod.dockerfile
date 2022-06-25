FROM debian:11.3-slim

WORKDIR /usr/share

COPY ./target/release/api /usr/share/dziro/api

CMD ["dziro/api"]