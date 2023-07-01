FROM rust:1.70-slim-buster as builder

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

RUN rustup toolchain install nightly; \
    rustup override set nightly

WORKDIR /app

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