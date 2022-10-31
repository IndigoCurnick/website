FROM rust:1.62-slim-buster as chef

COPY . .

RUN apt-get update
RUN apt-get install openssl -y

RUN apt-get install -y --no-install-recommends \
    pkg-config \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \ 
    libssl-dev \
    cmake

RUN rustup toolchain install nightly-2022-05-11; \
    rustup override set nightly-2022-05-11

WORKDIR /app

RUN cargo install cargo-chef

# ============= Chef =============
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ============= Rust compiler image =============
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN RUST_BACKTRACE=1 cargo build --release

# ============= Final image =============
FROM debian:buster-slim AS runtime

RUN apt-get update
WORKDIR /app
COPY . . 
COPY --from=builder /app/target/release/website /usr/local/bin

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/website"]