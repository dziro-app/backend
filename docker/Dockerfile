FROM rust:1.61.0-slim AS build
WORKDIR /app
COPY . /app/
RUN pwd && ls -lah ./
RUN apt-get update
RUN apt-get install pkg-config libssl-dev protobuf-compiler -y
RUN cargo build --release



FROM debian:11.3-slim
WORKDIR /dziro
RUN apt-get update && apt-get install curl -y
COPY --from=build /app/target/release/api /dziro
RUN chmod +x /dziro/api

CMD ["/dziro/api"]