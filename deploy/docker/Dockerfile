# syntax=docker/dockerfile:1

FROM node:20-alpine AS admin_builder
WORKDIR /app

COPY apps/admin_web/package.json apps/admin_web/package-lock.json ./
RUN npm ci

COPY apps/admin_web/ ./
RUN npm run build

FROM rust:latest AS backend_builder
WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=backend_builder /app/target/release/api_gateway /app/api_gateway
COPY --from=admin_builder /app/dist /app/admin_web

ENV RUST_LOG=info
ENV ADMIN_WEB_DIR=/app/admin_web
EXPOSE 8080

CMD ["/app/api_gateway"]
