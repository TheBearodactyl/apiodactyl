FROM rust:bookworm AS builder

RUN apt update && apt-get update
RUN apt upgrade -y && apt-get upgrade -y
RUN apt install -y \
    build-essential \
    libpq-dev \
    curl \
    neovim \
    fish \
    pkg-config
RUN apt-get install -y postgresql-client

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build -j8

FROM debian:bookworm-slim

RUN apt update && apt-get update
RUN apt upgrade -y && apt-get upgrade -y
RUN apt install -y \
    libpq5 \
    bash \
    curl \
    postgresql \
    fish \
    xz-utils \
    neovim \
    ca-certificates
RUN apt-get install -y postgresql-client

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
RUN cp /root/.cargo/bin/diesel /usr/bin/
RUN chmod +x /usr/bin/diesel

WORKDIR /app

COPY --from=builder /app/target/debug/apiodactyl .

COPY migrations ./migrations
COPY docker-entrypoint.sh .

RUN chmod +x docker-entrypoint.sh

ENTRYPOINT ["./docker-entrypoint.sh"]

