FROM rust:1.82 AS builder
WORKDIR /app
COPY ./ .
RUN cargo build --release
RUN pwd

FROM --platform=$TARGETPLATFORM alpine:3.20.3 AS alpine_base
ARG TARGETARCH
RUN apk update
# RUN apk add libssl1.1
RUN apk add libssl3

FROM alpine_base AS crawler
RUN apk update
# RUN apk add libssl1.1
RUN apk add libssl3
WORKDIR /app
COPY --from=builder /app/target/release/browsenscrape /app/crawler
COPY --from=builder /app/conf/config-prod.toml /app/config.toml
# RUN chmod +x /app/crawler
# RUN ls -al /
# RUN ls -al /app
# RUN ls -al /app/crawler
CMD /app/crawler /app/config.toml

# FROM alpine_base AS web
# RUN apk add libgcc
# RUN apk add gcompat
#
# COPY --from=builder ./target/release/web_server ./target/release/web_server
# RUN chmod +x ./target/release/web_server
# CMD /target/release/web_server 8080
