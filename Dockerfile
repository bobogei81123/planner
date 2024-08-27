FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS rust-prepare-backend
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY testlib ./testlib
RUN cargo chef prepare --recipe-path recipe-backend.json

FROM chef AS builder-backend
COPY --from=rust-prepare-backend /app/recipe-backend.json recipe-backend.json
RUN cargo chef cook --release --recipe-path recipe-backend.json
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY testlib ./testlib
COPY .sqlx ./.sqlx
RUN cargo build --release

FROM node:20-alpine AS builder-frontend
WORKDIR /app
RUN npm install -g pnpm
COPY frontend-react/package*.json frontend/
RUN pnpm --prefix=./frontend install
COPY frontend-react ./frontend
RUN pnpm --prefix=./frontend run build

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update && apt install -y openssl
COPY --from=builder-backend /app/target/release/planner-backend /usr/local/bin/backend
COPY --from=builder-frontend /app/frontend/dist ./assets
ENTRYPOINT ["/usr/local/bin/backend"]
