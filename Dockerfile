FROM rust:latest AS builder
COPY ./ .
RUN cargo build --release

FROM --platform=$TARGETPLATFORM alpine:latest AS crawler
ARG TARGETARCH
RUN apk update
RUN apk add libssl1.1

COPY --from=builder ./target/release/crawler ./target/release/crawler
COPY --from=builder conf/config-prod.toml ./target/release/config.toml
CMD /target/release/crawler /target/release/config.toml

FROM --platform=$TARGETPLATFORM alpine:latest AS web
ARG TARGETARCH
RUN apk update
RUN apk add libssl1.1

COPY --from=builder ./target/release/server ./target/release/server
CMD /target/release/server 8080