FROM rust:1.61.0-slim AS build
WORKDIR /app
RUN ls -lah
COPY ./ /app/
RUN apt-get update
RUN apt-get install pkg-config libssl-dev -y
RUN cargo build --release



FROM debian:11.3-slim
WORKDIR /dziro
COPY --from=build /app/target/release/api /dziro
RUN chmod +x /dziro/api

CMD ["/dziro/api"]