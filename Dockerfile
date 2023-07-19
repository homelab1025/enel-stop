FROM rust:latest AS builder
COPY ./ .
RUN cargo build --release

FROM --platform=$TARGETPLATFORM debian:bookworm-slim
ARG TARGETARCH
RUN apt-get update
RUN apt-get install wget --yes
RUN wget -O libssl1.1.deb http://security.debian.org/debian-security/pool/updates/main/o/openssl/libssl1.1_1.1.1n-0+deb11u5_${TARGETARCH}.deb
RUN apt-get install apt-utils
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN dpkg --install libssl1.1.deb

COPY --from=builder ./target/release/crawler ./target/release/crawler
COPY --from=builder conf/config-prod.toml ./target/release/config.toml
CMD /target/release/crawler /target/release/config.toml