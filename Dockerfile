FROM rust:latest AS builder
COPY ./ .
RUN cargo build --release

FROM --platform=amd64 debian:bookworm-slim AS prod-crawler-amd64
RUN apt-get update
RUN apt-get install wget --yes
RUN wget -O libssl1.1.deb http://security.debian.org/debian-security/pool/updates/main/o/openssl/libssl1.1_1.1.1n-0+deb11u5_amd64.deb
RUN apt-get install apt-utils
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN dpkg --install libssl1.1.deb

COPY --from=builder ./target/release/crawler ./target/release/crawler
COPY --from=builder conf/config-prod.toml ./target/release/config.toml
CMD /target/release/crawler /target/release/config.toml

FROM --platform=arm64 debian:bookworm-slim AS prod-crawler-arm64
RUN apt-get update
RUN apt-get install wget --yes
RUN wget -O libssl1.1.deb http://security.debian.org/debian-security/pool/updates/main/o/openssl/libssl1.1_1.1.1n-0+deb11u5_arm64.deb
RUN apt-get install apt-utils
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN dpkg --install libssl1.1.deb

COPY --from=builder ./target/release/crawler ./target/release/crawler
COPY --from=builder conf/config-prod.toml ./target/release/config.toml
CMD /target/release/crawler /target/release/config.toml