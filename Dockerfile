FROM rust:1.69.0 as builder

RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup toolchain install stable-aarch64-unknown-linux-gnu

WORKDIR /app

ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

COPY Cargo.toml Cargo.lock /app/
COPY src /app/src
COPY .cargo /app/.cargo

RUN cargo build --target aarch64-unknown-linux-gnu --release --verbose --verbose

FROM debian:bullseye-slim AS runtime 

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/sofar-mqtt /usr/local/bin

EXPOSE 8080

CMD /usr/local/bin/sofar-mqtt