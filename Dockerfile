FROM rust:1.84 AS builder
WORKDIR /app
COPY ./ .
RUN cargo build --release

# FROM --platform=$TARGETPLATFORM alpine:3.20.3 AS alpine_base
# ARG TARGETARCH
# RUN apk update
# # RUN apk add libssl1.1
# RUN apk add libssl3

# FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12 as debian_base
FROM --platform=$TARGETPLATFORM ghcr.io/linuxcontainers/debian-slim as debian_base
ARG TARGETARCH

FROM debian_base AS crawler
RUN apt-get update
RUN apt-get -y install chromium-driver
WORKDIR /app
COPY --from=builder /app/target/release/browsenscrape /app/crawler
COPY --from=builder /app/conf/config-prod.toml /app/config.toml
ENTRYPOINT [ "/app/crawler", "/app/config.toml" ]

FROM debian_base AS web
WORKDIR /app
COPY --from=builder /app/target/release/web_server /app/web_server
COPY --from=builder /app/conf/config-prod.toml /app/config.toml
RUN chmod +x /app/web_server
ENTRYPOINT [ "/app/web_server", "/app/config.toml" ]
# CMD /target/release/web_server 8080

FROM node:lts-alpine as webapp-build-stage
WORKDIR /app
COPY viewer/package*.json ./
RUN npm install
COPY viewer/. .
RUN npm run build

# production stage
FROM nginx:stable-alpine as webapp
COPY --from=webapp-build-stage /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]