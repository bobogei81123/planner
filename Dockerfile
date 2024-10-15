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
RUN apt-get update && apt install -y openssl curl
COPY --from=builder-backend /app/target/release/planner-backend /usr/local/bin/backend
COPY --from=builder-frontend /app/frontend/dist ./assets
# Install supercronic. Latest release at https://github.com/aptible/supercronic/releases
ENV SUPERCRONIC_URL=https://github.com/aptible/supercronic/releases/download/v0.2.32/supercronic-linux-amd64 \
    SUPERCRONIC=supercronic-linux-amd64 \
    SUPERCRONIC_SHA1SUM=7da26ce6ab48d75e97f7204554afe7c80779d4e0
RUN curl -fsSLO "$SUPERCRONIC_URL" \
 && echo "${SUPERCRONIC_SHA1SUM}  ${SUPERCRONIC}" | sha1sum -c - \
 && chmod +x "$SUPERCRONIC" \
 && mv "$SUPERCRONIC" "/usr/local/bin/${SUPERCRONIC}" \
 && ln -s "/usr/local/bin/${SUPERCRONIC}" /usr/local/bin/supercronic
ENTRYPOINT ["/usr/local/bin/backend"]
