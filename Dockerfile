FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS rust-prepare-backend
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
RUN cargo chef prepare --recipe-path recipe-backend.json

FROM chef AS builder-backend
COPY --from=rust-prepare-backend /app/recipe-backend.json recipe-backend.json
RUN cargo chef cook --release --recipe-path recipe-backend.json
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY .sqlx ./.sqlx
RUN cargo build --bin fly-io --release

FROM node:20-alpine AS builder-frontend
WORKDIR /app
RUN npm install -g pnpm
COPY frontend/package*.json frontend/
RUN pnpm --prefix=./frontend install
COPY frontend ./frontend
RUN pnpm --prefix=./frontend run build

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt install -y openssl
COPY --from=builder-backend /app/target/release/fly-io /usr/local/bin/backend
COPY --from=builder-frontend /app/assets ./assets
ENTRYPOINT ["/usr/local/bin/backend"]
