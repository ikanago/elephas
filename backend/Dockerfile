FROM rust:1.67.0 AS development

FROM lukemathwalker/cargo-chef:latest-rust-1.67.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build

FROM debian:bullseye AS production
COPY --from=builder /app/target/debug/backend /usr/bin/backend
