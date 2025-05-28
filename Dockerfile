FROM --platform=$TARGETPLATFORM ghcr.io/linuxcontainers/debian-slim as debian_base
ARG TARGETARCH

FROM debian_base AS crawler
RUN apt-get update
RUN apt-get -y install chromium-driver
COPY target/release/browsenscrape /app/crawler
COPY conf/config-prod.toml /app/config.toml
ENTRYPOINT [ "/app/crawler", "/app/config.toml" ]

FROM debian_base AS web
COPY target/release/web_server /app/web_server
COPY conf/config-prod.toml /app/config.toml
RUN chmod +x /app/web_server
ENTRYPOINT [ "/app/web_server", "/app/config.toml" ]
# CMD /target/release/web_server 8080

FROM node:lts-alpine as webapp-build-stage
WORKDIR /app
COPY webapp/package*.json ./
RUN npm install
COPY webapp/. .
RUN npm run build

# production stage
FROM nginx:stable-alpine as webapp
COPY --from=webapp-build-stage /app/dist /usr/share/nginx/html
COPY --from=webapp-build-stage /app/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]