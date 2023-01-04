FROM rust:latest AS builder
COPY ./ .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/enel-stop ./target/release/enel-stop
CMD ["/target/release/enel-stop"]