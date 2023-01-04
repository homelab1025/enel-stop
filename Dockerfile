FROM rust:latest AS builder
COPY ./ .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update
RUN apt-get install openssl
COPY --from=builder ./target/release/enel-stop ./target/release/enel-stop
COPY --from=builder ./config.toml ./target/release/config.toml
CMD ["/target/release/enel-stop config.toml"]