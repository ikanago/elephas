FROM lukemathwalker/cargo-chef:0.1.61-rust-slim-bullseye AS chef
WORKDIR /app
RUN apt update && apt install -y libssl-dev pkg-config

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN SQLX_OFFLINE=true cargo build --release --bin app

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/base-debian11 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]
