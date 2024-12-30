FROM rust:1.82 AS builder
WORKDIR /
COPY ./ .
RUN cargo build --release

FROM --platform=$TARGETPLATFORM alpine:3.20.3 AS alpine_base
ARG TARGETARCH
RUN apk update
# RUN apk add libssl1.1
RUN apk add libssl3

FROM alpine_base AS crawler
RUN apk update
# RUN apk add libssl1.1
RUN apk add libssl3

# WORKDIR /
COPY --from=builder ./target/release/browsenscrape ./target/release/browsenscrape
COPY --from=builder conf/config-prod.toml ./target/release/config.toml
CMD /target/release/browsenscrape /target/release/config.toml

FROM alpine_base AS web
RUN apk add libgcc
RUN apk add gcompat

COPY --from=builder ./target/release/web_server ./target/release/web_server
RUN chmod +x ./target/release/web_server
CMD /target/release/web_server 8080
