# Arguments
ARG APP=sofar-mqtt
ARG TARGET=aarch64-unknown-linux-gnu

FROM rust:1.69.0 as builder

# Setup envs
ARG APP
ARG TARGET
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

# Install OS dependencies
RUN apt update && apt upgrade -y
RUN apt install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross

# Setup rust toolchain
RUN rustup target add $TARGET
RUN rustup toolchain install stable-$TARGET

# Set working directory
WORKDIR /usr/src

# Create dummy project
RUN cargo new $APP

COPY Cargo.toml Cargo.lock /usr/src/$APP
COPY .cargo /usr/src/$APP/.cargo

# Set working directory
WORKDIR /usr/src/$APP

# Build project to cache dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --target $TARGET

# Copy project files
COPY src /usr/src/$APP/src

# Touch main.rs to prevent cached release build
RUN touch /usr/src/$APP/src/main.rs

# Build real project
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/$APP/target \
    cargo build --release --target $TARGET

FROM debian:bullseye-slim AS runtime
ARG APP
ARG TARGET
ENV APP ${APP}

# Copy application binary from builder image
COPY --from=builder /usr/src/$APP/target/$TARGET/release/$APP /usr/local/bin

# Expose app port
EXPOSE 8080

# Run the application
CMD /usr/local/bin/${APP}