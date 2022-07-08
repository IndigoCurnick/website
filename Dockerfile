FROM rust:1.61.0-slim-buster as builder

WORKDIR /app
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



# RUN rustup install nightly-2022-05-11; \

RUN rustup toolchain install nightly-2022-05-11; \
    rustup override set nightly-2022-05-11

RUN rustc --version

RUN RUST_BACKTRACE=1 cargo build --release



FROM debian:buster-slim AS runtime

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


WORKDIR /app
COPY . . 


COPY --from=builder /app/target/release/website /usr/local/bin

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/website"]