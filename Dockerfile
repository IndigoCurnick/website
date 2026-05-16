FROM rust:1.95.0-slim-bookworm AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-20260505-slim AS runtime

RUN apt-get update 
RUN apt-get install -y --no-install-recommends ca-certificates libc6 

WORKDIR /app
COPY . .
COPY --from=builder /app/target/release/website /usr/local/bin

EXPOSE 8080

CMD ["/usr/local/bin/website"]